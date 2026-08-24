//! Deployment wiring for the mock signal-driven covered call vault.
//!
//! The mock vault has no TSA: its subaccount is owned by the session key's owner wallet and
//! every action is signed by the session key itself via the regular API flow.
//!
//! [StrategyRunner] expects its caller to own market synchronization and readiness, so this
//! module maintains the market state the mock strategy decides on (subaccount positions plus
//! the tickers of the currently eligible calls) while the runner drives the decisions.

use crate::helpers::{
    get_expiry_options, subscribe_subaccount, subscribe_tickers, sync_subaccount, TickerInterval,
};
use crate::market::{new_market_state, MarketState};
use crate::signals::strategies::mock::{mock_signals, MockCCParams, MOCK_SIGNAL_POLL_INTERVAL};
use crate::signals::StrategyRunner;
use anyhow::{bail, Result};
use log::{error, info, warn};
use serde::Deserialize;
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::RwLock;

/// How often the eligible calls are re-fetched and re-subscribed to, so expiry rolls get picked up.
const TICKER_REFRESH_SEC: u64 = 900;
/// Backoff before a failed subscription or decision loop is restarted.
const RETRY_SEC: u64 = 15;
/// How long a decision waits for the tickers of every subscribed call to arrive.
const TICKER_WAIT_SEC: u64 = 60;
/// Poll interval of that wait.
const TICKER_WAIT_POLL_MS: u64 = 250;

/// The calls currently subscribed to, i.e. the ones a decision can select from.
type Candidates = Arc<RwLock<Vec<String>>>;

#[derive(Debug, Clone, Deserialize)]
pub struct MockCCVaultParams {
    pub env: String,        // Environment name (e.g. staging, prod)
    pub vault_name: String, // used for logging and as the default key name
    /// The subaccount the vault trades on. Passed as a param since a vault without a TSA has no
    /// contract to read it from.
    pub subaccount_id: i64,
    /// Name of the session key and its owner, i.e. `/session_keys/{env}/{key_name}` and
    /// `/owners/{env}/{key_name}` in the AWS parameter store. Defaults to the lowercased
    /// vault name. Ignored if `SESSION_PRIVATE_KEY` / `OWNER_PUBLIC_KEY` are set in the env.
    pub key_name: Option<String>,
    /// Decision cadence in seconds, defaults to [MOCK_SIGNAL_POLL_INTERVAL].
    pub decision_interval_sec: Option<u64>,

    pub strategy_params: MockCCParams,
}

impl MockCCVaultParams {
    pub fn key_name(&self) -> String {
        self.key_name.clone().unwrap_or(self.vault_name.to_lowercase())
    }

    pub fn decision_interval(&self) -> Duration {
        self.decision_interval_sec.map(Duration::from_secs).unwrap_or(MOCK_SIGNAL_POLL_INTERVAL)
    }
}

pub struct MockCCExecutor {
    params: MockCCVaultParams,
    subaccount_id: i64,
    market: MarketState,
    candidates: Candidates,
}

impl MockCCExecutor {
    /// Expects the env (session key, owner, `VAULT_NAME` and `SUBACCOUNT_ID`) to already be set up.
    pub async fn new(params: MockCCVaultParams) -> Result<Self> {
        let subaccount_id = params.subaccount_id;
        if subaccount_id <= 0 {
            bail!("set a valid subaccount_id in the {} vault params", params.vault_name);
        }
        let market = new_market_state();
        sync_subaccount(market.clone(), subaccount_id, vec![]).await?;
        let candidates = Arc::new(RwLock::new(vec![]));
        Ok(Self { params, subaccount_id, market, candidates })
    }

    pub async fn run(&self) -> Result<()> {
        let market_task = self.run_market();
        let decision_task = self.run_decisions();
        select! {
            res = market_task => res,
            res = decision_task => res,
        }
    }

    /// Keeps the market state coherent for the decision loop. Both subscriptions are restarted
    /// on failure, so this only returns on a fatal (e.g. misconfigured params) error.
    async fn run_market(&self) -> Result<()> {
        let subacc_sub = self.subscribe_subaccount_forever();
        let ticker_sub = self.subscribe_tickers_forever();
        select! {
            res = subacc_sub => res,
            res = ticker_sub => res,
        }
    }

    async fn subscribe_subaccount_forever(&self) -> Result<()> {
        loop {
            let res = subscribe_subaccount(self.market.clone(), self.subaccount_id).await;
            warn!("MockCC subaccount subscription exited with {:#?}, resubscribing", res);
            tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
            // positions may have moved (e.g. settlement) while disconnected
            if let Err(e) = sync_subaccount(self.market.clone(), self.subaccount_id, vec![]).await {
                warn!("MockCC subaccount re-sync failed with {:#?}", e);
            }
        }
    }

    async fn subscribe_tickers_forever(&self) -> Result<()> {
        let strategy = &self.params.strategy_params;
        let expiry_sec = strategy.expiry_sec()?;
        let min_expiry_sec = strategy.min_expiry_sec()?;
        loop {
            let currency = &strategy.option_currency;
            let options = get_expiry_options(currency, expiry_sec, min_expiry_sec, true).await;
            match options {
                Ok(options) => {
                    info!("MockCC subscribing to {} eligible {} calls", options.len(), currency);
                    let names =
                        options.iter().map(|o| o.instrument_name.clone()).collect::<Vec<_>>();
                    *self.candidates.write().await = names;
                    let sub =
                        subscribe_tickers(self.market.clone(), options, TickerInterval::_1000Ms);
                    let refresh = tokio::time::sleep(Duration::from_secs(TICKER_REFRESH_SEC));
                    select! {
                        res = sub => {
                            warn!("MockCC ticker subscription exited with {:#?}", res);
                            tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
                        },
                        _ = refresh => {},
                    }
                }
                Err(e) => {
                    warn!("MockCC failed to fetch the eligible calls with {:#?}", e);
                    tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
                }
            }
        }
    }

    /// Runs the mock signal decisions, restarting the runner if it exits with an error.
    ///
    /// Every decision is gated on the tickers of all subscribed calls having arrived: selection
    /// compares deltas across strikes, so deciding on a partially warmed set picks whichever
    /// strike happened to tick first rather than the one nearest the target delta.
    async fn run_decisions(&self) -> Result<()> {
        let strategy = self.params.strategy_params.clone();
        let mut runner = StrategyRunner::new(strategy, self.market.clone())?;
        let interval = self.params.decision_interval();
        info!("MockCC decisions every {} sec", interval.as_secs());
        loop {
            let market = self.market.clone();
            let candidates = self.candidates.clone();
            let res = runner
                .run(interval, move |decision_at| {
                    let market = market.clone();
                    let candidates = candidates.clone();
                    async move {
                        wait_for_tickers(&market, &candidates).await;
                        mock_signals(decision_at).await
                    }
                })
                .await;
            error!("MockCC decisions exited with {:#?}, restarting in {} sec", res, RETRY_SEC);
            tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
        }
    }
}

/// Waits until every subscribed call has been received at least once. Times out with a warning
/// rather than an error: an instrument whose feed never arrives should not block the vault, and
/// the selector ignores stale tickers anyway.
async fn wait_for_tickers(market: &MarketState, candidates: &Candidates) {
    let started = tokio::time::Instant::now();
    let timeout = Duration::from_secs(TICKER_WAIT_SEC);
    loop {
        let names = candidates.read().await.clone();
        let missing = {
            let reader = market.read().await;
            let tickers = reader.get_tickers();
            names.iter().filter(|name| !tickers.contains_key(*name)).count()
        };
        if !names.is_empty() && missing == 0 {
            info!("MockCC tickers ready for all {} subscribed calls", names.len());
            return;
        }
        if started.elapsed() >= timeout {
            warn!(
                "MockCC deciding without {} of {} call tickers after waiting {} sec",
                missing,
                names.len(),
                TICKER_WAIT_SEC
            );
            return;
        }
        tokio::time::sleep(Duration::from_millis(TICKER_WAIT_POLL_MS)).await;
    }
}
