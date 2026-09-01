//! Signal inputs read from the internal-analytics ClickHouse.
//!
//! This module is deliberately independent of the vault machinery: nothing here touches market
//! state, executors or the vault env vars, so the functions are callable (and testable) on their
//! own. Only [ClickhouseClient::from_env] reads the environment, and only for its own credentials.

pub mod client;
pub mod gate;
pub mod grid;
pub mod iv_rank;
pub mod momentum;
#[cfg(test)]
pub mod test_support;

pub use client::ClickhouseClient;
pub use gate::{
    fetch_gate, fetch_momentum_gate, GateParams, GateReading, MomentumGateParams,
    MomentumReading,
};
pub use grid::{floor_to_hour, forward_filled_grid, grid_gaps, BAR_SEC};
pub use iv_rank::{
    constant_maturity_iv_pct, constant_maturity_tau, fetch_hourly_atm_iv, fetch_hourly_slices,
    fetch_iv_rank, interpolate_total_variance, iv_rank_series, percentile_rank, HourlyIv, IvRank,
    SviSlice, IV_RANK_LOOKBACK_BARS, IV_RANK_MIN_BARS,
};
pub use momentum::{
    fetch_hourly_spot, fetch_momentum, fetch_momentum_with_window, momentum_pct, momentum_series,
    HourlySpot, Momentum, MOMENTUM_WINDOW_BARS,
};
