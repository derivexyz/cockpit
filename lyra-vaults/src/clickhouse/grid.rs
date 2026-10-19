//! The hourly grid every signal in the family is calibrated on.
//!
//! Both the momentum and the IV-rank signals are defined as bar arithmetic on a gap-free hourly
//! series, so they share the same flooring and forward-fill rules.

use anyhow::{bail, Result};

/// Seconds per bar.
pub const BAR_SEC: i64 = 3600;

/// Floors a timestamp to the start of its hour on the grid.
pub fn floor_to_hour(timestamp: i64) -> i64 {
    timestamp.div_euclid(BAR_SEC) * BAR_SEC
}

/// Expands sparse `(hour, value)` observations onto every hour of `[from_hour, to_hour]`, carrying
/// the last known value into hours that have none. Observations before `from_hour` are used only
/// to seed that carry; observations after `to_hour` are ignored.
///
/// Fails if nothing is known at or before `from_hour`, which is the case a forward fill cannot
/// honestly cover — the caller is asking about a window that starts before the data does.
pub fn forward_filled_grid(
    rows: &[(i64, f64)],
    from_hour: i64,
    to_hour: i64,
) -> Result<Vec<f64>> {
    let from_hour = floor_to_hour(from_hour);
    let to_hour = floor_to_hour(to_hour);
    if from_hour > to_hour {
        bail!("grid from {} is after to {}", from_hour, to_hour);
    }

    let bars = ((to_hour - from_hour) / BAR_SEC + 1) as usize;
    let mut in_window: Vec<Option<f64>> = vec![None; bars];
    let mut seed: Option<(i64, f64)> = None;
    for (hour, value) in rows {
        if !value.is_finite() || *value <= 0.0 {
            bail!("value {} at hour {} is not usable", value, hour);
        }
        let hour = floor_to_hour(*hour);
        if hour < from_hour {
            // keep the latest observation before the window as the initial carry
            if seed.map_or(true, |(seeded, _)| seeded < hour) {
                seed = Some((hour, *value));
            }
        } else if hour <= to_hour {
            in_window[((hour - from_hour) / BAR_SEC) as usize] = Some(*value);
        }
    }

    let mut grid = Vec::with_capacity(bars);
    let mut carry = seed.map(|(_, value)| value);
    for (bar, observed) in in_window.iter().enumerate() {
        if let Some(value) = observed {
            carry = Some(*value);
        }
        match carry {
            Some(value) => grid.push(value),
            None => bail!("no observation at or before hour {}", from_hour + bar as i64 * BAR_SEC),
        }
    }
    Ok(grid)
}

/// The gaps in a sorted hourly series, as `(hour before, hour after)` pairs.
pub fn grid_gaps(hours: &[i64]) -> Vec<(i64, i64)> {
    hours
        .windows(2)
        .filter(|pair| pair[1] - pair[0] != BAR_SEC)
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hours_floor_to_the_grid() {
        assert_eq!(floor_to_hour(1784156400), 1784156400);
        assert_eq!(floor_to_hour(1784156400 + 59 * 60), 1784156400);
        assert_eq!(floor_to_hour(1784156400 + BAR_SEC), 1784156400 + BAR_SEC);
        // and before the epoch, where a truncating division would round the wrong way
        assert_eq!(floor_to_hour(-1), -BAR_SEC);
    }

    #[test]
    fn gaps_carry_the_last_known_value() {
        let h = 1784156400;
        let rows = vec![(h, 100.0), (h + 2 * BAR_SEC, 120.0)];
        let grid = forward_filled_grid(&rows, h, h + 3 * BAR_SEC).unwrap();
        // hour 1 has no observation and carries 100, hour 3 carries 120
        assert_eq!(grid, vec![100.0, 100.0, 120.0, 120.0]);
    }

    #[test]
    fn out_of_window_rows_only_seed_the_carry() {
        let h = 1784156400;
        let rows = vec![
            (h - 5 * BAR_SEC, 90.0),
            (h - BAR_SEC, 95.0), // the latest before the window: the seed
            (h + BAR_SEC, 105.0),
            (h + 9 * BAR_SEC, 999.0), // after the window: ignored
        ];
        let grid = forward_filled_grid(&rows, h, h + 2 * BAR_SEC).unwrap();
        assert_eq!(grid, vec![95.0, 105.0, 105.0]);
    }

    #[test]
    fn unsorted_and_sub_hour_rows_land_on_their_bars() {
        let h = 1784156400;
        let rows = vec![(h + BAR_SEC + 1799, 105.0), (h + 42, 100.0)];
        let grid = forward_filled_grid(&rows, h, h + BAR_SEC).unwrap();
        assert_eq!(grid, vec![100.0, 105.0]);
    }

    #[test]
    fn a_window_starting_before_the_data_is_an_error() {
        let h = 1784156400;
        let err = forward_filled_grid(&[(h + BAR_SEC, 100.0)], h, h + BAR_SEC).unwrap_err();
        assert!(err.to_string().contains("no observation at or before"));

        assert!(forward_filled_grid(&[], h, h).is_err());
        assert!(forward_filled_grid(&[(h, 0.0)], h, h).is_err());
        assert!(forward_filled_grid(&[(h, 100.0)], h + BAR_SEC, h).is_err());
    }

    #[test]
    fn gaps_are_reported_as_pairs() {
        let h = 1784156400;
        let hours = vec![h, h + BAR_SEC, h + 5 * BAR_SEC, h + 6 * BAR_SEC];
        assert_eq!(grid_gaps(&hours), vec![(h + BAR_SEC, h + 5 * BAR_SEC)]);
        assert!(grid_gaps(&[h]).is_empty());
        assert!(grid_gaps(&[]).is_empty());
    }
}
