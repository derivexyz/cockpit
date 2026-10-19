//! Strategy-neutral runtime for signal-driven vaults.
//!
//! For now, callers supply a non-empty `Vec<bool>` and an externally maintained
//! [`crate::market::MarketState`]. [`SignalStrategy`] uses those inputs to
//! dynamically select an [`crate::shared::stages::ExecutorStage`], and
//! [`StrategyRunner`] runs the selected stage with its normal reconnect behavior.
//! Signal loading and validation will be introduced when their requirements
//! are known.
//!
//! Concrete strategy policy (including any Swather or Plume `get_action`
//! implementation) does not belong in this module.

pub mod mock_vault;
pub mod vault;
mod runner;
mod selector;
pub mod strategies;
mod strategy;

pub use mock_vault::*;
pub use vault::*;
pub use runner::*;
pub use selector::*;
pub use strategy::*;
