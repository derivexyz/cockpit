use crate::market::MarketState;
use crate::shared::stages::ExecutorStage;
use anyhow::Result;

/// Stateless decision policy for a signal-driven vault.
///
/// A strategy derives its next executable stage from signals and current
/// exchange state. Implementations should reconstruct critical state from the
/// market rather than relying on in-memory progress across calls.
#[async_trait::async_trait]
pub trait SignalStrategy: Send + Sync {
    fn name(&self) -> &str;

    /// `signals` is expected to be non-empty. A strategy considers its signal
    /// on only when every value is `true`.
    ///
    /// The shared market state lets implementations take short-lived locks as
    /// needed while constructing stages whose own constructors may perform I/O.
    /// Each call may return a different concrete stage type.
    async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>>;
}

#[async_trait::async_trait]
impl<T> SignalStrategy for Box<T>
where
    T: SignalStrategy + ?Sized,
{
    fn name(&self) -> &str {
        (**self).name()
    }

    async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>> {
        (**self).get_action(market, signals).await
    }
}
