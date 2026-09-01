//! Constant-maturity ATM implied vol and its 90-day percentile rank ("IV rank").
//!
//! # ATM vol definition
//!
//! Per each expiry's fitted SVI slice, evaluated at log-moneyness `k = 0` (i.e. struck at that
//! expiry's forward), then interpolated in TOTAL VARIANCE to a constant 30-day maturity:
//!
//! ```text
//! w_i(0)    = a + b * ( rho * (-m) + sqrt(m^2 + sigma^2) )     total variance at k = 0
//! w(T)      = w_1 + (w_2 - w_1) * (T - t_1) / (t_2 - t_1)      t_1 <= T <= t_2, the bracket
//! atm_iv(T) = sqrt( w(T) / T ) * 100                           annualised vol, in percent
//! ```
//!
//! NOTE this is deliberately NOT the definition in §2.1 of the strategy spec, which ships the
//! median of the per-expiry `k = 0` vols across every expiry with tau in [15, 45] days. The spec
//! measured a single constant-30d point (its option (b)) at only ~83-85% gate agreement against
//! the shipped series and advised against it. Constant-maturity interpolation is the production
//! choice here regardless, so expect the IV-rank gate to disagree with the backtest's on a few
//! percent of hours — see `matches_the_reference_on_clickhouse_iv` for the measured figure.
//!
//! # IV rank (spec §2.2)
//!
//! The percentile rank of `atm_iv(t)` within the trailing 90 days (2160 hourly bars, at least 540
//! of them), times 100. The window includes the current observation, so the rank is in (0, 100]
//! and an all-time high scores exactly 100. Ties take their average rank, matching the pandas
//! `rolling(...).rank(pct=True)` the reference vectors were generated with.

use crate::clickhouse::client::{de_i64, validate_currency, ClickhouseClient};
use crate::clickhouse::grid::{floor_to_hour, forward_filled_grid, BAR_SEC};
use anyhow::{bail, Error, Result};
use serde::Deserialize;
use std::collections::BTreeMap;

/// 90 days of hourly bars.
pub const IV_RANK_LOOKBACK_BARS: usize = 2160;
/// Warm-up: below this many observations the rank is undefined and the gate must read OFF.
pub const IV_RANK_MIN_BARS: usize = 540;
/// The constant maturity the surface is interpolated to.
pub const CONSTANT_MATURITY_DAYS: f64 = 30.0;
/// Days per year, matching the feed's own `svi_ref_tau` convention.
pub const YEAR_DAYS: f64 = 365.0;

const VOL_TABLE: &str = "default.raw_vol_feed";

/// The constant maturity in years.
pub fn constant_maturity_tau() -> f64 {
    CONSTANT_MATURITY_DAYS / YEAR_DAYS
}

/// A raw SVI slice as stored per expiry per snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct SviSlice {
    #[serde(deserialize_with = "de_i64")]
    pub hour: i64,
    #[serde(deserialize_with = "de_i64")]
    pub expiry: i64,
    /// Time to expiry in years, as quoted with the fit.
    pub tau: f64,
    pub a: f64,
    pub b: f64,
    pub rho: f64,
    pub m: f64,
    pub sigma: f64,
}

impl SviSlice {
    /// Total implied variance at log-moneyness `k`, i.e. raw SVI `w(k)`.
    pub fn total_variance(&self, k: f64) -> f64 {
        let shifted = k - self.m;
        self.a + self.b * (self.rho * shifted + (shifted * shifted + self.sigma * self.sigma).sqrt())
    }

    /// Total implied variance at the forward, `w(0)`.
    pub fn atm_total_variance(&self) -> f64 {
        self.total_variance(0.0)
    }
}

/// One hour of the ATM vol series.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct HourlyIv {
    /// Start of the hour, unix seconds.
    #[serde(deserialize_with = "de_i64")]
    pub hour: i64,
    /// Annualised vol in percent, e.g. `62.4`.
    pub iv: f64,
}

/// Hourly IV rows as `(hour, value)` pairs for [forward_filled_grid].
pub fn as_grid(rows: &[HourlyIv]) -> Vec<(i64, f64)> {
    rows.iter().map(|row| (row.hour, row.iv)).collect()
}

/// A computed IV-rank observation, carrying its inputs so a decision can be explained.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IvRank {
    pub hour: i64,
    /// The constant-maturity ATM vol at `hour`, annualised percent.
    pub atm_iv: f64,
    /// Percentile rank of `atm_iv` in the trailing window, in (0, 100].
    pub rank: f64,
    /// Observations the rank was computed over.
    pub bars: usize,
}

/// Interpolates total variance to `target_tau` between the two slices that bracket it.
///
/// Refuses to extrapolate: if the listed expiries do not straddle the target there is no honest
/// constant-maturity point, and per spec §2.5 an uncomputable input must read as gate OFF rather
/// than as a guess.
pub fn interpolate_total_variance(slices: &[SviSlice], target_tau: f64) -> Result<f64> {
    if !(target_tau > 0.0) {
        bail!("target tau {} must be positive", target_tau);
    }
    let mut below: Option<&SviSlice> = None;
    let mut above: Option<&SviSlice> = None;
    for slice in slices.iter().filter(|slice| slice.tau > 0.0) {
        if slice.tau <= target_tau && below.map_or(true, |b| slice.tau > b.tau) {
            below = Some(slice);
        }
        if slice.tau >= target_tau && above.map_or(true, |a| slice.tau < a.tau) {
            above = Some(slice);
        }
    }
    let (below, above) = match (below, above) {
        (Some(below), Some(above)) => (below, above),
        _ => bail!(
            "no expiries bracketing tau {:.6} among {} slices",
            target_tau,
            slices.len()
        ),
    };

    let (w1, w2) = (below.atm_total_variance(), above.atm_total_variance());
    if !w1.is_finite() || !w2.is_finite() || w1 <= 0.0 || w2 <= 0.0 {
        bail!("total variances {} and {} are not usable", w1, w2);
    }
    // an exact hit needs no interpolation, and would divide by zero below
    if above.tau <= below.tau {
        return Ok(w1);
    }
    let weight = (target_tau - below.tau) / (above.tau - below.tau);
    Ok(w1 + (w2 - w1) * weight)
}

/// The constant-maturity ATM vol implied by `slices`, annualised and in percent.
pub fn constant_maturity_iv_pct(slices: &[SviSlice], target_tau: f64) -> Result<f64> {
    let total_variance = interpolate_total_variance(slices, target_tau)?;
    Ok((total_variance / target_tau).sqrt() * 100.0)
}

/// The percentile rank of `value` within `window`, in (0, 100].
///
/// Ties take their average rank, so this matches pandas `rank(pct=True, method='average')`.
/// `window` is expected to contain `value` itself, as the spec's rolling window does.
pub fn percentile_rank(window: &[f64], value: f64) -> Result<f64> {
    if window.is_empty() {
        bail!("cannot rank against an empty window");
    }
    let mut less = 0usize;
    let mut equal = 0usize;
    for other in window {
        if !other.is_finite() {
            bail!("window contains a non-finite value");
        }
        if *other < value {
            less += 1;
        } else if *other == value {
            equal += 1;
        }
    }
    if equal == 0 {
        bail!("the window does not contain the ranked value");
    }
    let average_rank = less as f64 + (equal as f64 + 1.0) / 2.0;
    Ok(average_rank / window.len() as f64 * 100.0)
}

/// The rolling percentile rank of every bar of a filled series, `None` while below `min_bars`.
///
/// The window is trailing and inclusive of the current bar, and expands up to `lookback_bars`,
/// which is what pandas `rolling(lookback, min_periods=min_bars)` does.
pub fn iv_rank_series(
    iv: &[f64],
    lookback_bars: usize,
    min_bars: usize,
) -> Result<Vec<Option<f64>>> {
    if lookback_bars == 0 || min_bars == 0 {
        bail!("lookback and min bars must be at least one");
    }
    let mut out = Vec::with_capacity(iv.len());
    for bar in 0..iv.len() {
        let available = bar + 1;
        if available < min_bars {
            out.push(None);
            continue;
        }
        let start = available.saturating_sub(lookback_bars);
        out.push(Some(percentile_rank(&iv[start..available], iv[bar])?));
    }
    Ok(out)
}

/// Fetches the SVI slices of every listed expiry for each hour of `[from_hour, to_hour]`,
/// taking each hour's earliest snapshot.
pub async fn fetch_hourly_slices(
    client: &ClickhouseClient,
    currency: &str,
    from_hour: i64,
    to_hour: i64,
) -> Result<Vec<SviSlice>> {
    validate_currency(currency)?;
    let from_hour = floor_to_hour(from_hour);
    let until = floor_to_hour(to_hour) + BAR_SEC;
    // one argMin over a tuple, so every parameter comes from the same snapshot row
    let sql = format!(
        "SELECT hour, expiry, fit.1 AS tau, fit.2 AS a, fit.3 AS b, fit.4 AS rho, \
                fit.5 AS m, fit.6 AS sigma \
         FROM ( \
           SELECT intDiv(timestamp, {BAR_SEC}) * {BAR_SEC} AS hour, expiry, \
                  argMin(( \
                    toFloat64(svi_ref_tau), toFloat64(svi_a), toFloat64(svi_b), \
                    toFloat64(svi_rho), toFloat64(svi_m), toFloat64(svi_sigma) \
                  ), timestamp) AS fit \
           FROM {VOL_TABLE} \
           WHERE currency = '{currency}' AND timestamp >= {from_hour} AND timestamp < {until} \
           GROUP BY hour, expiry \
         ) ORDER BY hour, tau"
    );
    client.query(&sql).await
}

/// The hourly constant-maturity ATM vol series over `[from_hour, to_hour]`.
///
/// Hours whose listed expiries do not bracket the target maturity are omitted rather than
/// guessed, so the returned series can have holes; [forward_filled_grid] is what carries the
/// previous hour's value per spec §2.1, and the caller decides how much staleness is acceptable.
pub async fn fetch_hourly_atm_iv(
    client: &ClickhouseClient,
    currency: &str,
    from_hour: i64,
    to_hour: i64,
) -> Result<Vec<HourlyIv>> {
    let slices = fetch_hourly_slices(client, currency, from_hour, to_hour).await?;
    let target_tau = constant_maturity_tau();

    let mut by_hour: BTreeMap<i64, Vec<SviSlice>> = BTreeMap::new();
    for slice in slices {
        by_hour.entry(slice.hour).or_default().push(slice);
    }
    Ok(by_hour
        .into_iter()
        .filter_map(|(hour, slices)| {
            constant_maturity_iv_pct(&slices, target_tau)
                .ok()
                .map(|iv| HourlyIv { hour, iv })
        })
        .collect())
}

/// IV rank as of `at_ts`, over the default 90-day lookback.
///
/// Fails rather than guessing when the lookback holds fewer than [IV_RANK_MIN_BARS] observations,
/// which the caller must treat as gate OFF (spec §2.5).
pub async fn fetch_iv_rank(
    client: &ClickhouseClient,
    currency: &str,
    at_ts: i64,
) -> Result<IvRank> {
    let hour = floor_to_hour(at_ts);
    let from_hour = hour - IV_RANK_LOOKBACK_BARS as i64 * BAR_SEC;
    let rows = fetch_hourly_atm_iv(client, currency, from_hour, hour).await?;
    if rows.len() < IV_RANK_MIN_BARS {
        bail!(
            "only {} of the {} hours in the lookback have a usable surface, need {}",
            rows.len(),
            IV_RANK_LOOKBACK_BARS,
            IV_RANK_MIN_BARS
        );
    }

    // forward-fill from the first hour that actually has a fit, so the rank is taken over the
    // observations that exist rather than over a carry stretching back past the data
    let first_hour = rows.first().ok_or(Error::msg("empty surface series"))?.hour;
    let grid = forward_filled_grid(&as_grid(&rows), first_hour, hour)?;
    let ranks = iv_rank_series(&grid, IV_RANK_LOOKBACK_BARS, IV_RANK_MIN_BARS)?;
    let rank = ranks
        .last()
        .copied()
        .flatten()
        .ok_or(Error::msg("iv rank is still in warm-up"))?;
    let atm_iv = *grid.last().ok_or(Error::msg("empty surface grid"))?;
    Ok(IvRank { hour, atm_iv, rank, bars: grid.len().min(IV_RANK_LOOKBACK_BARS) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clickhouse::grid::grid_gaps;
    use crate::clickhouse::test_support::*;
    use std::collections::HashMap;

    fn slice(tau_days: f64, a: f64, b: f64, rho: f64, m: f64, sigma: f64) -> SviSlice {
        SviSlice { hour: 0, expiry: 0, tau: tau_days / YEAR_DAYS, a, b, rho, m, sigma }
    }

    /// A flat slice whose ATM total variance is exactly `vol^2 * tau`.
    fn flat_slice(tau_days: f64, vol: f64) -> SviSlice {
        let tau = tau_days / YEAR_DAYS;
        slice(tau_days, vol * vol * tau, 0.0, 0.0, 0.0, 0.0)
    }

    #[test]
    fn svi_evaluates_at_k_zero() {
        // a real row: ETH's 2026-08-28 expiry, 1.5 days out
        let s = SviSlice {
            hour: 0,
            expiry: 1787731200,
            tau: 0.00410832064942,
            a: 0.00040950354973679,
            b: 0.021550751096131467,
            rho: 0.04058678287769838,
            m: -0.014636514118657152,
            sigma: 0.045233615523965875,
        };
        let w0 = s.atm_total_variance();
        assert_eq!(format!("{:.12}", w0), "0.001446886464");
        // annualised: sqrt(w0 / tau), about 59.3% for that slice
        assert_eq!(format!("{:.6}", (w0 / s.tau).sqrt() * 100.0), "59.345127");

        // and away from the forward the smile is higher on both sides for a symmetric fit
        let symmetric = slice(30.0, 0.01, 0.05, 0.0, 0.0, 0.1);
        let atm = symmetric.total_variance(0.0);
        assert!(symmetric.total_variance(0.2) > atm);
        assert!(symmetric.total_variance(-0.2) > atm);
    }

    #[test]
    fn interpolation_is_linear_in_total_variance() {
        let target = constant_maturity_tau();
        // 20% vol at 10 days and 40% at 50 days: total variances 0.001096 and 0.021918
        let slices = vec![flat_slice(10.0, 0.20), flat_slice(50.0, 0.40)];
        let w = interpolate_total_variance(&slices, target).unwrap();
        let (t1, t2) = (10.0 / YEAR_DAYS, 50.0 / YEAR_DAYS);
        let (w1, w2) = (0.20 * 0.20 * t1, 0.40 * 0.40 * t2);
        let expected = w1 + (w2 - w1) * (target - t1) / (t2 - t1);
        assert!((w - expected).abs() < 1e-15, "{w} vs {expected}");
        // interpolating total variance, NOT vol: at 30d (halfway in tau) the 50d slice's much
        // larger total variance pulls the point to 37.4%, above the 30% midpoint of the two vols
        let iv = constant_maturity_iv_pct(&slices, target).unwrap();
        assert_eq!(format!("{:.6}", iv), "37.416574");

        // an exact hit returns that slice untouched
        let exact = vec![flat_slice(10.0, 0.20), flat_slice(30.0, 0.55), flat_slice(50.0, 0.40)];
        assert_eq!(format!("{:.6}", constant_maturity_iv_pct(&exact, target).unwrap()), "55.000000");
    }

    #[test]
    fn interpolation_picks_the_tightest_bracket() {
        let target = constant_maturity_tau();
        let slices = vec![
            flat_slice(1.0, 0.90),
            flat_slice(10.0, 0.20),
            flat_slice(29.0, 0.50), // nearest below
            flat_slice(31.0, 0.52), // nearest above
            flat_slice(180.0, 0.10),
        ];
        let iv = constant_maturity_iv_pct(&slices, target).unwrap();
        // between the 29d and 31d slices, so between 50% and 52%
        assert!(iv > 50.0 && iv < 52.0, "{iv}");
    }

    #[test]
    fn without_a_bracket_there_is_no_constant_maturity_point() {
        let target = constant_maturity_tau();
        // every listed expiry is short of 30 days: extrapolating would be a guess
        let short = vec![flat_slice(1.0, 0.5), flat_slice(7.0, 0.5), flat_slice(21.0, 0.5)];
        assert!(interpolate_total_variance(&short, target).is_err());
        // and likewise when they are all longer
        let long = vec![flat_slice(60.0, 0.5), flat_slice(180.0, 0.5)];
        assert!(interpolate_total_variance(&long, target).is_err());
        assert!(interpolate_total_variance(&[], target).is_err());
        assert!(interpolate_total_variance(&short, -1.0).is_err());
    }

    #[test]
    fn percentile_rank_matches_pandas_semantics() {
        // strictly increasing: the last value is the maximum, so it ranks 100
        assert_eq!(percentile_rank(&[1.0, 2.0, 3.0, 4.0], 4.0).unwrap(), 100.0);
        // the minimum of four distinct values ranks 1/4
        assert_eq!(percentile_rank(&[1.0, 2.0, 3.0, 4.0], 1.0).unwrap(), 25.0);
        assert_eq!(percentile_rank(&[1.0, 2.0, 3.0, 4.0], 3.0).unwrap(), 75.0);
        // ties take their average rank: two 2.0s occupy ranks 2 and 3, so 2.5 / 4
        assert_eq!(percentile_rank(&[1.0, 2.0, 2.0, 4.0], 2.0).unwrap(), 62.5);
        // a single observation is its own maximum
        assert_eq!(percentile_rank(&[5.0], 5.0).unwrap(), 100.0);
        // the rank is never 0
        assert!(percentile_rank(&[1.0, 2.0], 1.0).unwrap() > 0.0);

        assert!(percentile_rank(&[], 1.0).is_err());
        assert!(percentile_rank(&[1.0, 2.0], 3.0).is_err(), "value must be in the window");
        assert!(percentile_rank(&[f64::NAN], 1.0).is_err());
    }

    #[test]
    fn rank_series_warms_up_then_rolls() {
        let iv: Vec<f64> = (1..=10).map(|v| v as f64).collect();
        let ranks = iv_rank_series(&iv, 4, 3).unwrap();
        assert_eq!(ranks.len(), 10);
        // below min_bars there is no rank
        assert_eq!(ranks[0], None);
        assert_eq!(ranks[1], None);
        // the window expands until it reaches the lookback...
        assert_eq!(ranks[2], Some(100.0));
        // ...and every later bar is the max of its trailing four, so also 100
        assert_eq!(ranks[9], Some(100.0));

        // a falling series puts every bar at the bottom of its own window
        let falling: Vec<f64> = (1..=10).rev().map(|v| v as f64).collect();
        let ranks = iv_rank_series(&falling, 4, 3).unwrap();
        assert!((ranks[2].unwrap() - 100.0 / 3.0).abs() < 1e-12);
        assert_eq!(ranks[9], Some(25.0));

        assert!(iv_rank_series(&iv, 0, 3).is_err());
        assert!(iv_rank_series(&iv, 4, 0).is_err());
    }

    /// Recomputes IV rank from the reference series' own `atm_iv_pct` column and requires it to
    /// match `iv_rank_pctile`. This pins the rolling-percentile semantics (window, warm-up, tie
    /// handling) independently of where the vol numbers come from.
    ///
    ///   `cargo test -p lyra-vaults --lib matches_the_backtest_reference_iv_rank -- --nocapture`
    #[test]
    fn matches_the_backtest_reference_iv_rank() {
        // The reference stores atm_iv rounded to 2 decimals, so our input is not bit-identical to
        // the series it ranked: near-ties can order differently, and one observation out of 2160
        // is worth 0.046 percentile points. That sets the floor on achievable agreement — the
        // measured p95 is 0.049, i.e. a single observation's worth.
        const TOLERANCE: f64 = 0.3;
        const SHOW: usize = 10;

        let Some(rows) = load_reference(IV_RANK_LOOKBACK_BARS) else { return };
        let ivs: Vec<(i64, f64)> = rows.iter().map(|row| (row.hour, row.atm_iv)).collect();
        let grid = forward_filled_grid(&ivs, rows[0].hour, rows[rows.len() - 1].hour).unwrap();
        let ranks = iv_rank_series(&grid, IV_RANK_LOOKBACK_BARS, IV_RANK_MIN_BARS).unwrap();
        // the grid is hour-indexed and the reference is row-indexed, and they differ by the 84
        // filled hours of its gap, so key the ranks by hour rather than zipping positionally
        let by_hour: HashMap<i64, f64> = ranks
            .iter()
            .enumerate()
            .filter_map(|(bar, rank)| {
                rank.map(|rank| (rows[0].hour + bar as i64 * BAR_SEC, rank))
            })
            .collect();

        let mut diffs: Diffs = Vec::new();
        let mut post_gap: Diffs = Vec::new();
        let mut missing = Vec::new();
        let mut unexpected = Vec::new();
        for row in &rows {
            match (by_hour.get(&row.hour), row.iv_rank) {
                (Some(ours), Some(expected)) => {
                    let entry = (row.hour, *ours, expected, ours - expected);
                    match row.post_gap {
                        true => post_gap.push(entry),
                        false => diffs.push(entry),
                    }
                }
                (Some(_), None) => unexpected.push(row.hour),
                (None, Some(_)) => missing.push(row.hour),
                (None, None) => {}
            }
        }

        let (stats, _) = report("reference atm_iv", &diffs, SHOW);
        let over: Diffs =
            diffs.iter().filter(|(_, _, _, d)| d.abs() >= TOLERANCE).cloned().collect();
        println!(
            "  {} of {} hours over tolerance {:e}; {} excluded as post-gap; {} we compute that the \
             reference leaves null; {} the reference has that we cannot compute",
            over.len(),
            stats.compared,
            TOLERANCE,
            post_gap.len(),
            unexpected.len(),
            missing.len()
        );
        if !post_gap.is_empty() {
            report("post-gap rows (excluded)", &post_gap, SHOW);
        }

        // asserts last, so the tables above always print
        assert!(stats.compared > 25_000, "only {} hours compared", stats.compared);
        assert!(missing.is_empty(), "{} reference hours we could not compute", missing.len());
        // the reference's own warm-up is 540 bars, so it may hold values where we do too
        assert!(unexpected.len() <= IV_RANK_MIN_BARS, "{} unexpected hours", unexpected.len());
        assert!(
            over.is_empty(),
            "{} of {} hours disagree beyond {:e}, worst {:.6} at {} - see the tables above",
            over.len(),
            stats.compared,
            TOLERANCE,
            over[0].3,
            fmt_hour(over[0].0)
        );
        assert!(stats.max_abs < TOLERANCE, "max abs diff {:.9}", stats.max_abs);
        assert!(stats.mean_abs < 0.05, "mean abs diff {:.9}", stats.mean_abs);
        assert!(
            stats.mean_signed.abs() < 0.01,
            "mean signed diff {:+.9} suggests a systematic bias, not rounding",
            stats.mean_signed
        );
    }

    /// The same comparison on ATM vol read from ClickHouse: constant-maturity SVI k=0 rather than
    /// the reference's median-of-band markIv. Reports the level agreement and, more importantly,
    /// how often the IV-rank gate would land on the same side of each book's floor.
    ///
    /// Expect real disagreement here: per spec §2.1 this is definition (b), which the research
    /// measured at ~83-85% gate agreement against the shipped series. The floors below are what
    /// this implementation actually achieves, so a regression moves them, but they are NOT a claim
    /// that the two definitions are interchangeable.
    ///
    ///   `cargo test -p lyra-vaults --lib matches_the_reference_on_clickhouse_iv -- --nocapture`
    #[test]
    fn matches_the_reference_on_clickhouse_iv() {
        const SHOW: usize = 10;
        /// Hours of slack tolerated inside the "dense" run of the vol feed.
        const MAX_GAP_BARS: i64 = 6;

        let Some(rows) = load_reference(IV_RANK_LOOKBACK_BARS) else { return };
        let Some(ch) = load_ch_fixture(
            CH_IV_PATH,
            "iv",
            "ENV=staging cargo test -p lyra-vaults --lib -- --ignored \
             generate_clickhouse_iv_series --nocapture",
        ) else {
            return;
        };

        // The vol feed is only continuously populated for part of its span, so work inside its
        // longest dense run. The rank needs a full lookback of that run before it is comparable
        // with the reference's, which always has one.
        let hours: Vec<i64> = ch.iter().map(|(hour, _)| *hour).collect();
        let (dense_from, dense_to) = longest_dense_run(&hours, MAX_GAP_BARS);
        let ref_from = rows[0].hour;
        let ref_to = rows[rows.len() - 1].hour;
        let level_from = dense_from.max(ref_from);
        let level_to = dense_to.min(ref_to);
        let rank_from = level_from + IV_RANK_LOOKBACK_BARS as i64 * BAR_SEC;
        println!(
            "clickhouse atm_iv: {} hours, {} .. {}\n  longest dense run (gaps <= {}h): {} .. {}\
             \n  levels compared from {}, ranks from {} (a full lookback in)",
            ch.len(),
            fmt_hour(hours[0]),
            fmt_hour(hours[hours.len() - 1]),
            MAX_GAP_BARS,
            fmt_hour(dense_from),
            fmt_hour(dense_to),
            fmt_hour(level_from),
            fmt_hour(rank_from)
        );
        assert!(level_from < level_to, "the two series do not overlap");

        let grid = forward_filled_grid(&ch, level_from, level_to).unwrap();
        let ranks = iv_rank_series(&grid, IV_RANK_LOOKBACK_BARS, IV_RANK_MIN_BARS).unwrap();
        let mut our_iv: HashMap<i64, f64> = HashMap::new();
        let mut our_rank: HashMap<i64, f64> = HashMap::new();
        for (bar, value) in grid.iter().enumerate() {
            let hour = level_from + bar as i64 * BAR_SEC;
            our_iv.insert(hour, *value);
            if let Some(rank) = ranks[bar] {
                our_rank.insert(hour, rank);
            }
        }

        let mut levels: Diffs = Vec::new();
        let mut rank_diffs: Diffs = Vec::new();
        let mut gate40_agree = 0usize;
        let mut gate30_agree = 0usize;
        let mut gates = 0usize;
        for row in rows.iter().filter(|row| row.hour >= level_from && row.hour <= level_to) {
            if let Some(iv) = our_iv.get(&row.hour) {
                levels.push((row.hour, *iv, row.atm_iv, iv - row.atm_iv));
            }
            if row.post_gap || row.hour < rank_from {
                continue;
            }
            let (Some(ours), Some(expected)) = (our_rank.get(&row.hour), row.iv_rank) else {
                continue;
            };
            rank_diffs.push((row.hour, *ours, expected, ours - expected));
            gates += 1;
            if (*ours >= 40.0) == (expected >= 40.0) {
                gate40_agree += 1;
            }
            if (*ours >= 30.0) == (expected >= 30.0) {
                gate30_agree += 1;
            }
        }

        let (level_stats, _) = report("clickhouse atm_iv level (vol points)", &levels, SHOW);
        let (rank_stats, _) = report("clickhouse iv_rank (percentile points)", &rank_diffs, SHOW);
        let gate40 = gate40_agree as f64 / gates.max(1) as f64;
        let gate30 = gate30_agree as f64 / gates.max(1) as f64;
        println!(
            "\n  iv_rank >= 40 (Swather) agreement {:.3}% ({} of {} hours)\
             \n  iv_rank >= 30 (Plume)   agreement {:.3}% ({} of {} hours)\
             \n  spec §2.1(b) measured ~83-85% for a constant-maturity definition",
            gate40 * 100.0,
            gate40_agree,
            gates,
            gate30 * 100.0,
            gate30_agree,
            gates
        );

        // asserts last, so the tables above always print
        assert!(level_stats.compared > 3_000, "only {} levels compared", level_stats.compared);
        assert!(rank_stats.compared > 1_000, "only {} ranks compared", rank_stats.compared);
        // the level is the part this implementation controls: a constant-maturity k=0 point should
        // track the reference's median-of-band proxy to a couple of vol points
        assert!(
            level_stats.p95 < 4.0,
            "p95 level diff {:.6} vol points is larger than expected",
            level_stats.p95
        );
        assert!(
            level_stats.mean_abs < 2.5,
            "mean level diff {:.6} vol points is larger than expected",
            level_stats.mean_abs
        );
        // the rank is a different definition on a shorter history, so only a floor is meaningful
        assert!(
            gate40 >= 0.80,
            "Swather gate agreement {:.3}% fell below the 80% floor",
            gate40 * 100.0
        );
        assert!(
            gate30 >= 0.80,
            "Plume gate agreement {:.3}% fell below the 80% floor",
            gate30 * 100.0
        );
    }

    /// Hits ClickHouse: `ENV=staging cargo test -p lyra-vaults --lib -- --ignored --nocapture`
    #[tokio::test]
    #[ignore]
    async fn fetches_iv_rank_from_clickhouse() {
        let client = clickhouse_client();
        let now = chrono::Utc::now().timestamp();
        let rank = fetch_iv_rank(&client, "ETH", now).await.unwrap();
        println!("{:?}", rank);
        assert_eq!(rank.hour, floor_to_hour(now));
        assert!(rank.atm_iv > 0.0 && rank.atm_iv < 500.0);
        assert!(rank.rank > 0.0 && rank.rank <= 100.0);
        assert!(rank.bars >= IV_RANK_MIN_BARS);
    }

    /// Writes the hourly constant-maturity ATM vol series to [CH_IV_PATH], for
    /// [matches_the_reference_on_clickhouse_iv] to run against offline. Regenerate with:
    ///
    ///   `ENV=staging cargo test -p lyra-vaults --lib -- --ignored \
    ///    generate_clickhouse_iv_series --nocapture`
    #[tokio::test]
    #[ignore]
    async fn generate_clickhouse_iv_series() {
        const CURRENCY: &str = "ETH";
        const CHUNK_BARS: i64 = 24 * 30;

        let client = clickhouse_client();
        let bounds: Vec<HourlyIv> = client
            .query(&format!(
                "SELECT toInt64(min(timestamp)) AS hour, toFloat64(max(timestamp)) AS iv \
                 FROM {VOL_TABLE} WHERE currency = '{CURRENCY}'"
            ))
            .await
            .unwrap();
        let feed_from = floor_to_hour(bounds[0].hour);
        let feed_to = floor_to_hour(bounds[0].iv as i64);
        let reference_to =
            load_reference(IV_RANK_LOOKBACK_BARS).map(|rows| rows[rows.len() - 1].hour);
        let to_hour = reference_to.unwrap_or(feed_to).min(feed_to);
        println!(
            "\n{CURRENCY} vol feed spans {} .. {}, fetching {} .. {}",
            fmt_hour(feed_from),
            fmt_hour(feed_to),
            fmt_hour(feed_from),
            fmt_hour(to_hour)
        );

        let mut rows: Vec<HourlyIv> = Vec::new();
        let mut chunk_from = feed_from;
        while chunk_from <= to_hour {
            let chunk_to = (chunk_from + (CHUNK_BARS - 1) * BAR_SEC).min(to_hour);
            let chunk =
                fetch_hourly_atm_iv(&client, CURRENCY, chunk_from, chunk_to).await.unwrap();
            let bars = (chunk_to - chunk_from) / BAR_SEC + 1;
            println!(
                "  {} .. {}: {} of {} hours have a 30d bracket",
                fmt_hour(chunk_from),
                fmt_hour(chunk_to),
                chunk.len(),
                bars
            );
            rows.extend(chunk);
            chunk_from = chunk_to + BAR_SEC;
        }

        rows.sort_by_key(|row| row.hour);
        rows.dedup_by_key(|row| row.hour);
        assert!(rows.len() > 5_000, "only got {} hours", rows.len());
        let hours: Vec<i64> = rows.iter().map(|row| row.hour).collect();
        let spanned = (hours[hours.len() - 1] - hours[0]) / BAR_SEC + 1;
        let gaps = grid_gaps(&hours);
        println!(
            "fetched {} hours over a {} hour span, {} missing, {} gaps",
            rows.len(),
            spanned,
            spanned as usize - rows.len(),
            gaps.len()
        );
        for (from, to) in gaps.iter().take(10) {
            println!(
                "  gap: {} -> {} ({} hours missing)",
                fmt_hour(*from),
                fmt_hour(*to),
                (to - from) / BAR_SEC - 1
            );
        }
        let (dense_from, dense_to) = longest_dense_run(&hours, 6);
        println!(
            "longest dense run: {} .. {} ({} hours)",
            fmt_hour(dense_from),
            fmt_hour(dense_to),
            (dense_to - dense_from) / BAR_SEC + 1
        );

        write_fixture(
            CH_IV_PATH,
            &format!(
                "Hourly {CURRENCY} ATM implied vol from {VOL_TABLE}: each expiry's SVI slice at \
                 k=0, interpolated in total variance to a constant {CONSTANT_MATURITY_DAYS}-day \
                 maturity, annualised and in percent. Hours whose listed expiries do not bracket \
                 the target maturity are absent. Test fixture; regenerate with the \
                 generate_clickhouse_iv_series test."
            ),
            &[
                ("currency", serde_json::json!(CURRENCY).to_string()),
                ("table", serde_json::json!(VOL_TABLE).to_string()),
                ("constant_maturity_days", CONSTANT_MATURITY_DAYS.to_string()),
            ],
            "iv",
            &as_grid(&rows),
        );
    }
}
