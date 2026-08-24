use crate::market::MarketState;
use crate::signals::SignalStrategy;
use anyhow::{bail, Context, Result};
use std::future::Future;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct CycleReport {
    /// UTC timestamp in seconds.
    pub decision_at: i64,
    pub signals: Vec<bool>,
}

#[derive(Clone, Debug)]
pub enum TickResult {
    Processed(CycleReport),
    AlreadyProcessed(i64),
}

/// Coordinates serialized decisions for one strategy instance.
///
/// Execution is serialized. Each tick asks the strategy to select and build a
/// stage, then runs that stage to completion with its normal reconnect policy.
/// The selected stage owns its auction setup, cleanup, sizing, and stopping
/// behavior; the runner treats it as an opaque executable unit.
///
/// The caller owns market synchronization, subscriptions, readiness, and
/// reconnection. Until a proper signal interface is introduced, [`Self::run`]
/// accepts a callback that loads the signal values for each decision slot.
pub struct StrategyRunner<S> {
    strategy: S,
    market: MarketState,
    last_decision_at: i64,
}

impl<S> StrategyRunner<S>
where
    S: SignalStrategy,
{
    pub fn new(strategy: S, market: MarketState) -> Result<Self> {
        if strategy.name().trim().is_empty() {
            bail!("strategy name cannot be empty");
        }
        Ok(Self { strategy, market, last_decision_at: 0 })
    }

    /// UTC timestamp in seconds, or `0` before the first completed decision.
    pub fn last_decision_at(&self) -> i64 {
        self.last_decision_at
    }

    pub fn strategy(&self) -> &S {
        &self.strategy
    }

    pub fn market(&self) -> &MarketState {
        &self.market
    }

    /// Continuously evaluates the latest completed UTC decision slot.
    ///
    /// The first unprocessed slot is evaluated immediately. After a stage
    /// completes, the runner waits until a newer slot exists before loading
    /// signals and selecting another stage. If a stage spans multiple slots,
    /// stale intermediate slots are skipped and only the latest slot is used.
    pub async fn run<GetSignals, GetSignalsFuture>(
        &mut self,
        decision_interval: Duration,
        mut get_signals: GetSignals,
    ) -> Result<()>
    where
        GetSignals: FnMut(i64) -> GetSignalsFuture + Send,
        GetSignalsFuture: Future<Output = Result<Vec<bool>>> + Send,
    {
        interval_seconds(decision_interval)?;

        loop {
            let decision_at = self.wait_for_next_decision(decision_interval).await?;
            let signals = get_signals(decision_at).await?;
            self.tick_once(decision_at, signals).await?;
        }
    }

    /// Runs one completed decision bar against an already coherent, maintained
    /// market state and a non-empty signal vector. It performs no scheduling,
    /// signal loading, or market synchronization.
    pub async fn tick_once(&mut self, decision_at: i64, signals: Vec<bool>) -> Result<TickResult> {
        if signals.is_empty() {
            bail!("signal vector cannot be empty");
        }
        if self.last_decision_at >= decision_at {
            return Ok(TickResult::AlreadyProcessed(decision_at));
        }

        let mut stage = self.strategy.get_action(&self.market, signals.clone()).await?;
        stage.run_with_reconnect().await?;
        self.last_decision_at = decision_at;
        Ok(TickResult::Processed(CycleReport { decision_at, signals }))
    }

    async fn wait_for_next_decision(&self, decision_interval: Duration) -> Result<i64> {
        loop {
            let now = utc_now_seconds()?;
            let decision_at = decision_slot(now, decision_interval)?;
            if self.last_decision_at < decision_at {
                return Ok(decision_at);
            }

            tokio::time::sleep(duration_until_next_slot(now, decision_interval)?).await;
        }
    }
}

/// Floors a UTC timestamp in seconds to a decision boundary.
pub fn decision_slot(now: i64, interval: Duration) -> Result<i64> {
    let interval_seconds = interval_seconds(interval)?;
    Ok(now.div_euclid(interval_seconds) * interval_seconds)
}

fn interval_seconds(interval: Duration) -> Result<i64> {
    if interval.subsec_nanos() != 0 {
        bail!("decision interval must be a whole number of seconds");
    }

    let interval_seconds = i64::try_from(interval.as_secs())
        .context("decision interval is too large to represent in seconds")?;
    if interval_seconds == 0 {
        bail!("decision interval must be at least one second");
    }
    Ok(interval_seconds)
}

fn duration_until_next_slot(now: i64, interval: Duration) -> Result<Duration> {
    let interval_seconds = interval_seconds(interval)?;
    let elapsed_in_slot = now.rem_euclid(interval_seconds);
    let wait_seconds = u64::try_from(interval_seconds - elapsed_in_slot)
        .context("decision interval cannot be represented as a sleep duration")?;
    Ok(Duration::from_secs(wait_seconds))
}

fn utc_now_seconds() -> Result<i64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?;
    i64::try_from(now.as_secs()).context("UTC timestamp is too large to represent in seconds")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::new_market_state;
    use crate::shared::stages::ExecutorStage;
    use std::collections::VecDeque;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    };

    #[derive(Debug)]
    struct SignalOnStage {
        runs: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl ExecutorStage for SignalOnStage {
        async fn run(&self) -> Result<()> {
            panic!("runner must call run_with_reconnect")
        }

        async fn reconnect(&mut self) -> Result<()> {
            panic!("runner must call run_with_reconnect")
        }

        async fn run_with_reconnect(&mut self) -> Result<()> {
            self.runs.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[derive(Debug)]
    struct SignalOffStage {
        runs: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl ExecutorStage for SignalOffStage {
        async fn run(&self) -> Result<()> {
            panic!("runner must call run_with_reconnect")
        }

        async fn reconnect(&mut self) -> Result<()> {
            panic!("runner must call run_with_reconnect")
        }

        async fn run_with_reconnect(&mut self) -> Result<()> {
            self.runs.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    struct SwitchingStrategy {
        selections: Arc<AtomicUsize>,
        signal_on_runs: Arc<AtomicUsize>,
        signal_off_runs: Arc<AtomicUsize>,
    }

    #[async_trait::async_trait]
    impl SignalStrategy for SwitchingStrategy {
        fn name(&self) -> &str {
            "switching-strategy"
        }

        async fn get_action(
            &self,
            _market: &MarketState,
            signals: Vec<bool>,
        ) -> Result<Box<dyn ExecutorStage>> {
            self.selections.fetch_add(1, Ordering::SeqCst);
            let stage: Box<dyn ExecutorStage> = if signals.iter().all(|signal| *signal) {
                Box::new(SignalOnStage { runs: self.signal_on_runs.clone() })
            } else {
                Box::new(SignalOffStage { runs: self.signal_off_runs.clone() })
            };
            Ok(stage)
        }
    }

    #[test]
    fn decision_slots_align_to_utc_intervals() {
        let now = 12 * 60 * 60 + 37 * 60 + 42;
        let slot = decision_slot(now, Duration::from_secs(60 * 60)).unwrap();
        assert_eq!(slot, 12 * 60 * 60);
    }

    #[tokio::test]
    async fn strategy_can_select_different_stage_types() {
        let selections = Arc::new(AtomicUsize::new(0));
        let signal_on_runs = Arc::new(AtomicUsize::new(0));
        let signal_off_runs = Arc::new(AtomicUsize::new(0));
        let strategy = SwitchingStrategy {
            selections: selections.clone(),
            signal_on_runs: signal_on_runs.clone(),
            signal_off_runs: signal_off_runs.clone(),
        };
        let mut runner = StrategyRunner::new(strategy, new_market_state()).unwrap();
        assert_eq!(runner.last_decision_at(), 0);

        let signal_on_at = 1_755_518_400;
        let signal_off_at = signal_on_at + 60 * 60;

        let signal_on = runner.tick_once(signal_on_at, vec![true, true]).await.unwrap();
        assert!(matches!(signal_on, TickResult::Processed(_)));
        let signal_off = runner.tick_once(signal_off_at, vec![true, false]).await.unwrap();
        assert!(matches!(signal_off, TickResult::Processed(_)));

        assert_eq!(selections.load(Ordering::SeqCst), 2);
        assert_eq!(signal_on_runs.load(Ordering::SeqCst), 1);
        assert_eq!(signal_off_runs.load(Ordering::SeqCst), 1);

        let duplicate = runner.tick_once(signal_off_at, vec![false, false]).await.unwrap();
        assert!(matches!(duplicate, TickResult::AlreadyProcessed(at) if at == signal_off_at));
        assert_eq!(selections.load(Ordering::SeqCst), 2);
        assert_eq!(signal_on_runs.load(Ordering::SeqCst), 1);
        assert_eq!(signal_off_runs.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn run_loads_the_next_signal_only_after_the_current_stage_finishes() {
        let selections = Arc::new(AtomicUsize::new(0));
        let signal_on_runs = Arc::new(AtomicUsize::new(0));
        let signal_off_runs = Arc::new(AtomicUsize::new(0));
        let source_calls = Arc::new(AtomicUsize::new(0));
        let requested_slots = Arc::new(Mutex::new(Vec::new()));
        let signal_batches = Arc::new(Mutex::new(VecDeque::from([
            Ok(vec![true, true]),
            Err(anyhow::anyhow!("signal source stopped")),
            Ok(vec![false, false]),
        ])));

        let strategy = SwitchingStrategy {
            selections: selections.clone(),
            signal_on_runs: signal_on_runs.clone(),
            signal_off_runs: signal_off_runs.clone(),
        };
        let mut runner = StrategyRunner::new(strategy, new_market_state()).unwrap();

        let run_result = tokio::time::timeout(
            Duration::from_secs(2),
            runner.run(Duration::from_secs(1), {
                let signal_on_runs = signal_on_runs.clone();
                let signal_off_runs = signal_off_runs.clone();
                let source_calls = source_calls.clone();
                let requested_slots = requested_slots.clone();
                let signal_batches = signal_batches.clone();

                move |decision_at| {
                    let call = source_calls.fetch_add(1, Ordering::SeqCst);
                    let completed_stages = signal_on_runs.load(Ordering::SeqCst)
                        + signal_off_runs.load(Ordering::SeqCst);
                    assert_eq!(completed_stages, call);
                    requested_slots.lock().unwrap().push(decision_at);

                    std::future::ready(signal_batches.lock().unwrap().pop_front().unwrap())
                }
            }),
        )
        .await
        .expect("runner did not reach the source error")
        .unwrap_err();

        assert_eq!(run_result.to_string(), "signal source stopped");
        assert_eq!(source_calls.load(Ordering::SeqCst), 2);
        assert_eq!(selections.load(Ordering::SeqCst), 1);
        assert_eq!(signal_on_runs.load(Ordering::SeqCst), 1);
        assert_eq!(signal_off_runs.load(Ordering::SeqCst), 0);
        assert_eq!(signal_batches.lock().unwrap().len(), 1);

        let requested_slots = requested_slots.lock().unwrap();
        assert!(requested_slots.windows(2).all(|slots| slots[0] < slots[1]));
        assert_eq!(runner.last_decision_at(), requested_slots[0]);
    }
}
