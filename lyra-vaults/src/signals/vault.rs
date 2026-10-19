//! Runtime for a signal-driven vault, shared by every signal strategy.
//!
//! Decisions are hourly, so rather than holding subscriptions open between them, each decision
//! refreshes the market on demand: positions (and, when a leg is live, its trade history) over
//! REST, then a ticker subscription held only long enough for every candidate instrument to go
//! live. The auction a decision selects opens its own subscriptions for the one instrument it
//! trades, and lives as long as it needs to.
//!
//! These vaults have no TSA: the subaccount is owned by the session key's owner wallet and every
//! action is signed by the session key itself via the regular API flow.

use crate::helpers::{get_window_options, subscribe_tickers, sync_subaccount, TickerInterval};
use crate::market::{new_market_state, MarketState};
use crate::shared::stages::ExecutorStage;
use crate::signals::{SignalStrategy, StrategyRunner};
use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use bigdecimal::Zero;
use log::{error, info, warn};
use std::time::Duration;
use tokio::select;

/// Backoff before a failed decision loop is restarted.
const RETRY_SEC: u64 = 15;
/// How long a decision waits for the tickers of every candidate instrument to go live.
const TICKER_WAIT_SEC: u64 = 60;
/// Poll interval of that wait.
const TICKER_WAIT_POLL_MS: u64 = 100;

/// A decision that does nothing: hold whatever the vault currently has.
#[derive(Debug)]
pub struct NoopStage;

#[async_trait]
impl ExecutorStage for NoopStage {
    async fn run(&self) -> Result<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> Result<()> {
        Ok(())
    }
}

pub struct SignalVaultExecutor<V: SignalStrategy> {
    vault: V,
    subaccount_id: i64,
    decision_interval: Duration,
    market: MarketState,
}

impl<V: SignalStrategy> SignalVaultExecutor<V> {
    /// Expects the env (session key, owner, `VAULT_NAME` and `SUBACCOUNT_ID`) to already be set up.
    pub async fn new(
        vault: V,
        subaccount_id: i64,
        decision_interval: Duration,
    ) -> Result<Self> {
        if subaccount_id <= 0 {
            bail!("set a valid subaccount_id in the vault params");
        }
        let market = new_market_state();
        // fail fast on a bad subaccount or bad credentials rather than at the first decision
        sync_subaccount(market.clone(), subaccount_id, vec![]).await?;
        Ok(Self { vault, subaccount_id, decision_interval, market })
    }

    /// Runs the decisions, restarting the runner if it exits with an error.
    ///
    /// The market is refreshed as part of loading the signals, i.e. immediately before the
    /// strategy reads it, and every candidate instrument must be live before deciding: selection
    /// compares deltas across strikes, so a partially warmed set picks whichever strike ticked
    /// first rather than the one nearest the target.
    pub async fn run(&self) -> Result<()> {
        let mut runner = StrategyRunner::new(self.vault.clone(), self.market.clone())?;
        info!(
            "{} decisions every {} sec",
            self.vault.name(),
            self.decision_interval.as_secs()
        );
        loop {
            let market = self.market.clone();
            let vault = self.vault.clone();
            let subaccount_id = self.subaccount_id;
            let res = runner
                .run(self.decision_interval, move |decision_at| {
                    let market = market.clone();
                    let vault = vault.clone();
                    async move {
                        refresh_market(&market, subaccount_id, &vault).await?;
                        vault.signals(decision_at).await
                    }
                })
                .await;
            error!(
                "{} decisions exited with {:#?}, restarting in {} sec",
                self.vault.name(),
                res,
                RETRY_SEC
            );
            tokio::time::sleep(Duration::from_secs(RETRY_SEC)).await;
        }
    }
}

/// Brings the market up to date for one decision, then drops the ticker subscription.
///
/// Positions come over REST: between decisions there is nothing to react to, so a standing
/// subaccount subscription would only stream updates nobody reads. Note the auction that a
/// decision selects does subscribe to the subaccount for its own duration.
pub async fn refresh_market<V: SignalStrategy>(
    market: &MarketState,
    subaccount_id: i64,
    vault: &V,
) -> Result<()> {
    // positions first, then the trade history of anything held, which is how a strategy recovers
    // when a leg was opened (its open time gates any minimum-hold rule)
    sync_subaccount(market.clone(), subaccount_id, vec![]).await?;
    let held = held_option_names(market).await;
    if !held.is_empty() {
        info!("{} holds {:?}, syncing its trades", vault.name(), held);
        sync_subaccount(market.clone(), subaccount_id, held).await?;
    }

    let candidates = vault.candidates()?;
    let options = get_window_options(
        &candidates.currency,
        candidates.min_expiry_sec,
        candidates.max_expiry_sec,
        &candidates.option_types,
    )
    .await?;
    let names = options.iter().map(|o| o.instrument_name.clone()).collect::<Vec<_>>();
    info!(
        "{} subscribing to {} candidate {} instruments",
        vault.name(),
        names.len(),
        candidates.currency
    );

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

/// The option instruments the subaccount currently holds a non-zero position in.
async fn held_option_names(market: &MarketState) -> Vec<String> {
    let reader = market.read().await;
    reader
        .iter_positions()
        .filter(|position| {
            !position.amount.is_zero()
                && (position.instrument_name.ends_with("-C")
                    || position.instrument_name.ends_with("-P"))
        })
        .map(|position| position.instrument_name.clone())
        .collect()
}

/// Waits until every candidate has a live ticker. Times out with a warning rather than an error:
/// an instrument whose feed never arrives should not block the vault, and selection ignores stale
/// tickers anyway.
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
            info!("tickers live for all {} candidates", names.len());
            return;
        }
        if started.elapsed() >= timeout {
            warn!(
                "deciding without {} of {} candidate tickers after waiting {} sec",
                missing,
                names.len(),
                TICKER_WAIT_SEC
            );
            return;
        }
        tokio::time::sleep(Duration::from_millis(TICKER_WAIT_POLL_MS)).await;
    }
}
