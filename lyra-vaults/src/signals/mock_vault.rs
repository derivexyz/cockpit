//! Deployment wiring for the mock signal-driven covered call vault.
//!
//! The mock vault has no TSA: its subaccount is owned by the session key's owner wallet and
//! every action is signed by the session key itself via the regular API flow.
//!
//! [StrategyRunner] expects its caller to own market synchronization and readiness. Decisions
//! are hourly, so rather than holding subscriptions open between them, each decision refreshes
//! the market on demand: positions over REST, then a ticker subscription held only long enough
//! for every eligible call to go live. The auction the decision selects opens its own
//! subscriptions for the one instrument it trades, and lives as long as it needs to.

use crate::helpers::{get_expiry_options, subscribe_tickers, sync_subaccount, TickerInterval};
use crate::market::{new_market_state, MarketState};
use crate::signals::strategies::mock::{mock_signals, MockCCParams, MOCK_SIGNAL_POLL_INTERVAL};
use crate::signals::StrategyRunner;
use anyhow::{bail, Error, Result};
use log::{error, info, warn};
use serde::Deserialize;
use std::time::Duration;
use tokio::select;

/// Backoff before a failed decision loop is restarted.
const RETRY_SEC: u64 = 15;
/// How long a decision waits for the tickers of every eligible call to go live.
const TICKER_WAIT_SEC: u64 = 60;
/// Poll interval of that wait.
const TICKER_WAIT_POLL_MS: u64 = 100;

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
}

impl MockCCExecutor {
    /// Expects the env (session key, owner, `VAULT_NAME` and `SUBACCOUNT_ID`) to already be set up.
    pub async fn new(params: MockCCVaultParams) -> Result<Self> {
        let subaccount_id = params.subaccount_id;
        if subaccount_id <= 0 {
            bail!("set a valid subaccount_id in the {} vault params", params.vault_name);
        }
        let market = new_market_state();
        // fail fast on a bad subaccount or bad credentials rather than at the first decision
        sync_subaccount(market.clone(), subaccount_id, vec![]).await?;
        Ok(Self { params, subaccount_id, market })
    }

    /// Runs the mock signal decisions, restarting the runner if it exits with an error.
    ///
    /// The market is refreshed as part of loading the signals, i.e. immediately before the
    /// strategy reads it. Every eligible call must be live before deciding: selection compares
    /// deltas across strikes, so a partially warmed set picks whichever strike ticked first
    /// rather than the one nearest the target delta.
    pub async fn run(&self) -> Result<()> {
        let strategy = self.params.strategy_params.clone();
        let mut runner = StrategyRunner::new(strategy.clone(), self.market.clone())?;
        let interval = self.params.decision_interval();
        info!("MockCC decisions every {} sec", interval.as_secs());
        loop {
            let market = self.market.clone();
            let strategy = strategy.clone();
            let subaccount_id = self.subaccount_id;
            let res = runner
                .run(interval, move |decision_at| {
                    let market = market.clone();
                    let strategy = strategy.clone();
                    async move {
                        refresh_market(&market, subaccount_id, &strategy).await?;
                        mock_signals(decision_at).await
                    }
                })
                .await;
            error!("MockCC decisions exited with {:#?}, restarting in {} sec", res, RETRY_SEC);
            tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
        }
    }
}

/// Brings the market up to date for one decision, then drops the ticker subscription.
///
/// Positions come over REST: between decisions there is nothing to react to, so a standing
/// subaccount subscription would only stream updates nobody reads. Note the auction that a
/// decision selects does subscribe to the subaccount for its own duration.
async fn refresh_market(
    market: &MarketState,
    subaccount_id: i64,
    strategy: &MockCCParams,
) -> Result<()> {
    sync_subaccount(market.clone(), subaccount_id, vec![]).await?;

    let currency = &strategy.option_currency;
    let options =
        get_expiry_options(currency, strategy.expiry_sec()?, strategy.min_expiry_sec()?, true)
            .await?;
    let names = options.iter().map(|o| o.instrument_name.clone()).collect::<Vec<_>>();
    info!("MockCC subscribing to {} eligible {} calls", names.len(), currency);

    // the fastest interval: the subscription is held only until the tickers land, and the
    // sooner they all do, the smaller the chance an early one goes stale before the decision
    let sub = subscribe_tickers(market.clone(), options, TickerInterval::_100Ms);
    let ready = wait_for_tickers(market, &names);
    select! {
        res = sub => Err(Error::msg(format!("ticker subscription exited early with {:#?}", res))),
        _ = ready => Ok(()),
    }
    // the subscription is dropped here, closing its connection until the next decision
}

/// Waits until every eligible call has a live ticker. Times out with a warning rather than an
/// error: an instrument whose feed never arrives should not block the vault, and the selector
/// ignores stale tickers anyway.
///
/// Freshness (rather than mere presence) is what makes an on-demand subscription safe: it holds
/// until all of the tickers are live at the same moment, so none of them can have gone stale by
/// the time the strategy compares them.
async fn wait_for_tickers(market: &MarketState, names: &Vec<String>) {
    let started = tokio::time::Instant::now();
    let timeout = Duration::from_secs(TICKER_WAIT_SEC);
    loop {
        let missing = {
            let reader = market.read().await;
            names.iter().filter(|name| reader.get_ticker(name).is_none()).count()
        };
        if !names.is_empty() && missing == 0 {
            info!("MockCC tickers live for all {} eligible calls", names.len());
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
