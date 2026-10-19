use crate::market::MarketState;
use crate::shared::stages::ExecutorStage;
use crate::signals::Selector;
use anyhow::Result;

/// A signal-driven vault strategy: what to read, and what to do about it.
///
/// What it *quotes* comes from its [Selector], which owns both the candidate universe and the
/// selection within it.
///
/// Implementations should reconstruct critical state from the market rather than relying on
/// in-memory progress across calls, so that a restart resumes cleanly.
///
/// `Clone + 'static` because the runtime clones the strategy into each decision's task. That
/// makes the trait non-object-safe, which costs nothing while every vault is one static type —
/// running several strategies behind `dyn SignalStrategy` would mean moving these bounds back out
/// to the runtime.
#[async_trait::async_trait]
pub trait SignalStrategy: Selector + Clone + Send + Sync + 'static {
    fn name(&self) -> &str;

    /// The signal vector for a completed decision hour, non-empty. An `Err` means the vault holds
    /// and retries rather than acting, which is the conservative reading of a signal outage.
    async fn signals(&self, decision_at: i64) -> Result<Vec<bool>>;

    /// Derives the next executable stage from the signals and the current exchange state. A
    /// strategy considers its signal on only when every value is `true`.
    ///
    /// The shared market state lets implementations take short-lived locks as needed while
    /// constructing stages whose own constructors may perform I/O. Each call may return a
    /// different concrete stage type.
    async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>>;
}
