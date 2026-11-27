use crate::fitters::forward_var::{
    ClockWeights, Event, FitFlag, FittedCurve, ForwardVarModel, FwdVarConfig, NegativeForward, PostEvent,
    SessionCalendar,
};
use crate::SEC_PER_YEAR;
use anyhow::Result;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Monday 16 November 2026, 10:00 in New York.
const START: i64 = 1_794_841_200;
const H: i64 = 3_600;
const DAY: i64 = 86_400;
const SESSION: i64 = 23_400;
const CLOSE: i64 = START + 6 * H;

fn cfg() -> FwdVarConfig {
    FwdVarConfig {
        calendar: SessionCalendar::default(),
        weights: ClockWeights {
            night: 0.4,
            weekend: 0.8,
            half_day: 0.5,
            session_profile: None,
        },
        events: vec![],
        negative_forward: NegativeForward::Reject,
        post_event: None,
    }
}

fn jump(offset: i64) -> Event {
    Event {
        start_sec: START + offset,
        length_sec: 0,
    }
}

fn fade(offset: i64, length_sec: i64) -> Event {
    Event {
        start_sec: START + offset,
        length_sec,
    }
}

/// One expiry per hour from `START`, all inside Monday's session, where an hour weighs 1 / 6.5.
fn hourly(ws: &[f64]) -> BTreeMap<i64, f64> {
    (1_i64..).zip(ws).map(|(i, &w)| (START + i * H, w)).collect()
}

fn fit(events: &[Event], ws: &[f64]) -> Result<FittedCurve> {
    let mut cfg = cfg();
    cfg.events = events.to_vec();
    ForwardVarModel::new(cfg)?.fit(START, &hourly(ws))
}

fn near(got: f64, want: f64) {
    assert!((got - want).abs() <= 1e-12 * want.abs().max(1e-8), "{got} vs {want}")
}

fn rebuilds(curve: &FittedCurve) -> Result<()> {
    for (&expiry, &w) in curve.knots_used() {
        near(curve.get_total_var(curve.ref_sec(), expiry)?, w);
    }
    Ok(())
}

#[test]
fn a_session_weighs_one_and_closures_their_weights() -> Result<()> {
    let model = ForwardVarModel::new(cfg())?;
    near(model.clock_mass(CLOSE - SESSION, CLOSE)?, 1.0);
    near(model.clock_mass(CLOSE, CLOSE + DAY - SESSION)?, 0.4);
    near(model.clock_mass(CLOSE + 4 * DAY, CLOSE + 7 * DAY - SESSION)?, 0.8);
    // Friday's close to Wednesday's: a weekend, two nights, three sessions
    near(model.clock_mass(CLOSE + 4 * DAY, CLOSE + 9 * DAY)?, 4.6);
    // Monday 12:45 to Wednesday 11:07:30: half of Monday, Tuesday, a quarter of Wednesday, two nights
    near(
        model.clock_mass(START + 2 * H + 45 * 60, START + 2 * DAY + H + 450)?,
        2.55,
    );
    Ok(())
}

#[test]
fn a_holiday_adds_half_the_weekends_extra_and_a_half_day_weighs_its_own_mass() -> Result<()> {
    let mut holiday = cfg();
    holiday.calendar.holidays.insert(CLOSE / DAY + 1);
    near(
        ForwardVarModel::new(holiday)?.clock_mass(CLOSE, CLOSE + 2 * DAY - SESSION)?,
        0.6,
    );

    // a closure may run past a year: 399 closed dates from Monday's close to a Tuesday's open
    let mut long = cfg();
    long.calendar.holidays.extend(CLOSE / DAY + 1..CLOSE / DAY + 400);
    let long = ForwardVarModel::new(long)?;
    near(long.clock_mass(CLOSE, CLOSE + 400 * DAY - SESSION)?, 0.4 + 399. * 0.2);
    // spread evenly over its wall time: a day in the middle holds a day's share
    let share = DAY as f64 / (400 * DAY - SESSION) as f64;
    near(long.clock_mass(CLOSE + 200 * DAY, CLOSE + 201 * DAY)?, 80.2 * share);

    let mut early = cfg();
    early.calendar.early_closes.insert(CLOSE / DAY);
    near(
        ForwardVarModel::new(early)?.clock_mass(CLOSE - SESSION, CLOSE - 3 * H)?,
        0.5,
    );

    // Thanksgiving 2026: Wednesday's close to Monday's is the holiday closure, the Friday half-day, the
    // weekend and Monday
    let mut thanksgiving = cfg();
    thanksgiving.calendar.holidays.insert(CLOSE / DAY + 10);
    thanksgiving.calendar.early_closes.insert(CLOSE / DAY + 11);
    thanksgiving.weights.half_day = 3.5 / 6.5;
    near(
        ForwardVarModel::new(thanksgiving)?.clock_mass(CLOSE + 9 * DAY, CLOSE + 14 * DAY)?,
        0.6 + 3.5 / 6.5 + 0.8 + 1.0,
    );
    Ok(())
}

#[test]
fn the_open_follows_new_york_across_both_clock_changes() -> Result<()> {
    let model = ForwardVarModel::new(cfg())?;
    // 2026: Mondays 2 and 9 March open at 14:30 and 13:30 UTC, 26 October and 2 November at 13:30 and 14:30
    for open in [1_772_461_800, 1_773_063_000, 1_793_021_400, 1_793_629_800] {
        near(model.clock_mass(open, open + H)?, 1.0 / 6.5);
        near(model.clock_mass(open, open + SESSION)?, 1.0);
        assert!(
            model.clock_mass(open - H, open)? < 0.1,
            "the hour before the open is closed"
        );
    }
    Ok(())
}

#[test]
fn a_session_profile_front_loads_the_session_and_shrinks_onto_a_half_day() -> Result<()> {
    let mut cfg = cfg();
    cfg.weights.session_profile = Some(vec![(0.25, 0.5)]);
    cfg.calendar.early_closes.insert(CLOSE / DAY + 1);
    let model = ForwardVarModel::new(cfg)?;
    let open = CLOSE - SESSION;
    near(model.clock_mass(open, open + SESSION / 4)?, 0.5);
    near(model.clock_mass(open + SESSION / 4, CLOSE)?, 0.5);
    near(model.clock_mass(open, CLOSE)?, 1.0);
    // Tuesday closes at 13:00: the first quarter of its 3.5 hours holds half of the half-day's mass
    let tuesday = open + DAY;
    near(model.clock_mass(tuesday, tuesday + 12_600 / 4)?, 0.25);
    near(model.clock_mass(tuesday, tuesday + 12_600)?, 0.5);
    Ok(())
}

#[test]
fn bad_configs_are_refused() {
    let with = |change: fn(&mut FwdVarConfig)| {
        let mut cfg = cfg();
        change(&mut cfg);
        ForwardVarModel::new(cfg)
    };
    assert!(with(|_| ()).is_ok());
    assert!(with(|c| c.weights.night = 0.).is_err());
    assert!(with(|c| c.weights.night = f64::NAN).is_err());
    assert!(
        with(|c| c.weights.weekend = 0.3).is_err(),
        "a weekend lighter than a night"
    );
    assert!(with(|c| c.weights.half_day = 0.).is_err());
    assert!(with(|c| c.weights.session_profile = Some(vec![(0.5, 0.6), (0.4, 0.7)])).is_err());
    assert!(with(|c| c.weights.session_profile = Some(vec![(0.5, 1.1)])).is_err());
    assert!(with(|c| c.weights.session_profile = Some(vec![(0.3, 0.6), (0.5, 0.4)])).is_err());
    assert!(with(|c| c.events = vec![fade(0, -1)]).is_err());
    assert!(with(|c| c.post_event = Some(PostEvent {
        excess: -1.,
        half_life: 1.
    }))
    .is_err());
    assert!(with(|c| c.post_event = Some(PostEvent {
        excess: 1.,
        half_life: 0.
    }))
    .is_err());
}

#[test]
fn a_config_reads_from_json_with_its_optional_parts_left_out() -> Result<()> {
    let short: FwdVarConfig = serde_json::from_str(r#"{"weights": {"night": 0.4, "weekend": 0.8, "half_day": 0.5}}"#)?;
    assert_eq!(short, cfg());
    let mut full = cfg();
    full.calendar.holidays.insert(20_783);
    full.weights.session_profile = Some(vec![(0.25, 0.5)]);
    full.events = vec![jump(60), fade(H, 900)];
    full.negative_forward = NegativeForward::Project;
    full.post_event = Some(PostEvent {
        excess: 2.,
        half_life: 0.5,
    });
    let back: FwdVarConfig = serde_json::from_str(&serde_json::to_string(&full)?)?;
    assert_eq!(back, full);
    Ok(())
}

#[test]
fn the_fitted_term_structure_is_rebuilt() -> Result<()> {
    // expiries over two weeks of nights, weekends, a half-day and a holiday, with a jump, a fade, a
    // raised base after each and a session profile
    let mut cfg = cfg();
    cfg.calendar.early_closes.insert(CLOSE / DAY + 2);
    cfg.calendar.holidays.insert(CLOSE / DAY + 3);
    cfg.weights.session_profile = Some(vec![(0.2, 0.5), (0.5, 0.75), (0.8, 0.9)]);
    cfg.events = vec![jump(DAY + 7 * H), fade(8 * DAY, 4 * H)];
    cfg.post_event = Some(PostEvent {
        excess: 2.,
        half_life: 0.5,
    });
    // 0.002 a day, 0.02 more from the jump's expiry on and 0.01 more from the fade's
    let days = [
        (1, 0.002),
        (2, 0.024),
        (4, 0.028),
        (7, 0.034),
        (9, 0.048),
        (11, 0.052),
        (14, 0.058),
    ];
    let knots: BTreeMap<i64, f64> = days.iter().map(|&(d, w)| (CLOSE + d * DAY, w)).collect();
    let curve = ForwardVarModel::new(cfg)?.fit(START, &knots)?;
    assert!(curve.flags().is_empty(), "{:?}", curve.flags());
    assert!(
        curve.event_sizes().iter().all(|&size| size > 0.),
        "{:?}",
        curve.event_sizes()
    );
    assert_eq!(curve.knots_used(), &knots);
    rebuilds(&curve)?;
    // and it adds up across any split
    let (mid, last) = (CLOSE + 5 * DAY + 1_234, CLOSE + 14 * DAY);
    near(
        curve.get_total_var(START, mid)? + curve.get_total_var(mid, last)?,
        knots[&last],
    );
    Ok(())
}

#[test]
fn an_event_takes_what_the_nearest_clean_interval_leaves() -> Result<()> {
    // the second hour sets the base at 1; the first hour's other 3 is the event
    let first = fit(&[jump(1)], &[4., 5.])?;
    near(first.event_sizes()[0], 3.);
    rebuilds(&first)?;
    // clean intervals either side at 1 and 3: the later one lends its level
    let tie = fit(&[jump(2 * H)], &[1., 10., 13.])?;
    near(tie.event_sizes()[0], 6.);
    rebuilds(&tie)?;
    // two events in one interval can only be told as a sum, so they split it
    let shared = fit(&[jump(20), jump(H)], &[7., 8.])?;
    near(shared.event_sizes()[0], 3.);
    near(shared.event_sizes()[1], 3.);
    rebuilds(&shared)
}

#[test]
fn a_jump_is_in_the_total_but_not_in_the_rate() -> Result<()> {
    let curve = fit(&[jump(60)], &[3., 4.])?;
    near(curve.get_total_var(START + 59, START + 60)?, 2. + 1. / 3600.);
    near(curve.get_total_var(START + 60, START + 61)?, 1. / 3600.);
    near(curve.get_forward_var(START + 60)?, SEC_PER_YEAR / 3600.);
    // rounding across jumps at one instant must not leave a negative variance
    let stacked = fit(&[jump(H + 1), jump(H + 1), jump(H + 1)], &[1., 1.01, 1.01])?;
    assert!(stacked.get_total_var(START + 2 * H - 1, START + 2 * H)? >= 0.);
    Ok(())
}

#[test]
fn a_fading_event_releases_its_variance_linearly_to_zero() -> Result<()> {
    // base 1 an hour, and an event of 4 over 400 seconds from START + 100
    let curve = fit(&[fade(100, 400)], &[5., 6.])?;
    near(curve.event_sizes()[0], 4.);
    near(curve.get_total_var(START + 300, START + H)?, 3300. / 3600. + 1.);
    // the rate is the base plus 2 * 4 / 400 times the part still to fade
    near(
        curve.get_forward_var(START + 200)?,
        (1. / 3600. + 0.02 * 0.75) * SEC_PER_YEAR,
    );
    near(
        curve.get_forward_var(START + 400)?,
        (1. / 3600. + 0.02 * 0.25) * SEC_PER_YEAR,
    );
    // and it integrates to the total
    let mut integral = 0.;
    for dt in (0..3600).step_by(2) {
        integral += 2. * curve.get_forward_var(START + dt + 1)? / SEC_PER_YEAR;
    }
    near(integral, 5.);
    near(curve.get_total_var(START, START + H)?, integral);
    Ok(())
}

#[test]
fn the_base_stays_raised_after_an_event_and_decays() -> Result<()> {
    // total variance written in closed form: a base, and per event a lump plus an excess that halves
    let (level, lump, half_life) = (0.003, 0.008, 0.07);
    for excess in [0.5, 3., 12.] {
        for events in [vec![900], vec![900, 1500]] {
            let total = |t: i64| {
                let mut v = level * t as f64 / SESSION as f64;
                for &s in events.iter().filter(|&&s| s <= t) {
                    let age = (t - s) as f64 / SESSION as f64;
                    v += lump + level * excess * half_life / std::f64::consts::LN_2 * (1. - (-age / half_life).exp2());
                }
                v
            };
            let mut cfg = cfg();
            // an event before the fit time is ignored
            cfg.events = events.iter().copied().chain([-60]).map(jump).collect();
            cfg.post_event = Some(PostEvent { excess, half_life });
            let knots = [3000, 7000, 12000].map(|t| (START + t, total(t))).into();
            let curve = ForwardVarModel::new(cfg)?.fit(START, &knots)?;
            rebuilds(&curve)?;
            assert!(
                curve.base_levels().iter().all(|x| (x - level).abs() < 1e-10),
                "{:?}",
                curve.base_levels()
            );
            assert!(curve.event_sizes()[..events.len()].iter().all(|size| (size - lump).abs() < 1e-10));
            for (a, b) in [(0, 100), (1800, 2450), (4000, 5070), (6800, 8100)] {
                let want = total(b) - total(a);
                assert!((curve.get_total_var(START + a, START + b)? - want).abs() < 1e-12 * want);
                let slope = curve.get_total_var(START + b - 1, START + b + 1)? * SEC_PER_YEAR / 2.;
                assert!((curve.get_forward_var(START + b)? / slope - 1.).abs() < 1e-7);
            }
        }
    }
    Ok(())
}

#[test]
fn the_edge_cases_are_flagged() -> Result<()> {
    // the donor's base already exceeds the interval: the event is 0
    let clipped = fit(&[jump(1)], &[1., 4.])?;
    assert_eq!(clipped.flags(), [FitFlag::NegativeLeftover]);
    near(clipped.event_sizes()[0], 0.);
    rebuilds(&clipped)?;
    // one interval, and it holds the event
    let alone = fit(&[jump(1)], &[1.])?;
    assert_eq!(alone.flags(), [FitFlag::NoCleanDonor]);
    near(alone.event_sizes()[0], 0.);
    rebuilds(&alone)?;
    // an event after the last expiry
    let late = fit(&[jump(H + 1)], &[1.])?;
    assert_eq!(late.flags(), [FitFlag::EventDropped]);
    // an event before the fit time is not an edge case
    assert!(fit(&[jump(-1)], &[1.])?.flags().is_empty());
    // a window across an expiry or across the fit time cannot be split
    assert!(fit(&[fade(H, 1)], &[2., 3.]).is_err());
    assert!(fit(&[fade(-1, 2)], &[2., 3.]).is_err());
    Ok(())
}

#[test]
fn a_falling_term_structure_is_refused_or_projected() -> Result<()> {
    assert!(fit(&[], &[0.2, 0.1]).is_err());
    let flat = fit(&[], &[1., 1.])?;
    near(flat.base_levels()[1], 0.);

    let mut cfg = cfg();
    cfg.negative_forward = NegativeForward::Project;
    let curve = ForwardVarModel::new(cfg)?.fit(START, &hourly(&[1., 5., 4., 3., 7.]))?;
    assert_eq!(curve.flags(), [FitFlag::Projected]);
    for (got, want) in curve.knots_used().values().zip([1., 4., 4., 4., 7.]) {
        near(*got, want);
    }
    rebuilds(&curve)
}

#[test]
fn queries_outside_the_fit_and_unusable_knots_are_errors() -> Result<()> {
    let curve = fit(&[], &[1.])?;
    assert!(curve.get_total_var(START - 1, START).is_err());
    assert!(curve.get_total_var(START, START + H + 1).is_err());
    assert!(curve.get_total_var(START + 1, START).is_err());
    assert!(curve.get_forward_var(START + H).is_err(), "no rate at the last expiry");
    let model = ForwardVarModel::new(cfg())?;
    for knots in [
        hourly(&[]),
        hourly(&[0.]),
        hourly(&[f64::NAN]),
        BTreeMap::from([(START, 1.)]),
        // past the year 9999
        BTreeMap::from([(253_402_300_800, 1.)]),
    ] {
        assert!(model.fit(START, &knots).is_err());
    }
    Ok(())
}

/// A case of `fixtures/forward_var.json`: synthetic inputs and the reference implementation's values.
/// Floats are stored as their bits.
#[derive(Deserialize)]
struct Recorded {
    night: u64,
    weekend: u64,
    half_day: u64,
    session_profile: Option<Vec<(u64, u64)>>,
    holidays: Vec<i64>,
    early_closes: Vec<i64>,
    events: Vec<(i64, i64)>,
    project: bool,
    post_event: Option<(u64, u64)>,
    ref_sec: i64,
    knots: Vec<(i64, u64)>,
    /// (t, expiry, total variance, rate at t)
    queries: Vec<(i64, i64, u64, u64)>,
}

impl Recorded {
    fn model(&self) -> Result<ForwardVarModel> {
        let f = f64::from_bits;
        ForwardVarModel::new(FwdVarConfig {
            calendar: SessionCalendar {
                holidays: self.holidays.iter().copied().collect(),
                early_closes: self.early_closes.iter().copied().collect(),
            },
            weights: ClockWeights {
                night: f(self.night),
                weekend: f(self.weekend),
                half_day: f(self.half_day),
                session_profile: self.session_profile.as_ref().map(|ps| ps.iter().map(|p| (f(p.0), f(p.1))).collect()),
            },
            events: self.events.iter().map(|&(start_sec, length_sec)| Event { start_sec, length_sec }).collect(),
            negative_forward: if self.project {
                NegativeForward::Project
            } else {
                NegativeForward::Reject
            },
            post_event: self.post_event.map(|(excess, half_life)| PostEvent {
                excess: f(excess),
                half_life: f(half_life),
            }),
        })
    }

    fn knots(&self) -> BTreeMap<i64, f64> {
        self.knots.iter().map(|&(t, w)| (t, f64::from_bits(w))).collect()
    }
}

fn recorded() -> Vec<Recorded> {
    serde_json::from_str(include_str!("fixtures/forward_var.json")).expect("the fixture parses")
}

#[test]
fn recorded_curves_are_reproduced_to_the_bit() -> Result<()> {
    let cases = recorded();
    assert_eq!(cases.len(), 48);
    let mut flags = Vec::new();
    for case in &cases {
        let curve = case.model()?.fit(case.ref_sec, &case.knots())?;
        flags.extend_from_slice(curve.flags());
        rebuilds(&curve)?;
        for &(t, expiry, total, rate) in &case.queries {
            assert_eq!(
                curve.get_total_var(t, expiry)?.to_bits(),
                total,
                "total variance {t} -> {expiry}"
            );
            assert_eq!(curve.get_forward_var(t)?.to_bits(), rate, "rate at {t}");
        }
    }
    // the file reaches every edge case
    for flag in [
        FitFlag::Projected,
        FitFlag::EventDropped,
        FitFlag::NegativeLeftover,
        FitFlag::NoCleanDonor,
    ] {
        assert!(flags.contains(&flag), "{flag:?}");
    }
    Ok(())
}

#[test]
fn a_curve_refitted_from_its_ref_sec_and_knots_is_the_same_curve() -> Result<()> {
    // what a process that only receives the stored fit would hold
    for case in recorded() {
        let curve = case.model()?.fit(case.ref_sec, &case.knots())?;
        let again = case.model()?.fit(curve.ref_sec(), curve.knots_used())?;
        assert_eq!(again.knots_used(), curve.knots_used());
        assert_eq!(again.base_levels(), curve.base_levels());
        assert_eq!(again.event_sizes(), curve.event_sizes());
        assert!(
            !again.flags().contains(&FitFlag::Projected),
            "the stored knots need no projection"
        );
        for &(t, expiry, total, rate) in &case.queries {
            assert_eq!(again.get_total_var(t, expiry)?.to_bits(), total);
            assert_eq!(again.get_forward_var(t)?.to_bits(), rate);
        }
    }
    Ok(())
}

#[test]
fn curves_shared_or_cloned_across_threads_answer_as_on_one() -> Result<()> {
    let cases = recorded();
    let curves = cases
        .iter()
        .map(|case| case.model()?.fit(case.ref_sec, &case.knots()))
        .collect::<Result<Vec<_>>>()?;
    let queries: usize = cases.iter().map(|case| case.queries.len()).sum();
    std::thread::scope(|scope| -> Result<()> {
        let readers: Vec<_> = (0..4)
            .map(|k| {
                // two readers borrow the curves, two take clones of their own
                let own = (k % 2 == 0).then(|| curves.clone());
                let (cases, curves) = (&cases, &curves);
                scope.spawn(move || -> Result<usize> {
                    let mut checked = 0;
                    for (case, curve) in cases.iter().zip(own.as_ref().unwrap_or(curves)) {
                        for &(t, expiry, total, rate) in &case.queries {
                            assert_eq!(curve.get_total_var(t, expiry)?.to_bits(), total);
                            assert_eq!(curve.get_forward_var(t)?.to_bits(), rate);
                            checked += 1;
                        }
                    }
                    Ok(checked)
                })
            })
            .collect();
        for reader in readers {
            assert_eq!(reader.join().expect("the reader finished")?, queries);
        }
        Ok(())
    })
}
