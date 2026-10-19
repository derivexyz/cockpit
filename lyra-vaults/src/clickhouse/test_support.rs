//! Shared scaffolding for the signal comparison tests: loading the backtest reference vectors,
//! loading the ClickHouse fixtures, and reporting diffs before any assertion runs.

use crate::clickhouse::client::ClickhouseClient;
use crate::clickhouse::grid::{grid_gaps, BAR_SEC};

/// Hourly reference vectors shipped with the strategy spec.
pub const REFERENCE_PATH: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/signals/signals_timeseries.json");
/// Hourly spot from `raw_spot_feed`, written by the momentum generator test.
pub const CH_SPOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/signals/momentum_ch.json");
/// Hourly constant-maturity ATM vol from `raw_vol_feed`, written by the IV generator test.
pub const CH_IV_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/signals/iv_percentile_ch.json");

/// One hour of the backtest's reference series.
pub struct ReferenceRow {
    pub hour: i64,
    pub spot: f64,
    pub atm_iv: f64,
    pub mom: Option<f64>,
    pub iv_rank: Option<f64>,
    /// Within `window_bars` rows after a hole in the reference's own grid, where its positional
    /// rolling window spans more than `window_bars` hours and cannot agree with an hourly one.
    pub post_gap: bool,
}

/// Loads the reference series, marking the rows contaminated by its own grid gaps for a rolling
/// window of `window_bars`. Returns None (rather than failing) when the fixture is not checked out.
pub fn load_reference(window_bars: usize) -> Option<Vec<ReferenceRow>> {
    let Ok(file) = std::fs::read_to_string(REFERENCE_PATH) else {
        eprintln!("skipping: {REFERENCE_PATH} is not present");
        return None;
    };
    let vectors: serde_json::Value = serde_json::from_str(&file).unwrap();
    let series = vectors["series"].as_array().unwrap();
    let mut rows: Vec<ReferenceRow> = series
        .iter()
        .map(|row| ReferenceRow {
            hour: parse_hour(row["ts"].as_str().unwrap()),
            spot: row["spot_usd"].as_f64().unwrap(),
            atm_iv: row["atm_iv_pct"].as_f64().unwrap(),
            mom: row["mom_30d_pct"].as_f64(),
            iv_rank: row["iv_rank_pctile"].as_f64(),
            post_gap: false,
        })
        .collect();
    assert!(rows.len() > 30_000, "unexpected reference series length");

    let hours: Vec<i64> = rows.iter().map(|row| row.hour).collect();
    let gaps = grid_gaps(&hours);
    println!(
        "\nreference series: {} rows, {} .. {}, {} gaps",
        rows.len(),
        fmt_hour(rows[0].hour),
        fmt_hour(rows[rows.len() - 1].hour),
        gaps.len()
    );
    for (from, to) in &gaps {
        let idx = hours.iter().position(|hour| hour == from).unwrap();
        println!(
            "  gap: {} -> {} ({} hours missing), excluding the next {} rows",
            fmt_hour(*from),
            fmt_hour(*to),
            (to - from) / BAR_SEC - 1,
            window_bars
        );
        let end = (idx + 1 + window_bars).min(rows.len());
        rows[idx + 1..end].iter_mut().for_each(|row| row.post_gap = true);
    }
    Some(rows)
}

/// Loads a generated ClickHouse fixture as `(hour, value)` rows, reading `value_key` from each
/// entry of its `series`. None when the fixture has not been generated yet.
pub fn load_ch_fixture(path: &str, value_key: &str, regenerate_with: &str) -> Option<Vec<(i64, f64)>> {
    let Ok(file) = std::fs::read_to_string(path) else {
        eprintln!("skipping: {path} is not present, generate it with");
        eprintln!("  {regenerate_with}");
        return None;
    };
    let fixture: serde_json::Value = serde_json::from_str(&file).unwrap();
    let rows: Vec<(i64, f64)> = fixture["series"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| (row["hour"].as_i64().unwrap(), row[value_key].as_f64().unwrap()))
        .collect();
    assert!(!rows.is_empty(), "{path} has no rows");
    Some(rows)
}

/// (hour, computed, reference, diff) per compared hour.
pub type Diffs = Vec<(i64, f64, f64, f64)>;

pub struct Stats {
    pub compared: usize,
    pub max_abs: f64,
    pub mean_abs: f64,
    pub mean_signed: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
}

impl Stats {
    /// Share of compared hours whose absolute diff is at least `threshold`.
    pub fn share_over(&self, sorted_abs: &[f64], threshold: f64) -> f64 {
        let over = sorted_abs.iter().filter(|d| **d >= threshold).count();
        over as f64 / self.compared.max(1) as f64
    }
}

/// Prints the comparison tables and returns its summary statistics, plus the sorted absolute
/// diffs for quantile checks. Printing happens before any assertion so that a disagreement is
/// visible in full rather than aborting at row one.
pub fn report(label: &str, diffs: &Diffs, show: usize) -> (Stats, Vec<f64>) {
    println!("\n{label}: {} hours compared", diffs.len());
    println!("  {:<22} {:>14} {:>14} {:>14}", "hour", "computed", "reference", "diff");
    for (hour, ours, expected, diff) in diffs.iter().take(show) {
        println!("  {:<22} {:>14.6} {:>14.6} {:>14.6}", fmt_hour(*hour), ours, expected, diff);
    }

    let mut worst = diffs.clone();
    worst.sort_by(|a, b| b.3.abs().partial_cmp(&a.3.abs()).unwrap());
    println!("  worst {show} absolute diffs:");
    for (hour, ours, expected, diff) in worst.iter().take(show) {
        println!("  {:<22} {:>14.6} {:>14.6} {:>14.6}", fmt_hour(*hour), ours, expected, diff);
    }

    let denominator = diffs.len().max(1) as f64;
    let mut sorted_abs: Vec<f64> = diffs.iter().map(|(_, _, _, d)| d.abs()).collect();
    sorted_abs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let quantile = |q: f64| match sorted_abs.is_empty() {
        true => 0.0,
        false => sorted_abs[((sorted_abs.len() - 1) as f64 * q) as usize],
    };
    let stats = Stats {
        compared: diffs.len(),
        max_abs: worst.first().map_or(0.0, |w| w.3.abs()),
        mean_abs: sorted_abs.iter().sum::<f64>() / denominator,
        // a signed mean near zero says the diffs are noise; a systematic error (an off-by-one
        // bar, a unit slip) biases it even when the absolute diffs look small
        mean_signed: diffs.iter().map(|(_, _, _, d)| *d).sum::<f64>() / denominator,
        p50: quantile(0.50),
        p95: quantile(0.95),
        p99: quantile(0.99),
    };
    println!(
        "  max abs {:.9}, mean abs {:.9}, mean signed {:+.9}",
        stats.max_abs, stats.mean_abs, stats.mean_signed
    );
    println!("  abs diff p50 {:.9}, p95 {:.9}, p99 {:.9}", stats.p50, stats.p95, stats.p99);
    (stats, sorted_abs)
}

/// The longest run of hours in a sorted series with no gap longer than `max_gap_bars`.
pub fn longest_dense_run(hours: &[i64], max_gap_bars: i64) -> (i64, i64) {
    let mut best = (0i64, 0i64);
    let mut best_len = 0i64;
    let mut start = match hours.first() {
        Some(first) => *first,
        None => return best,
    };
    for pair in hours.windows(2) {
        if (pair[1] - pair[0]) / BAR_SEC > max_gap_bars {
            if pair[0] - start > best_len {
                best_len = pair[0] - start;
                best = (start, pair[0]);
            }
            start = pair[1];
        }
    }
    let last = *hours.last().unwrap();
    if last - start > best_len {
        best = (start, last);
    }
    best
}

pub fn parse_hour(ts: &str) -> i64 {
    chrono::DateTime::parse_from_rfc3339(ts).unwrap().timestamp()
}

pub fn fmt_hour(hour: i64) -> String {
    chrono::DateTime::from_timestamp(hour, 0).unwrap().format("%Y-%m-%dT%H:%MZ").to_string()
}

/// Loads the env files (without touching the logger, so several tests can run in one process)
/// and builds a client. Defaults to staging, where the ClickHouse keys live.
pub fn clickhouse_client() -> ClickhouseClient {
    let env = std::env::var("ENV").unwrap_or("staging".to_string());
    let _ = dotenv::from_filename(".env");
    let _ = dotenv::from_filename(format!(".env.constants.{env}"));
    let _ = dotenv::from_filename(format!(".env.keys.{env}"));
    ClickhouseClient::from_env().unwrap()
}

/// Writes a generated hourly fixture, one row per line so regenerations diff readably.
pub fn write_fixture(
    path: &str,
    description: &str,
    extra: &[(&str, String)],
    value_key: &str,
    rows: &[(i64, f64)],
) {
    let mut out = String::new();
    out.push_str("{\n");
    out.push_str(&format!("  \"description\": {},\n", serde_json::json!(description)));
    for (key, value) in extra {
        out.push_str(&format!("  {}: {},\n", serde_json::json!(key), value));
    }
    out.push_str(&format!("  \"first_hour\": {},\n", rows[0].0));
    out.push_str(&format!("  \"last_hour\": {},\n", rows[rows.len() - 1].0));
    out.push_str(&format!("  \"hours\": {},\n", rows.len()));
    out.push_str("  \"series\": [\n");
    for (idx, (hour, value)) in rows.iter().enumerate() {
        let comma = if idx + 1 == rows.len() { "" } else { "," };
        out.push_str(&format!("    {{\"hour\": {hour}, \"{value_key}\": {value}}}{comma}\n"));
    }
    out.push_str("  ]\n}\n");
    std::fs::write(path, out).unwrap();
    println!("wrote {path}");
}
