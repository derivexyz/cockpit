//! 30-day trailing spot momentum, per §2.3 of the Swather/Plume strategy spec:
//!
//! ```text
//! mom(t) = ( spot(t) / spot(t - 720 hourly bars) - 1 ) x 100
//! ```
//!
//! `spot` is the venue index price on an hourly grid, taking the FIRST observation of each hour
//! (the backtest reads the first quote row's index price per hour). The shift is 720 BARS on that
//! grid, not 720 wall-clock hours, so hours missing from the feed are forward-filled rather than
//! skipped — a gap must not silently shorten the lookback.
//!
//! The fetch functions are thin: all of the grid and arithmetic logic is in pure functions so it
//! can be tested against the reference vectors in `signals/signals_timeseries.json`.

use crate::clickhouse::client::{de_i64, validate_currency, ClickhouseClient};
use crate::clickhouse::grid::{floor_to_hour, forward_filled_grid, BAR_SEC};
use anyhow::{bail, Error, Result};
use serde::Deserialize;

/// 30 days of hourly bars.
pub const MOMENTUM_WINDOW_BARS: usize = 720;
/// Hours fetched before the window starts, so that a missing first hour can still be carried
/// forward from an earlier observation instead of failing.
pub const SEED_BARS: i64 = 24;

const SPOT_TABLE: &str = "default.raw_spot_feed";

/// One hour of the spot grid.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct HourlySpot {
    /// Start of the hour, unix seconds.
    #[serde(deserialize_with = "de_i64")]
    pub hour: i64,
    pub spot: f64,
}

/// Hourly spot rows as `(hour, value)` pairs for [forward_filled_grid].
pub fn as_grid(rows: &[HourlySpot]) -> Vec<(i64, f64)> {
    rows.iter().map(|row| (row.hour, row.spot)).collect()
}

/// A computed momentum observation, carrying its inputs so a decision can be explained.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Momentum {
    /// The hour the momentum is as of, unix seconds.
    pub hour: i64,
    pub spot: f64,
    /// The hour `window_bars` earlier, i.e. the denominator's bar.
    pub reference_hour: i64,
    pub reference_spot: f64,
    /// The trailing return in percent, e.g. `5.0` for +5%.
    pub pct: f64,
}

/// The trailing return in percent between two spot observations.
pub fn momentum_pct(spot: f64, reference_spot: f64) -> Result<f64> {
    if !spot.is_finite() || spot <= 0.0 {
        bail!("spot {} is not a usable price", spot);
    }
    if !reference_spot.is_finite() || reference_spot <= 0.0 {
        bail!("reference spot {} is not a usable price", reference_spot);
    }
    Ok((spot / reference_spot - 1.0) * 100.0)
}

/// Momentum for every hour of a filled grid that has a full window behind it. `first_hour` is the
/// hour of `grid[0]`. Returns an empty vec if the grid is shorter than the window.
pub fn momentum_series(grid: &[f64], first_hour: i64, window_bars: usize) -> Result<Vec<Momentum>> {
    if window_bars == 0 {
        bail!("momentum window must be at least one bar");
    }
    let first_hour = floor_to_hour(first_hour);
    let mut out = Vec::new();
    for bar in window_bars..grid.len() {
        let reference_spot = grid[bar - window_bars];
        let spot = grid[bar];
        out.push(Momentum {
            hour: first_hour + bar as i64 * BAR_SEC,
            spot,
            reference_hour: first_hour + (bar - window_bars) as i64 * BAR_SEC,
            reference_spot,
            pct: momentum_pct(spot, reference_spot)?,
        });
    }
    Ok(out)
}

/// Fetches the hourly spot grid for `[from_hour, to_hour]` inclusive, one row per hour that has
/// data. Each hour's value is its earliest observation.
pub async fn fetch_hourly_spot(
    client: &ClickhouseClient,
    currency: &str,
    from_hour: i64,
    to_hour: i64,
) -> Result<Vec<HourlySpot>> {
    validate_currency(currency)?;
    let from_hour = floor_to_hour(from_hour);
    let until = floor_to_hour(to_hour) + BAR_SEC;
    let sql = format!(
        "SELECT intDiv(timestamp, {BAR_SEC}) * {BAR_SEC} AS hour, \
                toFloat64(argMin(price, timestamp)) AS spot \
         FROM {SPOT_TABLE} \
         WHERE currency = '{currency}' AND timestamp >= {from_hour} AND timestamp < {until} \
         GROUP BY hour ORDER BY hour"
    );
    client.query(&sql).await
}

/// Momentum as of `at_ts`, over the default 30-day window.
///
/// The hour containing `at_ts` is used as-is, so calling this mid-hour reads a bar that is still
/// forming — which is what the strategy wants, since it decides on the current hour's spot.
pub async fn fetch_momentum(
    client: &ClickhouseClient,
    currency: &str,
    at_ts: i64,
) -> Result<Momentum> {
    fetch_momentum_with_window(client, currency, at_ts, MOMENTUM_WINDOW_BARS).await
}

/// [fetch_momentum] with an explicit window length in bars.
pub async fn fetch_momentum_with_window(
    client: &ClickhouseClient,
    currency: &str,
    at_ts: i64,
    window_bars: usize,
) -> Result<Momentum> {
    if window_bars == 0 {
        bail!("momentum window must be at least one bar");
    }
    let hour = floor_to_hour(at_ts);
    let reference_hour = hour - window_bars as i64 * BAR_SEC;
    let rows =
        fetch_hourly_spot(client, currency, reference_hour - SEED_BARS * BAR_SEC, hour).await?;
    let grid = forward_filled_grid(&as_grid(&rows), reference_hour, hour)?;

    let reference_spot = *grid.first().ok_or(Error::msg("empty spot grid"))?;
    let spot = *grid.last().ok_or(Error::msg("empty spot grid"))?;
    Ok(Momentum {
        hour,
        spot,
        reference_hour,
        reference_spot,
        pct: momentum_pct(spot, reference_spot)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clickhouse::test_support::*;
    use std::collections::HashMap;

    /// Recomputes momentum from the reference series' own spot column and requires it to match
    /// `mom_30d_pct`, which is what pins the 720-bar shift semantics to the shipped signal.
    ///
    ///   `cargo test -p lyra-vaults --lib matches_the_backtest_reference_vectors -- --nocapture`
    #[test]
    fn matches_the_backtest_reference_vectors() {
        // the reference rounds mom to 3 decimals (5e-4) and spot to 2 (another ~6e-4 once it
        // propagates through the ratio), so agreement can only be asserted to about 1e-3
        const TOLERANCE: f64 = 2e-3;
        const SHOW: usize = 10;

        let Some(rows) = load_reference(MOMENTUM_WINDOW_BARS) else { return };
        let spots: Vec<(i64, f64)> = rows.iter().map(|row| (row.hour, row.spot)).collect();
        let grid = forward_filled_grid(&spots, rows[0].hour, rows[rows.len() - 1].hour).unwrap();
        let computed = momentum_series(&grid, rows[0].hour, MOMENTUM_WINDOW_BARS).unwrap();
        let by_hour: HashMap<i64, f64> = computed.iter().map(|m| (m.hour, m.pct)).collect();

        // compare hour by hour, collecting every disagreement rather than failing on the first
        let mut diffs: Diffs = Vec::new();
        let mut post_gap: Diffs = Vec::new();
        let mut missing = Vec::new();
        let mut unexpected = Vec::new();
        for row in &rows {
            match (by_hour.get(&row.hour), row.mom) {
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

        let (stats, _) = report("reference spot", &diffs, SHOW);
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

        // the excluded rows, for inspection: these are where the reference's 720-ROW lookback
        // crosses its own gap and ours stays at 720 hours
        if !post_gap.is_empty() {
            report("post-gap rows (excluded)", &post_gap, SHOW);
        }

        // asserts last, so the tables above always print
        assert!(stats.compared > 30_000, "only {} hours compared", stats.compared);
        assert!(missing.is_empty(), "{} reference hours we could not compute", missing.len());
        assert!(unexpected.is_empty(), "{} hours the reference leaves null", unexpected.len());
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
        // and in aggregate the diffs must look like rounding, not like a systematic offset
        assert!(stats.mean_abs < 5e-4, "mean abs diff {:.9}", stats.mean_abs);
        assert!(
            stats.mean_signed.abs() < 1e-4,
            "mean signed diff {:+.9} suggests a systematic bias, not rounding",
            stats.mean_signed
        );
        assert_eq!(post_gap.len(), MOMENTUM_WINDOW_BARS, "post-gap exclusion window changed");
    }

    /// The same comparison, but on spot read from ClickHouse instead of the reference's own
    /// column: this is the end-to-end check of the series the production signal will actually see.
    /// Only the hours both series cover are compared, since `raw_spot_feed` starts much later.
    ///
    ///   `cargo test -p lyra-vaults --lib matches_the_reference_on_clickhouse_spot -- --nocapture`
    #[test]
    fn matches_the_reference_on_clickhouse_spot() {
        const SHOW: usize = 10;
        /// These are two different index feeds, not the same numbers rounded, so the bounds are on
        /// the bulk of the distribution. Single hours can be far apart: both series carry the odd
        /// stale or thinly-sampled hour (e.g. at 2024-11-11T04:00Z the reference reads 2973.28
        /// where ClickHouse reads 3197.66, with the neighbouring hours agreeing to 0.05%), and one
        /// such hour shows up twice — once as a numerator, once as a denominator 30 days later.
        const MAX_P99_ABS_DIFF: f64 = 2.0;
        const MAX_MEAN_ABS_DIFF: f64 = 0.5;
        const MAX_MEAN_SIGNED_DIFF: f64 = 0.05;
        /// How many hours may sit far out in the tail.
        const OUTLIER_DIFF: f64 = 5.0;
        const MAX_OUTLIER_SHARE: f64 = 0.002;
        /// The decision-relevant metric: how often the two agree on `mom <= +5%`.
        const MIN_GATE_AGREEMENT: f64 = 0.99;

        let Some(rows) = load_reference(MOMENTUM_WINDOW_BARS) else { return };
        let Some(ch) = load_ch_fixture(
            CH_SPOT_PATH,
            "spot",
            "ENV=staging cargo test -p lyra-vaults --lib -- --ignored \
             generate_clickhouse_spot_series --nocapture",
        ) else {
            return;
        };

        // the comparable window is where both series have data, with 720 bars of ClickHouse
        // lead-in so the first compared hour has a full window behind it
        let ch_first = ch.first().unwrap().0;
        let ch_last = ch.last().unwrap().0;
        let from_hour = ch_first.max(rows[0].hour) + MOMENTUM_WINDOW_BARS as i64 * BAR_SEC;
        let to_hour = ch_last.min(rows[rows.len() - 1].hour);
        println!(
            "clickhouse spot: {} hours, {} .. {}\noverlap compared: {} .. {}",
            ch.len(),
            fmt_hour(ch_first),
            fmt_hour(ch_last),
            fmt_hour(from_hour),
            fmt_hour(to_hour)
        );
        assert!(from_hour < to_hour, "the two series do not overlap");

        let grid_from = from_hour - MOMENTUM_WINDOW_BARS as i64 * BAR_SEC;
        let grid = forward_filled_grid(&ch, grid_from, to_hour).unwrap();
        let computed = momentum_series(&grid, grid_from, MOMENTUM_WINDOW_BARS).unwrap();
        let by_hour: HashMap<i64, f64> = computed.iter().map(|m| (m.hour, m.pct)).collect();

        let mut diffs: Diffs = Vec::new();
        let mut gate_agree = 0usize;
        let mut skipped_post_gap = 0usize;
        for row in rows.iter().filter(|row| row.hour >= from_hour && row.hour <= to_hour) {
            let (Some(ours), Some(expected)) = (by_hour.get(&row.hour), row.mom) else {
                continue;
            };
            if row.post_gap {
                skipped_post_gap += 1;
                continue;
            }
            diffs.push((row.hour, *ours, expected, ours - expected));
            // the gate the strategy actually asks for: "not running hot"
            if (*ours <= 5.0) == (expected <= 5.0) {
                gate_agree += 1;
            }
        }

        let (stats, sorted_abs) = report("clickhouse spot", &diffs, SHOW);
        let agreement = gate_agree as f64 / stats.compared.max(1) as f64;
        let outlier_share = stats.share_over(&sorted_abs, OUTLIER_DIFF);
        println!(
            "  mom <= +5% gate agreement {:.3}% ({} of {} hours); {:.3}% of hours over {:.1}pp; \
             {} skipped as post-gap",
            agreement * 100.0,
            gate_agree,
            stats.compared,
            outlier_share * 100.0,
            OUTLIER_DIFF,
            skipped_post_gap
        );

        // asserts last, so the tables above always print
        assert!(stats.compared > 10_000, "only {} hours compared", stats.compared);
        assert!(
            stats.p99 < MAX_P99_ABS_DIFF,
            "p99 abs diff {:.6} between the feeds is larger than expected",
            stats.p99
        );
        assert!(
            stats.mean_abs < MAX_MEAN_ABS_DIFF,
            "mean abs diff {:.6} between the feeds is larger than expected",
            stats.mean_abs
        );
        // the feeds may disagree, but not in one direction: that would be a units or timing bug
        assert!(
            stats.mean_signed.abs() < MAX_MEAN_SIGNED_DIFF,
            "mean signed diff {:+.6} suggests a systematic bias rather than feed noise",
            stats.mean_signed
        );
        assert!(
            outlier_share < MAX_OUTLIER_SHARE,
            "{:.3}% of hours are over {:.1}pp apart, above the {:.3}% allowance",
            outlier_share * 100.0,
            OUTLIER_DIFF,
            MAX_OUTLIER_SHARE * 100.0
        );
        assert!(
            agreement >= MIN_GATE_AGREEMENT,
            "gate agreement {:.3}% is below the {:.1}% floor",
            agreement * 100.0,
            MIN_GATE_AGREEMENT * 100.0
        );
    }

    /// Hits ClickHouse: `ENV=staging cargo test -p lyra-vaults --lib -- --ignored --nocapture`
    #[tokio::test]
    #[ignore]
    async fn fetches_momentum_from_clickhouse() {
        let client = clickhouse_client();
        let now = chrono::Utc::now().timestamp();
        let mom = fetch_momentum(&client, "ETH", now).await.unwrap();
        println!("{:?}", mom);
        assert_eq!(mom.hour, floor_to_hour(now));
        assert_eq!(mom.reference_hour, mom.hour - MOMENTUM_WINDOW_BARS as i64 * BAR_SEC);
        assert!(mom.spot > 0.0 && mom.reference_spot > 0.0);
        assert_eq!(mom.pct, momentum_pct(mom.spot, mom.reference_spot).unwrap());
    }

    /// Writes the hourly ETH spot series from `raw_spot_feed` to [CH_SPOT_PATH], for
    /// [matches_the_reference_on_clickhouse_spot] to run against offline. Regenerate with:
    ///
    ///   `ENV=staging cargo test -p lyra-vaults --lib -- --ignored \
    ///    generate_clickhouse_spot_series --nocapture`
    ///
    /// Fetched in chunks because a single multi-year aggregate exceeds the endpoint's own timeout.
    #[tokio::test]
    #[ignore]
    async fn generate_clickhouse_spot_series() {
        const CURRENCY: &str = "ETH";
        const CHUNK_BARS: i64 = 24 * 45;

        let client = clickhouse_client();
        // start where the feed does, end where the reference series ends
        let bounds: Vec<HourlySpot> = client
            .query(&format!(
                "SELECT toInt64(min(timestamp)) AS hour, toFloat64(max(timestamp)) AS spot \
                 FROM {SPOT_TABLE} WHERE currency = '{CURRENCY}'"
            ))
            .await
            .unwrap();
        let feed_from = floor_to_hour(bounds[0].hour);
        let feed_to = floor_to_hour(bounds[0].spot as i64);
        let reference_to =
            load_reference(MOMENTUM_WINDOW_BARS).map(|rows| rows[rows.len() - 1].hour);
        let to_hour = reference_to.unwrap_or(feed_to).min(feed_to);
        println!(
            "\n{CURRENCY} feed spans {} .. {}, fetching {} .. {}",
            fmt_hour(feed_from),
            fmt_hour(feed_to),
            fmt_hour(feed_from),
            fmt_hour(to_hour)
        );

        let mut rows: Vec<HourlySpot> = Vec::new();
        let mut chunk_from = feed_from;
        while chunk_from <= to_hour {
            let chunk_to = (chunk_from + (CHUNK_BARS - 1) * BAR_SEC).min(to_hour);
            let chunk = fetch_hourly_spot(&client, CURRENCY, chunk_from, chunk_to).await.unwrap();
            println!(
                "  {} .. {}: {} hours",
                fmt_hour(chunk_from),
                fmt_hour(chunk_to),
                chunk.len()
            );
            rows.extend(chunk);
            chunk_from = chunk_to + BAR_SEC;
        }

        rows.sort_by_key(|row| row.hour);
        rows.dedup_by_key(|row| row.hour);
        assert!(rows.len() > 10_000, "only got {} hours", rows.len());
        let spanned = ((rows[rows.len() - 1].hour - rows[0].hour) / BAR_SEC + 1) as usize;
        let gaps: Vec<(i64, i64)> = rows
            .windows(2)
            .filter(|pair| pair[1].hour - pair[0].hour != BAR_SEC)
            .map(|pair| (pair[0].hour, pair[1].hour))
            .collect();
        println!(
            "fetched {} hours over a {} hour span, {} missing, {} gaps",
            rows.len(),
            spanned,
            spanned - rows.len(),
            gaps.len()
        );
        for (from, to) in gaps.iter().take(20) {
            println!(
                "  gap: {} -> {} ({} hours missing)",
                fmt_hour(*from),
                fmt_hour(*to),
                (to - from) / BAR_SEC - 1
            );
        }

        // one row per line, so a regenerated fixture diffs readably
        let mut out = String::new();
        out.push_str("{\n");
        out.push_str(&format!(
            "  \"description\": {},\n",
            serde_json::json!(format!(
                "Hourly {CURRENCY} spot from {SPOT_TABLE}, the earliest observation of each hour \
                 (argMin(price, timestamp)). Test fixture for the momentum implementation; \
                 regenerate with the generate_clickhouse_spot_series test."
            ))
        ));
        out.push_str(&format!("  \"currency\": {},\n", serde_json::json!(CURRENCY)));
        out.push_str(&format!("  \"table\": {},\n", serde_json::json!(SPOT_TABLE)));
        out.push_str(&format!("  \"first_hour\": {},\n", rows[0].hour));
        out.push_str(&format!("  \"last_hour\": {},\n", rows[rows.len() - 1].hour));
        out.push_str(&format!("  \"hours\": {},\n", rows.len()));
        out.push_str("  \"series\": [\n");
        for (idx, row) in rows.iter().enumerate() {
            let comma = if idx + 1 == rows.len() { "" } else { "," };
            out.push_str(&format!("    {{\"hour\": {}, \"spot\": {}}}{}\n", row.hour, row.spot, comma));
        }
        out.push_str("  ]\n}\n");
        std::fs::write(CH_SPOT_PATH, out).unwrap();
        println!("wrote {CH_SPOT_PATH}");
    }
}
