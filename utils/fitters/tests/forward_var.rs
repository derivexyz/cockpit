use super::clock::{equity_clock, near};
use crate::fitters::clock::{UsEquityClock, VarClock};
use crate::fitters::forward_var::{
    project_total_vars, Event, FitFlag, FittedEvent, ForwardVarCurve, ForwardVarCurveFit, ForwardVarModel, FwdVarConfig,
};
use crate::SEC_PER_YEAR;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fmt::Debug;

// monday, 16 november 2026 at 10:00 eastern
const START: i64 = 1_794_841_200;
const HOUR: i64 = 3600;
const DAY: i64 = 86_400;

fn event(offset: i64, length_sec: i64) -> Event {
    Event {
        start_sec: START + offset,
        length_sec,
    }
}

fn hourly(values: &[f64]) -> BTreeMap<i64, f64> {
    (1..).zip(values).map(|(i, &v)| (START + i * HOUR, v)).collect()
}

fn fit(events: &[Event], values: &[f64]) -> Result<ForwardVarCurveFit> {
    let config = FwdVarConfig {
        clock: equity_clock(),
        events: events.to_vec(),
    };
    ForwardVarModel::new(config)?.fit(START, &hourly(values))
}

fn rebuilds(curve: &ForwardVarCurve, inputs: &BTreeMap<i64, f64>) -> Result<()> {
    for (&expiry, &total) in inputs {
        near(curve.get_total_var(curve.ref_sec, expiry)?, total);
    }
    Ok(())
}

#[test]
fn the_curve_rebuilds_its_expiries_and_adds_up_across_session_and_event_boundaries() -> Result<()> {
    let mut clock = equity_clock();
    clock.early_closes.insert(START / DAY + 2);
    clock.holidays.insert(START / DAY + 3);
    let config = FwdVarConfig {
        clock,
        events: vec![event(DAY + 7 * HOUR, 0), event(8 * DAY, 4 * HOUR)],
    };
    let close = START + 6 * HOUR;
    let inputs: BTreeMap<_, _> = [(1, 2.), (2, 24.), (4, 28.), (7, 34.), (9, 48.), (11, 52.), (14, 58.)]
        .into_iter()
        .map(|(d, v)| (close + d * DAY, v))
        .collect();
    let fit = ForwardVarModel::new(config)?.fit(START, &inputs)?;
    assert!(fit.flags.is_empty());
    let curve = fit.curve;
    assert_eq!(curve.events.len(), 2);
    assert!(curve.events.iter().all(|e| e.total_var > 0.));
    assert_eq!((curve.ref_sec, curve.end_sec), (START, close + 14 * DAY));
    rebuilds(&curve, &inputs)?;
    for split in [
        START,
        close,
        START + DAY + 7 * HOUR,
        close + 5 * DAY,
        START + 8 * DAY + HOUR,
        curve.end_sec,
    ] {
        near(
            curve.get_total_var(START, split)? + curve.get_total_var(split, curve.end_sec)?,
            58.,
        );
    }
    Ok(())
}

struct LunchBreak;

impl VarClock for LunchBreak {
    fn weights(&self, from: i64, to: i64) -> Result<Vec<(i64, f64)>> {
        anyhow::ensure!(from <= to, "reversed interval");
        if from == to {
            return Ok(Vec::new());
        }
        let initial = if from < 5 {
            1.
        } else if from < 8 {
            0.
        } else {
            2.
        };
        let mut pieces = vec![(from, initial)];
        pieces.extend([(5, 0.), (8, 2.)].into_iter().filter(|&(s, _)| from < s && s < to));
        Ok(pieces)
    }
}

#[test]
fn a_clock_with_a_lunch_break_fits_in_its_own_time() -> Result<()> {
    let model = ForwardVarModel::new(FwdVarConfig {
        clock: LunchBreak,
        events: vec![],
    })?;
    let inputs = [(10, 9.), (20, 49.)].into();
    let curve = model.fit(0, &inputs)?.curve;
    rebuilds(&curve, &inputs)?;
    near(curve.get_total_var(4, 9)?, 3.);
    for (t, rate) in [(4, 1.), (5, 0.), (7, 0.), (8, 2.), (9, 2.), (10, 4.)] {
        near(curve.get_forward_var(t)?, rate * SEC_PER_YEAR);
    }
    Ok(())
}

#[test]
fn the_nearest_clean_interval_lends_its_level_and_the_events_share_the_rest() -> Result<()> {
    let first = fit(&[event(1, 0)], &[4., 5., 15.])?.curve;
    near(first.events[0].total_var, 3.);
    // the later donor at 3 wins the tie with the earlier donor at 1
    let tie = fit(&[event(2 * HOUR, 0)], &[1., 10., 13.])?.curve;
    near(tie.events[0].total_var, 6.);
    let shared = fit(&[event(30, 0), event(HOUR, 0)], &[7., 8.])?.curve;
    assert_eq!(shared.events.len(), 2);
    for e in &shared.events {
        near(e.total_var, 3.);
    }
    rebuilds(&shared, &hourly(&[7., 8.]))
}

#[test]
fn jumps_belong_to_the_right_endpoint_and_have_no_rate() -> Result<()> {
    let curve = fit(&[event(60, 0)], &[3., 4.])?.curve;
    near(curve.get_total_var(START + 59, START + 60)?, 2. + 1. / 3600.);
    near(curve.get_total_var(START + 60, START + 61)?, 1. / 3600.);
    near(curve.get_total_var(START + 60, START + 60)?, 0.);
    near(curve.get_forward_var(START + 60)?, SEC_PER_YEAR / 3600.);
    let t = START + HOUR + 1;
    let stacked = fit(&[event(HOUR + 1, 0); 3], &[1., 1.01, 1.01])?.curve;
    assert_eq!(stacked.events.len(), 3);
    near(stacked.get_total_var(t - 1, t)?, 0.01);
    near(stacked.get_total_var(t, START + 2 * HOUR)?, 0.);
    Ok(())
}

#[test]
fn a_fade_releases_variance_at_a_rate_that_falls_linearly_to_zero() -> Result<()> {
    let curve = fit(&[event(100, 400)], &[5., 6.])?.curve;
    near(curve.events[0].total_var, 4.);
    near(curve.get_total_var(START + 100, START + 300)?, 200. / 3600. + 3.);
    near(curve.get_total_var(START + 300, START + HOUR)?, 3300. / 3600. + 1.);
    for (offset, extra) in [(100, 0.02), (200, 0.015), (400, 0.005), (500, 0.)] {
        near(
            curve.get_forward_var(START + offset)?,
            (1. / 3600. + extra) * SEC_PER_YEAR,
        );
    }
    let mut integral = 0.;
    for dt in (0..HOUR).step_by(2) {
        integral += 2. * curve.get_forward_var(START + dt + 1)? / SEC_PER_YEAR;
    }
    near(integral, 5.);
    near(curve.get_total_var(START, START + HOUR)?, integral);
    Ok(())
}

#[test]
fn a_fit_flags_each_event_problem_once_and_rejects_straddling_windows() -> Result<()> {
    for (events, values, flag) in [
        (
            vec![event(1, 0), event(HOUR + 1, 0)],
            vec![1., 2., 5.],
            FitFlag::NegativeLeftover,
        ),
        (
            vec![event(1, 0), event(HOUR + 1, 0)],
            vec![1., 2.],
            FitFlag::NoCleanDonor,
        ),
        (vec![event(HOUR + 1, 0); 2], vec![1.], FitFlag::EventDropped),
        (vec![event(10, 0)], vec![1., 2.], FitFlag::NegativeLeftover),
    ] {
        let fit = fit(&events, &values)?;
        assert_eq!(fit.flags, [flag]);
        assert!(fit.curve.events.is_empty());
        rebuilds(&fit.curve, &hourly(&values))?;
    }
    for e in [event(-1, 1), event(0, 0)] {
        let past = fit(&[e], &[1.])?;
        assert!(past.flags.is_empty() && past.curve.events.is_empty());
    }
    for e in [event(-1, 2), event(0, 1), event(HOUR, 1)] {
        assert!(fit(&[e], &[2., 3.]).is_err());
    }
    for e in [
        event(1, -1),
        Event {
            start_sec: i64::MAX,
            length_sec: 1,
        },
    ] {
        assert!(ForwardVarModel::new(FwdVarConfig {
            clock: equity_clock(),
            events: vec![e]
        })
        .is_err());
    }
    Ok(())
}

#[test]
fn an_event_on_a_flat_term_structure_is_flagged_whatever_the_rounding() -> Result<()> {
    // equal levels either side leave the event nothing but rounding noise, of either sign
    // (up to 15 weeks, so no interval holds a clock change)
    for weeks in 1..=5 {
        let config = FwdVarConfig {
            clock: equity_clock(),
            events: vec![event(weeks * 7 * DAY + 1, 0)],
        };
        let inputs: BTreeMap<_, _> = (1..=3).map(|i| (START + i * weeks * 7 * DAY, 0.25 * i as f64)).collect();
        let fit = ForwardVarModel::new(config)?.fit(START, &inputs)?;
        assert_eq!(fit.flags, [FitFlag::NegativeLeftover], "{weeks} weeks");
        assert!(fit.curve.events.is_empty(), "{weeks} weeks");
        rebuilds(&fit.curve, &inputs)?;
    }
    Ok(())
}

#[test]
fn decreasing_totals_name_the_expiry_and_their_projection_fits() -> Result<()> {
    let config = FwdVarConfig {
        clock: equity_clock(),
        events: vec![],
    };
    let model = ForwardVarModel::new(config.clone())?;
    assert_eq!(model.config(), &config);
    let inputs = hourly(&[1., 5., 4., 3., 7.]);
    let err = model.fit(START, &inputs).unwrap_err();
    assert!(err.to_string().contains(&(START + 3 * HOUR).to_string()));
    let projected = project_total_vars(&inputs);
    assert_eq!(projected, hourly(&[1., 4., 4., 4., 7.]));
    assert_eq!(project_total_vars(&projected), projected);
    let curve = model.fit(START, &projected)?.curve;
    rebuilds(&curve, &projected)?;
    near(curve.get_forward_var(START + 2 * HOUR)?, 0.);
    Ok(())
}

struct FixedPieces(Vec<(i64, f64)>);

impl VarClock for FixedPieces {
    fn weights(&self, _: i64, _: i64) -> Result<Vec<(i64, f64)>> {
        Ok(self.0.clone())
    }
}

#[test]
fn unusable_inputs_and_queries_outside_the_fitted_range_are_errors() -> Result<()> {
    let curve = fit(&[], &[1.])?.curve;
    for (a, b) in [(START - 1, START), (START, START + HOUR + 1), (START + 1, START)] {
        assert!(curve.get_total_var(a, b).is_err());
    }
    for t in [START - 1, START + HOUR] {
        assert!(curve.get_forward_var(t).is_err());
    }
    let model = ForwardVarModel::new(FwdVarConfig {
        clock: equity_clock(),
        events: vec![],
    })?;
    for inputs in [
        hourly(&[]),
        hourly(&[0.]),
        hourly(&[f64::NAN]),
        [(START, 1.)].into(),
        [(253_402_300_800, 1.)].into(),
    ] {
        assert!(model.fit(START, &inputs).is_err());
    }
    for pieces in [
        vec![],
        vec![(0, 0.)],
        vec![(0, f64::NAN)],
        vec![(0, f64::MAX)],
        vec![(0, f64::from_bits(1))],
        vec![(1, 1.)],
        vec![(0, 1.), (0, 2.)],
        vec![(0, 1.), (11, 2.)],
        vec![(0, -1.), (5, 3.)],
    ] {
        let model = ForwardVarModel::new(FwdVarConfig {
            clock: FixedPieces(pieces),
            events: vec![],
        })?;
        assert!(model.fit(0, &[(10, 1.)].into()).is_err());
    }
    // a level too small to hold: the curve could not give back its input
    let lost = ForwardVarModel::new(FwdVarConfig {
        clock: FixedPieces(vec![(0, 1e300)]),
        events: vec![],
    })?;
    assert!(lost.fit(0, &[(10, 1e-100)].into()).is_err());
    Ok(())
}

#[test]
fn a_stored_curve_answers_without_its_clock_or_config() -> Result<()> {
    let short: FwdVarConfig<UsEquityClock> =
        serde_json::from_str(r#"{"clock":{"night":0.4,"weekend":0.8,"half_day":0.5}}"#)?;
    assert_eq!(
        short,
        FwdVarConfig {
            clock: equity_clock(),
            events: vec![]
        }
    );
    assert_eq!(serde_json::from_str::<Event>(r#"{"start_sec":10}"#)?.length_sec, 0);
    let curve = fit(&[event(60, 0), event(200, 400)], &[5., 6.])?.curve;
    let json = serde_json::to_string(&curve)?;
    let stored: ForwardVarCurve = serde_json::from_str(&json)?;
    for t in [START, START + 60, START + 300, START + HOUR] {
        near(stored.get_forward_var(t)?, curve.get_forward_var(t)?);
        near(
            stored.get_total_var(t, curve.end_sec)?,
            curve.get_total_var(t, curve.end_sec)?,
        );
    }
    Ok(())
}

#[test]
fn both_queries_refuse_a_malformed_curve_even_outside_the_damaged_piece() {
    let valid = ForwardVarCurve {
        ref_sec: 0,
        end_sec: 10,
        rates: vec![(0, 1.), (5, 2.)],
        events: vec![],
    };
    let refused = |curve: ForwardVarCurve| {
        assert!(curve.get_total_var(0, 1).is_err(), "{curve:?}");
        assert!(curve.get_forward_var(0).is_err(), "{curve:?}");
    };
    for rates in [
        vec![],
        vec![(1, 1.)],
        vec![(0, 1.), (5, 2.), (2, 3.)],
        vec![(0, 1.), (0, 2.)],
        vec![(0, 1.), (10, 0.)],
        vec![(0, 1.), (20, 0.)],
        vec![(0, 1.), (5, -1.)],
        vec![(0, 1.), (5, f64::NAN)],
        vec![(0, 1.), (5, f64::INFINITY)],
    ] {
        refused(ForwardVarCurve { rates, ..valid.clone() });
    }
    for (start_sec, length_sec, total_var) in [
        (5, -1, 1.),
        (5, 1, -1.),
        (5, 0, f64::NAN),
        (5, 1, f64::INFINITY),
        (i64::MAX, 1, 1.),
        // outside the curve's own range
        (-5, 10, 1.),
        (8, 5, 1.),
        (0, 0, 1.),
    ] {
        refused(ForwardVarCurve {
            events: vec![FittedEvent {
                start_sec,
                length_sec,
                total_var,
            }],
            ..valid.clone()
        });
    }
}

#[test]
fn curves_shared_or_cloned_across_threads_answer_as_on_one() -> Result<()> {
    fn shareable<T: Send + Sync + Clone + Debug>() {}
    shareable::<ForwardVarModel<UsEquityClock>>();
    shareable::<FwdVarConfig<UsEquityClock>>();
    shareable::<ForwardVarCurve>();
    shareable::<ForwardVarCurveFit>();
    let curve = fit(&[event(100, 400)], &[5., 6.])?.curve;
    let query = |c: &ForwardVarCurve| -> Result<_> {
        Ok((
            c.get_total_var(START + 200, c.end_sec)?,
            c.get_forward_var(START + 200)?,
        ))
    };
    let expected = query(&curve)?;
    std::thread::scope(|scope| -> Result<()> {
        let shared = scope.spawn(|| query(&curve));
        let owned = curve.clone();
        let cloned = scope.spawn(move || query(&owned));
        for reader in [shared, cloned] {
            assert_eq!(reader.join().expect("reader finished")?, expected);
        }
        Ok(())
    })
}

/// Recorded from a reference implementation. Floating-point fields hold their bits.
#[derive(Deserialize)]
struct Recorded {
    night: u64,
    weekend: u64,
    half_day: u64,
    holidays: Vec<i64>,
    early_closes: Vec<i64>,
    events: Vec<(i64, i64)>,
    project: bool,
    ref_sec: i64,
    total_vars: Vec<(i64, u64)>,
    queries: Vec<(i64, i64, u64, u64)>,
}

#[test]
fn the_recorded_curves_are_reproduced_to_rounding() -> Result<()> {
    let cases: Vec<Recorded> = serde_json::from_str(include_str!("fixtures/forward_var.json"))?;
    assert_eq!(cases.len(), 48);
    assert!(cases.iter().filter(|c| c.project).count() >= 4);
    let mut flags = Vec::new();
    for (i, case) in cases.into_iter().enumerate() {
        let clock = UsEquityClock {
            night: f64::from_bits(case.night),
            weekend: f64::from_bits(case.weekend),
            half_day: f64::from_bits(case.half_day),
            holidays: case.holidays.into_iter().collect(),
            early_closes: case.early_closes.into_iter().collect(),
        };
        let events = case.events.into_iter().map(|(start_sec, length_sec)| Event { start_sec, length_sec }).collect();
        let inputs: BTreeMap<_, _> = case.total_vars.into_iter().map(|(t, v)| (t, f64::from_bits(v))).collect();
        let largest = inputs.values().copied().fold(0., f64::max);
        let inputs = if case.project {
            project_total_vars(&inputs)
        } else {
            inputs
        };
        let fit = ForwardVarModel::new(FwdVarConfig { clock, events })?.fit(case.ref_sec, &inputs)?;
        flags.extend(fit.flags);
        rebuilds(&fit.curve, &inputs)?;
        assert_eq!(case.queries.len(), 12);
        for (t, expiry, total, rate) in case.queries {
            let (total, rate) = (f64::from_bits(total), f64::from_bits(rate));
            let got = fit.curve.get_total_var(t, expiry)?;
            assert!(
                (got - total).abs() <= 1e-12 * largest,
                "case {i}, total over ({t}, {expiry}]: {got} vs {total}"
            );
            let got = fit.curve.get_forward_var(t)?;
            assert!(
                (got - rate).abs() <= 1e-9 * rate.abs(),
                "case {i}, rate at {t}: {got} vs {rate}"
            );
        }
    }
    for flag in [FitFlag::EventDropped, FitFlag::NegativeLeftover, FitFlag::NoCleanDonor] {
        assert!(flags.contains(&flag), "{flag:?}");
    }
    Ok(())
}
