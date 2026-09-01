//! The two-part regime gate both books share (spec §2.4):
//!
//! ```text
//! sig_iv(t)   = iv_rank(t) >= FLOOR          40 for Swather, 30 for Plume
//! sig_mom(t)  = mom(t) <= +5.0 %
//! gate_raw(t) = sig_iv(t) AND sig_mom(t)
//! ```
//!
//! plus the per-book anti-flicker layer applied on top: Swather holds the gate True for 72 hours
//! after its most recent True ("minhold", §3), Plume requires 6 consecutive True bars before it
//! will enter ("debounce", §4).
//!
//! # Missing data
//!
//! Spec §2.5 is "lookup failure == gate off, never trade through a signal gap". This module reads
//! that as: an input that is *legitimately absent* (an hour whose listed expiries do not bracket
//! 30 days, an hour with no spot print) contributes `false`, so the gate reads OFF. A *failure to
//! reach ClickHouse*, or a lookback too empty to rank against, is an `Err` instead — the caller
//! then holds its position rather than acting, because an exit driven by a dead data feed is worse
//! than a late exit. That is a deliberate departure from the literal reading; see the vault docs.

use crate::clickhouse::client::ClickhouseClient;
use crate::clickhouse::grid::{floor_to_hour, forward_filled_grid, BAR_SEC};
use crate::clickhouse::iv_rank::{
    as_grid as iv_as_grid, fetch_hourly_atm_iv, iv_rank_series, IV_RANK_LOOKBACK_BARS,
    IV_RANK_MIN_BARS,
};
use crate::clickhouse::momentum::{
    as_grid as spot_as_grid, fetch_hourly_spot, momentum_pct, MOMENTUM_WINDOW_BARS, SEED_BARS,
};
use anyhow::{bail, Result};
use log::{debug, info};
use serde::Deserialize;
use std::collections::HashMap;

/// Swather's IV-rank floor (spec §8).
pub const SWATHER_IV_RANK_FLOOR: f64 = 40.0;
/// Plume's IV-rank floor.
pub const PLUME_IV_RANK_FLOOR: f64 = 30.0;
/// The shared momentum ceiling, in percent.
pub const MOM_MAX_PCT: f64 = 5.0;
/// Swather's signal-level minimum-on, in hours.
pub const SWATHER_MINHOLD_HOURS: usize = 72;
/// Plume's entry persistence, in hours.
pub const PLUME_DEBOUNCE_HOURS: usize = 6;

#[derive(Debug, Clone, Deserialize)]
pub struct GateParams {
    pub option_currency: String,
    /// `iv_rank >= floor`, i.e. 40 for Swather and 30 for Plume.
    pub iv_rank_floor: f64,
    /// `mom <= max`, in percent.
    pub mom_max_pct: f64,
    /// Hours the raw gate is held True after its most recent True. 1 disables the minhold.
    pub minhold_hours: usize,
}

impl GateParams {
    /// Swather's gate: i40m5 with a 72 hour minimum-on.
    pub fn swather(option_currency: &str) -> Self {
        Self {
            option_currency: option_currency.to_string(),
            iv_rank_floor: SWATHER_IV_RANK_FLOOR,
            mom_max_pct: MOM_MAX_PCT,
            minhold_hours: SWATHER_MINHOLD_HOURS,
        }
    }
}

/// One hour of the raw gate and the inputs behind it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GateHour {
    pub hour: i64,
    pub iv_rank: Option<f64>,
    pub mom_pct: Option<f64>,
    pub raw: bool,
}

/// The gate as of one decision hour, with the minhold window that produced it.
#[derive(Debug, Clone)]
pub struct GateReading {
    pub hour: i64,
    /// The floor this reading was taken against, for logging.
    pub iv_rank_floor: f64,
    pub iv_rank: Option<f64>,
    pub mom_pct: Option<f64>,
    pub sig_iv: bool,
    pub sig_mom: bool,
    /// This hour's gate, before the minhold.
    pub raw: bool,
    /// What the engine acts on: raw, held True for `minhold_hours` after any True.
    pub effective: bool,
    /// The ceiling this reading was taken against, for logging.
    pub mom_max_pct: f64,
    /// The minhold window, oldest first, current hour last.
    pub history: Vec<GateHour>,
}

impl GateReading {
    /// A one-line summary for the vault log.
    pub fn summary(&self) -> String {
        let fmt = |value: Option<f64>| match value {
            Some(value) => format!("{value:.2}"),
            None => "n/a".to_string(),
        };
        format!(
            "iv_rank {} (>= {}: {}), mom {}% (<= {}%: {}), raw {}, effective {} over {}h",
            fmt(self.iv_rank),
            self.iv_rank_floor,
            self.sig_iv,
            fmt(self.mom_pct),
            self.mom_max_pct,
            self.sig_mom,
            self.raw,
            self.effective,
            self.history.len()
        )
    }
}

/// `gate_raw` for one hour. A missing input is OFF, never ON (spec §2.5).
pub fn raw_gate(iv_rank: Option<f64>, mom_pct: Option<f64>, floor: f64, max_pct: f64) -> bool {
    match (iv_rank, mom_pct) {
        (Some(rank), Some(mom)) => rank >= floor && mom <= max_pct,
        _ => false,
    }
}

/// Swather's minimum-on: True if the raw gate was True anywhere in the window (spec §3).
pub fn apply_minhold(raw: &[bool]) -> bool {
    raw.iter().any(|on| *on)
}

/// Plume's entry persistence: True only if every bar of the window is True (spec §4).
pub fn apply_debounce(raw: &[bool], hours: usize) -> bool {
    raw.len() >= hours && raw[raw.len() - hours..].iter().all(|on| *on)
}

/// Reads the gate as of `at_ts` from ClickHouse.
///
/// Two queries: the spot grid behind the momentum window, and the vol surfaces behind the IV-rank
/// lookback, each extended by the minhold so every hour of that window can be evaluated.
pub async fn fetch_gate(
    client: &ClickhouseClient,
    params: &GateParams,
    at_ts: i64,
) -> Result<GateReading> {
    let hour = floor_to_hour(at_ts);
    let minhold_bars = params.minhold_hours.max(1) as i64;
    let history_from = hour - (minhold_bars - 1) * BAR_SEC;
    let currency = &params.option_currency;

    // momentum over the minhold window: 720 bars of lead-in per hour evaluated
    let spot_from = history_from - (MOMENTUM_WINDOW_BARS as i64 + SEED_BARS) * BAR_SEC;
    let spot_rows = fetch_hourly_spot(client, currency, spot_from, hour).await?;
    if spot_rows.is_empty() {
        bail!("no {} spot observations between {} and {}", currency, spot_from, hour);
    }
    let mom_grid_from = history_from - MOMENTUM_WINDOW_BARS as i64 * BAR_SEC;
    let spot_grid = forward_filled_grid(&spot_as_grid(&spot_rows), mom_grid_from, hour)?;
    let mut mom_by_hour: HashMap<i64, f64> = HashMap::new();
    for bar in MOMENTUM_WINDOW_BARS..spot_grid.len() {
        let at = mom_grid_from + bar as i64 * BAR_SEC;
        let pct = momentum_pct(spot_grid[bar], spot_grid[bar - MOMENTUM_WINDOW_BARS])?;
        mom_by_hour.insert(at, pct);
    }

    // iv rank over the same window: 2160 bars of lead-in per hour evaluated
    let iv_from = history_from - IV_RANK_LOOKBACK_BARS as i64 * BAR_SEC;
    let iv_rows = fetch_hourly_atm_iv(client, currency, iv_from, hour).await?;
    if iv_rows.len() < IV_RANK_MIN_BARS {
        bail!(
            "only {} of the {} hours in the {} IV lookback have a usable surface, need {}",
            iv_rows.len(),
            IV_RANK_LOOKBACK_BARS,
            currency,
            IV_RANK_MIN_BARS
        );
    }
    let iv_grid_from = iv_rows[0].hour.max(iv_from);
    let iv_grid = forward_filled_grid(&iv_as_grid(&iv_rows), iv_grid_from, hour)?;
    let ranks = iv_rank_series(&iv_grid, IV_RANK_LOOKBACK_BARS, IV_RANK_MIN_BARS)?;
    let mut rank_by_hour: HashMap<i64, f64> = HashMap::new();
    for (bar, rank) in ranks.iter().enumerate() {
        if let Some(rank) = rank {
            rank_by_hour.insert(iv_grid_from + bar as i64 * BAR_SEC, *rank);
        }
    }

    let history: Vec<GateHour> = (0..minhold_bars)
        .map(|back| {
            let at = history_from + back * BAR_SEC;
            let iv_rank = rank_by_hour.get(&at).copied();
            let mom_pct = mom_by_hour.get(&at).copied();
            GateHour {
                hour: at,
                iv_rank,
                mom_pct,
                raw: raw_gate(iv_rank, mom_pct, params.iv_rank_floor, params.mom_max_pct),
            }
        })
        .collect();

    let current = *history.last().expect("the minhold window always holds the current hour");
    let raws: Vec<bool> = history.iter().map(|entry| entry.raw).collect();
    let reading = GateReading {
        hour,
        iv_rank_floor: params.iv_rank_floor,
        mom_max_pct: params.mom_max_pct,
        iv_rank: current.iv_rank,
        mom_pct: current.mom_pct,
        sig_iv: current.iv_rank.map_or(false, |rank| rank >= params.iv_rank_floor),
        sig_mom: current.mom_pct.map_or(false, |mom| mom <= params.mom_max_pct),
        raw: current.raw,
        effective: apply_minhold(&raws),
        history,
    };
    debug!("gate history: {:?}", reading.history);
    info!("gate at {}: {}", hour, reading.summary());
    Ok(reading)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_gate_needs_both_components() {
        let (floor, max) = (40.0, 5.0);
        assert!(raw_gate(Some(40.0), Some(5.0), floor, max), "both exactly at the threshold");
        assert!(raw_gate(Some(80.0), Some(-20.0), floor, max));
        assert!(!raw_gate(Some(39.99), Some(0.0), floor, max), "iv below the floor");
        assert!(!raw_gate(Some(80.0), Some(5.01), floor, max), "running hot");
        // a missing input is OFF, never ON
        assert!(!raw_gate(None, Some(0.0), floor, max));
        assert!(!raw_gate(Some(80.0), None, floor, max));
        assert!(!raw_gate(None, None, floor, max));
    }

    #[test]
    fn minhold_holds_the_gate_on_after_the_last_true() {
        // a single True anywhere in the window keeps the effective gate on
        assert!(apply_minhold(&[true, false, false]));
        assert!(apply_minhold(&[false, false, true]));
        assert!(!apply_minhold(&[false, false, false]));
        assert!(!apply_minhold(&[]));
        // which is exactly the anti-flicker property: one hour's dip does not flip it off
        let mut window = vec![true; 72];
        window[71] = false;
        assert!(apply_minhold(&window));
        // and 72 consecutive off hours do flip it
        assert!(!apply_minhold(&vec![false; 72]));
    }

    #[test]
    fn debounce_requires_consecutive_true_bars() {
        assert!(apply_debounce(&[true; 6], 6));
        assert!(apply_debounce(&[false, true, true, true, true, true, true], 6));
        assert!(!apply_debounce(&[true, true, true, true, true, false], 6));
        assert!(!apply_debounce(&[true; 5], 6), "not enough history yet");
        assert!(!apply_debounce(&[], 6));
    }
}

#[cfg(test)]
mod live_tests {
    use super::*;
    use crate::clickhouse::test_support::clickhouse_client;

    /// Reads the live Swather gate end to end, which is the pre-deploy smoke test:
    /// `ENV=staging cargo test -p lyra-vaults --lib -- --ignored reads_the_live_gate --nocapture`
    #[tokio::test]
    #[ignore]
    async fn reads_the_live_gate() {
        let client = clickhouse_client();
        let params = GateParams::swather("ETH");
        let now = chrono::Utc::now().timestamp();
        let gate = fetch_gate(&client, &params, now).await.unwrap();

        println!("\n{}", gate.summary());
        let on = gate.history.iter().filter(|entry| entry.raw).count();
        println!(
            "minhold window: {} of {} hours raw-on, oldest {:?}, newest {:?}",
            on,
            gate.history.len(),
            gate.history.first(),
            gate.history.last()
        );

        assert_eq!(gate.history.len(), SWATHER_MINHOLD_HOURS);
        assert_eq!(gate.hour, floor_to_hour(now));
        assert_eq!(gate.history[gate.history.len() - 1].hour, gate.hour);
        // the inputs must actually be present, else the gate is reading OFF for lack of data
        assert!(gate.iv_rank.is_some(), "no iv_rank for the current hour");
        assert!(gate.mom_pct.is_some(), "no momentum for the current hour");
        // and the effective gate is the minhold of the window it reported
        assert_eq!(gate.effective, on > 0);
        assert_eq!(gate.raw, gate.sig_iv && gate.sig_mom);
    }
}

/// A two-sided momentum regime gate: one sleeve for an uptrend, one for a downtrend, and a dead
/// zone in between (Weathervane, NEW_SPEC).
///
/// ```text
/// bull(t) = mom30(t) >  bull_threshold_pct     -> sell puts
/// bear(t) = mom30(t) <  bear_threshold_pct     -> sell calls
/// flat    = neither
/// ```
///
/// Each side must then hold for `persist_hours` consecutive bars before its sleeve may act, which
/// is the spec's anti-churn persistence.
#[derive(Debug, Clone, Deserialize)]
pub struct MomentumGateParams {
    pub option_currency: String,
    /// Sell puts above this trailing return, in percent.
    pub bull_threshold_pct: f64,
    /// Sell calls below this trailing return, in percent. Negative.
    pub bear_threshold_pct: f64,
    /// Consecutive hours a side must hold before its sleeve acts. 1 disables the persistence.
    pub persist_hours: usize,
}

impl MomentumGateParams {
    /// Weathervane's gate: a +/-5% dead zone with a 6 hour persistence.
    pub fn weathervane(option_currency: &str) -> Self {
        Self {
            option_currency: option_currency.to_string(),
            bull_threshold_pct: 5.0,
            bear_threshold_pct: -5.0,
            persist_hours: 6,
        }
    }
}

/// One hour of the two-sided gate.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MomentumHour {
    pub hour: i64,
    pub mom_pct: Option<f64>,
    pub bull: bool,
    pub bear: bool,
}

/// The gate as of one decision hour, with the persistence window behind it.
#[derive(Debug, Clone)]
pub struct MomentumReading {
    pub hour: i64,
    pub mom_pct: Option<f64>,
    /// This hour's raw sides, before the persistence.
    pub bull_raw: bool,
    pub bear_raw: bool,
    /// What each sleeve acts on: the side held for `persist_hours` consecutive bars.
    pub bull: bool,
    pub bear: bool,
    /// The persistence window, oldest first, current hour last.
    pub history: Vec<MomentumHour>,
}

impl MomentumReading {
    pub fn summary(&self) -> String {
        let mom = match self.mom_pct {
            Some(mom) => format!("{mom:.2}%"),
            None => "n/a".to_string(),
        };
        format!(
            "mom {}, raw bull {} bear {}, persisted bull {} bear {} over {}h",
            mom,
            self.bull_raw,
            self.bear_raw,
            self.bull,
            self.bear,
            self.history.len()
        )
    }
}

/// The raw sides for one hour. A missing input is OFF on both sides, never ON (spec §2.5).
pub fn momentum_sides(mom_pct: Option<f64>, params: &MomentumGateParams) -> (bool, bool) {
    match mom_pct {
        Some(mom) => (mom > params.bull_threshold_pct, mom < params.bear_threshold_pct),
        None => (false, false),
    }
}

/// Reads the two-sided momentum gate as of `at_ts` from ClickHouse.
///
/// One query: the spot grid behind the momentum window, extended by the persistence so every
/// hour of that window can be evaluated.
pub async fn fetch_momentum_gate(
    client: &ClickhouseClient,
    params: &MomentumGateParams,
    at_ts: i64,
) -> Result<MomentumReading> {
    let hour = floor_to_hour(at_ts);
    let persist_bars = params.persist_hours.max(1) as i64;
    let history_from = hour - (persist_bars - 1) * BAR_SEC;
    let currency = &params.option_currency;

    let spot_from = history_from - (MOMENTUM_WINDOW_BARS as i64 + SEED_BARS) * BAR_SEC;
    let spot_rows = fetch_hourly_spot(client, currency, spot_from, hour).await?;
    if spot_rows.is_empty() {
        bail!("no {} spot observations between {} and {}", currency, spot_from, hour);
    }
    let grid_from = history_from - MOMENTUM_WINDOW_BARS as i64 * BAR_SEC;
    let grid = forward_filled_grid(&spot_as_grid(&spot_rows), grid_from, hour)?;
    let mut mom_by_hour: HashMap<i64, f64> = HashMap::new();
    for bar in MOMENTUM_WINDOW_BARS..grid.len() {
        let at = grid_from + bar as i64 * BAR_SEC;
        mom_by_hour.insert(at, momentum_pct(grid[bar], grid[bar - MOMENTUM_WINDOW_BARS])?);
    }

    let history: Vec<MomentumHour> = (0..persist_bars)
        .map(|back| {
            let at = history_from + back * BAR_SEC;
            let mom_pct = mom_by_hour.get(&at).copied();
            let (bull, bear) = momentum_sides(mom_pct, params);
            MomentumHour { hour: at, mom_pct, bull, bear }
        })
        .collect();

    let current = *history.last().expect("the window always holds the current hour");
    let bulls: Vec<bool> = history.iter().map(|entry| entry.bull).collect();
    let bears: Vec<bool> = history.iter().map(|entry| entry.bear).collect();
    let reading = MomentumReading {
        hour,
        mom_pct: current.mom_pct,
        bull_raw: current.bull,
        bear_raw: current.bear,
        bull: apply_debounce(&bulls, params.persist_hours.max(1)),
        bear: apply_debounce(&bears, params.persist_hours.max(1)),
        history,
    };
    info!("momentum gate at {}: {}", hour, reading.summary());
    Ok(reading)
}

#[cfg(test)]
mod momentum_gate_tests {
    use super::*;

    fn params() -> MomentumGateParams {
        MomentumGateParams::weathervane("ETH")
    }

    #[test]
    fn the_dead_zone_holds_both_sleeves_off() {
        let p = params();
        // strictly above / below, so the thresholds themselves are inside the dead zone
        assert_eq!(momentum_sides(Some(5.01), &p), (true, false));
        assert_eq!(momentum_sides(Some(5.0), &p), (false, false));
        assert_eq!(momentum_sides(Some(0.0), &p), (false, false));
        assert_eq!(momentum_sides(Some(-5.0), &p), (false, false));
        assert_eq!(momentum_sides(Some(-5.01), &p), (false, true));
        // and the two sides can never both be on
        for mom in [-40.0, -5.0, 0.0, 5.0, 40.0] {
            let (bull, bear) = momentum_sides(Some(mom), &p);
            assert!(!(bull && bear), "both sleeves on at {mom}");
        }
        // a missing input is off on both sides
        assert_eq!(momentum_sides(None, &p), (false, false));
    }

    #[test]
    fn persistence_requires_six_consecutive_hours() {
        // the sleeve acts only once the side has held for the whole window
        assert!(apply_debounce(&[true; 6], 6));
        assert!(!apply_debounce(&[false, true, true, true, true, true], 6));
        assert!(!apply_debounce(&[true, true, true, true, true, false], 6));
        // a flip inside the window resets it, which is the anti-churn property
        assert!(!apply_debounce(&[true, true, false, true, true, true], 6));
    }
}

#[cfg(test)]
mod momentum_live_tests {
    use super::*;
    use crate::clickhouse::test_support::clickhouse_client;

    /// Reads the live Weathervane gate end to end:
    /// `ENV=staging cargo test -p lyra-vaults --lib -- --ignored reads_the_live_momentum_gate --nocapture`
    #[tokio::test]
    #[ignore]
    async fn reads_the_live_momentum_gate() {
        let client = clickhouse_client();
        let params = MomentumGateParams::weathervane("ETH");
        let now = chrono::Utc::now().timestamp();
        let gate = fetch_momentum_gate(&client, &params, now).await.unwrap();

        println!("\n{}", gate.summary());
        for entry in &gate.history {
            println!(
                "  {} mom {:?} bull {} bear {}",
                entry.hour, entry.mom_pct, entry.bull, entry.bear
            );
        }

        assert_eq!(gate.history.len(), params.persist_hours);
        assert_eq!(gate.hour, floor_to_hour(now));
        assert_eq!(gate.history[gate.history.len() - 1].hour, gate.hour);
        assert!(gate.mom_pct.is_some(), "no momentum for the current hour");
        // the sleeves are mutually exclusive, and each needs the whole window
        assert!(!(gate.bull && gate.bear));
        assert_eq!(gate.bull, gate.history.iter().all(|entry| entry.bull));
        assert_eq!(gate.bear, gate.history.iter().all(|entry| entry.bear));
    }
}
