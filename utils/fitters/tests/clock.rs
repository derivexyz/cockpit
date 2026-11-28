use crate::fitters::clock::{UsEquityClock, VarClock};
use anyhow::Result;

const DAY: i64 = 86_400;
const SESSION: i64 = 23_400;
// monday, 16 november 2026 at 16:00 eastern
const CLOSE: i64 = 1_794_862_800;

pub(super) fn equity_clock() -> UsEquityClock {
    UsEquityClock {
        night: 0.4,
        weekend: 0.8,
        half_day: 0.5,
        holidays: Default::default(),
        early_closes: Default::default(),
    }
}

fn mass(clock: &UsEquityClock, from: i64, to: i64) -> Result<f64> {
    let pieces = clock.weights(from, to)?;
    assert_eq!(pieces.first().map(|p| p.0), Some(from));
    let mut sum = 0.;
    for (&(start, weight), end) in pieces.iter().zip(pieces.iter().skip(1).map(|p| p.0).chain([to])) {
        assert!(end > start && weight.is_finite() && weight > 0.);
        sum += (end - start) as f64 * weight;
    }
    Ok(sum)
}

pub(super) fn near(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-12 * want.abs(), "{got} vs {want}");
}

#[test]
fn a_session_weighs_one_and_closures_their_weights() -> Result<()> {
    let clock = equity_clock();
    near(mass(&clock, CLOSE - SESSION, CLOSE)?, 1.);
    near(mass(&clock, CLOSE, CLOSE + DAY - SESSION)?, 0.4);
    near(mass(&clock, CLOSE + 4 * DAY, CLOSE + 7 * DAY - SESSION)?, 0.8);
    near(mass(&clock, CLOSE + 4 * DAY, CLOSE + 9 * DAY)?, 4.6);
    Ok(())
}

#[test]
fn a_holiday_extends_the_closure_and_an_early_close_shortens_the_session() -> Result<()> {
    let mut clock = equity_clock();
    let monday = CLOSE / DAY;
    clock.holidays.extend([monday + 1, monday + 5]);
    clock.early_closes.extend([monday, monday + 1, monday + 6]);
    let early = CLOSE - 3 * 3600;
    near(mass(&clock, CLOSE - SESSION, early)?, 0.5);
    near(mass(&clock, early, CLOSE + 2 * DAY - SESSION)?, 0.6);
    near(mass(&clock, CLOSE + 4 * DAY, CLOSE + 7 * DAY - SESSION)?, 0.8);

    let mut long = equity_clock();
    long.holidays.extend(monday + 1..monday + 400);
    near(mass(&long, CLOSE, CLOSE + 400 * DAY - SESSION)?, 80.2);
    let share = DAY as f64 / (400 * DAY - SESSION) as f64;
    near(mass(&long, CLOSE + 200 * DAY, CLOSE + 201 * DAY)?, 80.2 * share);
    Ok(())
}

#[test]
fn the_open_follows_new_york_across_both_clock_changes() -> Result<()> {
    let clock = equity_clock();
    for open in [
        // 2026: march 2 and 9, october 26 and november 2
        1_772_461_800,
        1_773_063_000,
        1_793_021_400,
        1_793_629_800,
        // 2027: march 8 and 15, november 1 and 8
        1_804_516_200,
        1_805_117_400,
        1_825_075_800,
        1_825_684_200,
    ] {
        near(mass(&clock, open, open + SESSION)?, 1.);
        near(mass(&clock, open, open + 1)?, 1. / SESSION as f64);
        assert!(mass(&clock, open - 1, open)? < 1. / SESSION as f64);
    }
    Ok(())
}

#[test]
fn cutting_a_session_or_a_closure_preserves_its_mass() -> Result<()> {
    let clock = equity_clock();
    let (from, to) = (CLOSE - 1000, CLOSE + 3 * DAY + 2000);
    let full = mass(&clock, from, to)?;
    for mid in [CLOSE - 500, CLOSE + 500] {
        near(mass(&clock, from, mid)? + mass(&clock, mid, to)?, full);
    }
    assert!(clock.weights(from, from)?.is_empty());
    Ok(())
}

#[test]
fn the_calendar_defaults_to_empty_and_invalid_weights_or_times_are_refused() -> Result<()> {
    let clock: UsEquityClock = serde_json::from_str(r#"{"night":0.4,"weekend":0.8,"half_day":0.5}"#)?;
    assert_eq!(clock, equity_clock());
    for bad in [-1., 0., f64::NAN, f64::INFINITY] {
        assert!(UsEquityClock {
            night: bad,
            ..clock.clone()
        }
        .weights(0, 1)
        .is_err());
        assert!(UsEquityClock {
            half_day: bad,
            ..clock.clone()
        }
        .weights(0, 1)
        .is_err());
    }
    for weekend in [0.3, f64::NAN, f64::INFINITY] {
        assert!(UsEquityClock {
            weekend,
            ..clock.clone()
        }
        .weights(0, 1)
        .is_err());
    }
    const LAST: i64 = 253_402_300_799;
    for (a, b) in [(1, 0), (-1, 0), (-1, -1), (LAST, LAST + 1), (i64::MIN, i64::MAX)] {
        assert!(clock.weights(a, b).is_err());
    }
    for (a, b) in [(0, 1), (LAST - 1, LAST)] {
        assert!(mass(&clock, a, b)? > 0.);
    }
    Ok(())
}
