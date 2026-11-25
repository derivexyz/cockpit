use super::{expiry, REF_SEC};
use crate::black76::OptionContract;
use crate::fitters::slice_fitter::{default_cost_cfg, JWFit, JWFitCostConfig};
use crate::fitters::surface_fitter::{SliceStart, SliceStatus, SurfaceFitter};
use crate::fitters::svi::SVIJWParams;
use crate::SEC_PER_YEAR;
use ndarray::Array1;
use std::collections::{HashMap, HashSet};

#[test]
fn bootstrap_slices_interpolates_a_slice_too_thin_to_bootstrap() {
    // one thinly quoted expiry still gets a smile, from its neighbours, instead of failing the surface
    let quotes =
        Quotes::default()
            .on(&SMILE, 7, strikes())
            .on(&SMILE, 14, [99.0, 100.0, 101.0])
            .on(&SMILE, 30, strikes());
    let (bids, asks, strikes, expiries, is_calls) = quotes.into_arrays();
    let surface = Quotes::get_fitter_on(&expiries)
        .bootstrap_slices(bids, asks, strikes, expiries, is_calls)
        .expect("bootstrap a surface with one thin slice");

    let [near, thin, far] = [7, 14, 30].map(expiry);
    let fit = |expiry: i64| surface.slices()[&expiry].init_fit();
    let neighbours = HashMap::from([(near, fit(near)), (far, fit(far))]);
    let expected = JWFit::get_interpolated_at(&neighbours, thin, REF_SEC).expect("thin lies between its neighbours");
    let actual = fit(thin);
    for (name, a, e) in [
        ("v", actual.v, expected.v),
        ("psi", actual.psi, expected.psi),
        ("p", actual.p, expected.p),
        ("c", actual.c, expected.c),
        ("v_diff", actual.v_diff, expected.v_diff),
    ] {
        assert!((a - e).abs() < 1e-12, "{name}: {a} != {e}");
    }
}

const FWD: f64 = 100.0;
const SMILE: JWFit = JWFit {
    v: 0.04,
    psi: -0.3,
    p: 1.0,
    c: 0.4,
    v_diff: 0.01,
};
const HALF_SPREAD: f64 = 0.005;

/// The default cost config, with a stricter bar for a fit to be part of an interpolated start.
fn cost_cfg() -> JWFitCostConfig {
    JWFitCostConfig {
        interpolation_max_liq: 0.1,
        ..default_cost_cfg()
    }
}

#[derive(Default)]
struct Quotes {
    bids: Vec<f64>,
    asks: Vec<f64>,
    strikes: Vec<f64>,
    expiries: Vec<i64>,
    is_calls: Vec<bool>,
}

impl Quotes {
    /// Out-of-the-money quotes on `smile` at `strikes`, `HALF_SPREAD` of vol either side of it.
    fn on(self, smile: &JWFit, days: i64, strikes: impl IntoIterator<Item = f64>) -> Self {
        self.on_with_spread(smile, days, strikes, HALF_SPREAD)
    }

    /// Out-of-the-money quotes on `smile` at `strikes`, `half_spread` of vol either side of it.
    fn on_with_spread(
        mut self,
        smile: &JWFit,
        days: i64,
        strikes: impl IntoIterator<Item = f64>,
        half_spread: f64,
    ) -> Self {
        let expiry_sec = (expiry(days) - REF_SEC) as f64;
        let raw = smile.to_svi_jw(FWD, expiry_sec / SEC_PER_YEAR).to_raw();
        for strike in strikes {
            let contract = OptionContract {
                strike,
                expiry_sec,
                is_call: strike >= FWD,
            };
            let vol = raw.get_vol(strike);
            self.bids.push(contract.price(FWD, vol - half_spread));
            self.asks.push(contract.price(FWD, vol + half_spread));
            self.strikes.push(strike);
            self.expiries.push(expiry(days));
            self.is_calls.push(contract.is_call);
        }
        self
    }

    fn get_surface(self) -> SurfaceFitter {
        let (bids, asks, strikes, expiries, is_calls) = self.into_arrays();
        Self::get_fitter_on(&expiries)
            .with_quotes(bids, asks, strikes, expiries, is_calls)
            .expect("a forward and a discount for every quoted expiry")
    }

    /// The surface `build_slices` makes, its slices built on `inits`.
    fn get_surface_built_on(self, inits: HashMap<i64, SVIJWParams>) -> SurfaceFitter {
        let (bids, asks, strikes, expiries, is_calls) = self.into_arrays();
        Self::get_fitter_on(&expiries)
            .build_slices(inits, bids, asks, strikes, expiries, is_calls)
            .expect("a forward, a discount and an init for every quoted expiry")
    }

    fn get_fitter_on(expiries: &Array1<i64>) -> SurfaceFitter {
        let expiries: HashSet<i64> = expiries.iter().copied().collect();
        let forwards: HashMap<i64, f64> = expiries.iter().map(|&expiry| (expiry, FWD)).collect();
        let discounts: HashMap<i64, f64> = expiries.iter().map(|&expiry| (expiry, 1.0)).collect();
        SurfaceFitter::new(REF_SEC, forwards, discounts, cost_cfg())
    }

    #[allow(clippy::type_complexity)]
    fn into_arrays(self) -> (Array1<f64>, Array1<f64>, Array1<f64>, Array1<i64>, Array1<bool>) {
        let Self {
            bids,
            asks,
            strikes,
            expiries,
            is_calls,
        } = self;
        (
            Array1::from(bids),
            Array1::from(asks),
            Array1::from(strikes),
            Array1::from(expiries),
            Array1::from(is_calls),
        )
    }
}

fn strikes() -> impl Iterator<Item = f64> {
    (0..15).map(|i| 86.0 + 2.0 * i as f64)
}

fn svi(smile: &JWFit, days: i64) -> SVIJWParams {
    smile.to_svi_jw(FWD, (expiry(days) - REF_SEC) as f64 / SEC_PER_YEAR)
}

#[test]
fn a_slice_that_fails_leaves_the_others_fitted() {
    // one bad slice used to fail the whole surface, leaving a market maker no smile to quote off
    let surface = Quotes::default()
        .on(&SMILE, 30, strikes())
        .on(&SMILE, 60, strikes())
        .on(&SMILE, 120, strikes())
        .get_surface();
    let ran_off = JWFit { v: 1e5, ..SMILE };
    let starts = HashMap::from([
        (expiry(30), SliceStart::Init(svi(&ran_off, 30))),
        (expiry(60), SliceStart::Init(svi(&SMILE, 60))),
        (expiry(120), SliceStart::Bootstrap(vec![])),
    ]);
    let outcomes = surface.fit_multithread(&starts, 100).expect("every start is quoted");

    let failed_refit = &outcomes[&expiry(30)];
    let ran_off_error = |status: &SliceStatus| matches!(status, SliceStatus::Failed(error) if error.contains("ran off"));
    assert!(ran_off_error(&failed_refit.status), "{:?}", failed_refit.status);
    let (init, _) = failed_refit.fit.as_ref().expect("a failed refit still has its init to quote off");
    assert!((init.v / ran_off.v - 1.0).abs() < 1e-9, "init v {}", init.v);
    assert_eq!(outcomes[&expiry(60)].status, SliceStatus::Refit);
    assert_eq!(outcomes[&expiry(120)].status, SliceStatus::Bootstrapped);
}

#[test]
fn a_held_slice_is_priced_on_its_quotes_not_refitted() {
    // callers weigh a refit or reseed against the hold's cost; priced on no quotes it would cost nothing
    let surface = Quotes::default().on(&SMILE, 30, strikes()).get_surface();
    let stale = JWFit { v: 0.09, ..SMILE };
    let starts = HashMap::from([(expiry(30), SliceStart::Hold(svi(&stale, 30)))]);
    let held = surface
        .fit_multithread(&starts, 100)
        .expect("the start is quoted")
        .remove(&expiry(30))
        .expect("an outcome");

    assert_eq!(held.status, SliceStatus::Held);
    let (fit, bbo_cost) = held.fit.expect("a hold");
    assert!((fit.v - stale.v).abs() < 1e-12, "held at v {}", fit.v);
    assert!(bbo_cost > 0.0, "a 30% hold against a 20% book misses it");
}

#[test]
fn hold_outliers_holds_an_illiquid_slice_and_drops_an_outlier_from_a_copy() {
    let surface = Quotes::default()
        .on(&SMILE, 30, strikes())
        .on(&JWFit { v: 0.09, ..SMILE }, 30, [111.0])
        .on(&SMILE, 60, [99.0, 100.0, 101.0])
        .get_surface();
    let inits = HashMap::from([(expiry(30), svi(&SMILE, 30)), (expiry(60), svi(&SMILE, 60))]);
    let (cleaned, held) = surface.hold_outliers(&inits, 1, 3.0, 1.0).expect("valid outlier settings");

    let quotes = |surface: &SurfaceFitter, days: i64| {
        surface.slices()[&expiry(days)].bbo_residual_costs_from_fit(&SMILE, false).expect("residuals").len()
    };
    assert_eq!(held, HashSet::from([expiry(60)]), "three quotes are too few to fit");
    assert_eq!(
        quotes(&cleaned, 60),
        3,
        "a held slice keeps its quotes, to price its hold on"
    );
    assert_eq!(
        quotes(&cleaned, 30),
        15,
        "the quote 10 vol points off the smile is dropped"
    );
    // a reseed's bootstrap fits from the quotes alone, so it should see the quotes the init disowns
    assert_eq!(quotes(&surface, 30), 16, "the original keeps every quote");
}

#[test]
fn a_start_with_no_forward_is_an_error() {
    // fitting nothing for it would pass a caller's bug off as a failed slice
    let surface = Quotes::default().on(&SMILE, 30, strikes()).get_surface();
    let starts = HashMap::from([(expiry(60), SliceStart::Bootstrap(vec![]))]);
    assert!(surface.fit_multithread(&starts, 100).is_err());
}

#[test]
fn a_slice_with_no_quotes_is_held_on_an_empty_book() {
    // books empty at a daily halt, and a market maker keeps quoting the smile it held
    let (bids, asks, strikes, expiries, is_calls) = Quotes::default().on(&SMILE, 30, strikes()).into_arrays();
    let both = HashMap::from([(expiry(30), FWD), (expiry(60), FWD)]);
    let surface = SurfaceFitter::new(
        REF_SEC,
        both.clone(),
        both.iter().map(|(&expiry, _)| (expiry, 1.0)).collect(),
        cost_cfg(),
    )
    .with_quotes(bids, asks, strikes, expiries, is_calls)
    .expect("a forward and a discount for every quoted expiry");
    let start = |start: SliceStart| {
        surface
            .fit_multithread(&HashMap::from([(expiry(60), start)]), 100)
            .expect("a forward")
            .remove(&expiry(60))
    };
    let held = start(SliceStart::Hold(svi(&SMILE, 60))).expect("an outcome");
    assert_eq!(held.status, SliceStatus::Held);
    let (fit, bbo_cost) = held.fit.expect("the held smile");
    assert_eq!((fit.v, bbo_cost), (SMILE.v, 0.0));
    let bootstrap = start(SliceStart::Bootstrap(vec![])).expect("an outcome");
    assert!(
        matches!(bootstrap.status, SliceStatus::Failed(_)),
        "nothing to bootstrap from: {:?}",
        bootstrap.status
    );
}

#[test]
fn an_init_the_fitter_cannot_start_from_comes_back_failed() {
    // the fitter asserts on it, and a panic aborts a process built with panic = "abort", every slice with it
    let surface = Quotes::default().on(&SMILE, 30, strikes()).on(&SMILE, 60, strikes()).get_surface();
    let flat_put_wing = JWFit { p: 0.0, ..SMILE };
    let starts = HashMap::from([
        (expiry(30), SliceStart::Init(svi(&flat_put_wing, 30))),
        (expiry(60), SliceStart::Init(svi(&SMILE, 60))),
    ]);
    let outcomes = surface.fit_multithread(&starts, 100).expect("every start is quoted");

    let unusable = &outcomes[&expiry(30)].status;
    assert!(
        matches!(unusable, SliceStatus::Failed(error) if error.contains("must be positive")),
        "{unusable:?}"
    );
    assert_eq!(outcomes[&expiry(60)].status, SliceStatus::Refit);
}

#[test]
fn a_bootstrap_leaves_the_init_its_slice_was_built_on() {
    // a reseed bootstraps because the market left the init; regularized towards it, it would land between
    let stale = JWFit { v: 0.09, ..SMILE };
    let surface = Quotes::default()
        .on(&SMILE, 30, strikes())
        .get_surface_built_on(HashMap::from([(expiry(30), svi(&stale, 30))]));
    let atm_vol = |start: SliceStart| {
        let mut outcomes =
            surface.fit_multithread(&HashMap::from([(expiry(30), start)]), 100).expect("the start is quoted");
        let (fit, _) = outcomes.remove(&expiry(30)).and_then(|outcome| outcome.fit).expect("a fit");
        fit.v.sqrt()
    };
    let market = SMILE.v.sqrt();
    let bootstrapped = atm_vol(SliceStart::Bootstrap(vec![]));
    assert!(
        (bootstrapped - market).abs() < HALF_SPREAD,
        "bootstrapped at {bootstrapped:.4}"
    );
    let refit = atm_vol(SliceStart::Init(svi(&stale, 30)));
    assert!(
        refit > market + HALF_SPREAD,
        "regularized towards a 30% init, refit at {refit:.4}"
    );
}

#[test]
fn a_bootstrap_too_wide_to_pin_a_smile_takes_the_interpolated_start() {
    // a wide book takes its shape from the slices whose books pin one, not from other wide ones
    let other = JWFit {
        v: 0.09,
        p: 0.2,
        c: 0.2,
        ..SMILE
    };
    // 2.5 vol points either side reads starts off the quotes, but every smile inside costs nothing: a tie it wins
    for wide in [0.025, 0.15] {
        let surface = Quotes::default()
            .on(&SMILE, 7, strikes())
            .on_with_spread(&SMILE, 14, strikes(), wide)
            .on_with_spread(&other, 21, strikes(), 0.15)
            .on(&SMILE, 30, strikes())
            .get_surface();
        let starts = HashMap::from([
            (expiry(7), SliceStart::Init(svi(&SMILE, 7))),
            (expiry(14), SliceStart::Bootstrap(vec![])),
            (expiry(21), SliceStart::Init(svi(&other, 21))),
            (expiry(30), SliceStart::Init(svi(&SMILE, 30))),
        ]);
        let outcomes = surface.fit_multithread(&starts, 100).expect("every start has a forward");

        let wide_slice = &outcomes[&expiry(14)];
        assert_eq!(wide_slice.status, SliceStatus::Bootstrapped, "half spread {wide}");
        let (fit, _) = wide_slice.fit.as_ref().expect("an interpolated smile");
        for (name, got, want) in [("v", fit.v, SMILE.v), ("p", fit.p, SMILE.p), ("c", fit.c, SMILE.c)] {
            assert!(
                (got - want).abs() < 0.05 * want,
                "half spread {wide}: {name} {got:.4}, from the liquid 7- and 30-day slices it is {want}"
            );
        }
    }
}

#[test]
fn a_bootstrap_takes_the_interpolated_start_unless_its_book_pins_a_smile() {
    // the 7- and 30-day slices refit on SMILE; the 14-day market is 2 vol points higher with a steeper put wing
    let market = JWFit {
        v: 0.0484,
        p: 1.3,
        ..SMILE
    };
    let fit_14_days = |quotes: Quotes| {
        let (bids, asks, strikes, expiries, is_calls) =
            quotes.on(&SMILE, 7, strikes()).on(&SMILE, 30, strikes()).into_arrays();
        let surface = Quotes::get_fitter_on(&Array1::from(vec![expiry(7), expiry(14), expiry(30)]))
            .with_quotes(bids, asks, strikes, expiries, is_calls)
            .expect("a forward and a discount for every quoted expiry");
        let starts = HashMap::from([
            (expiry(7), SliceStart::Init(svi(&SMILE, 7))),
            (expiry(14), SliceStart::Bootstrap(vec![])),
            (expiry(30), SliceStart::Init(svi(&SMILE, 30))),
        ]);
        let outcome = surface.fit_multithread(&starts, 100).expect("every start has a forward").remove(&expiry(14));
        let outcome = outcome.expect("an outcome");
        assert_eq!(outcome.status, SliceStatus::Bootstrapped);
        outcome.fit.expect("a bootstrap").0
    };
    let is_near = |fit: &JWFit, smile: &JWFit| {
        (fit.v.sqrt() - smile.v.sqrt()).abs() < HALF_SPREAD && (fit.p - smile.p).abs() < 0.1 * smile.p
    };

    // an empty book has only the interpolated smile to go on
    let empty = fit_14_days(Quotes::default());
    assert!(is_near(&empty, &SMILE), "empty book: {empty:?}");
    // 15 vol points either side reads no start off the quotes, and the interpolated smile fits inside them
    let wide = fit_14_days(Quotes::default().on_with_spread(&market, 14, strikes(), 0.15));
    assert!(is_near(&wide, &SMILE), "wide book: {wide:?}");
    // a tight book pins its own smile, which the quote heuristics find and the interpolated start misses
    let tight = fit_14_days(Quotes::default().on(&market, 14, strikes()));
    assert!(is_near(&tight, &market), "tight book: {tight:?}");
}

#[test]
fn a_restart_interpolates_a_wide_slice_from_the_other_bootstraps() {
    // a runner with no fits to start from bootstraps every slice: a wide one must not fail for want of a refit
    let surface = Quotes::default()
        .on(&SMILE, 7, strikes())
        .on_with_spread(&SMILE, 14, strikes(), 0.15)
        .on(&SMILE, 30, strikes())
        .get_surface();
    let starts: HashMap<i64, SliceStart> =
        [7, 14, 30].into_iter().map(|days| (expiry(days), SliceStart::Bootstrap(vec![]))).collect();
    let outcomes = surface.fit_multithread(&starts, 100).expect("every start has a forward");

    let wide = &outcomes[&expiry(14)];
    assert_eq!(wide.status, SliceStatus::Bootstrapped);
    let (fit, _) = wide.fit.as_ref().expect("an interpolated smile");
    assert!(
        (fit.v - SMILE.v).abs() < 0.05 * SMILE.v,
        "v {:.4}, the bootstrapped 7- and 30-day slices are near {}",
        fit.v,
        SMILE.v
    );
}

#[test]
fn a_bootstrap_tie_goes_to_the_earliest_given_start() {
    // on a book that cannot tell smiles apart, the caller's first start comes back
    let surface = Quotes::default().on_with_spread(&SMILE, 30, strikes(), 0.02).get_surface();
    let (first, second) = (
        SMILE,
        JWFit {
            psi: SMILE.psi * 0.98,
            ..SMILE
        },
    );
    let psi_from = |given: Vec<SVIJWParams>| {
        let mut outcomes = surface.fit_multithread(&HashMap::from([(expiry(30), SliceStart::Bootstrap(given))]), 100);
        let outcome = outcomes.as_mut().expect("a forward").remove(&expiry(30)).expect("an outcome");
        assert_eq!(outcome.status, SliceStatus::Bootstrapped);
        outcome.fit.expect("a bootstrap").0.psi
    };
    assert_eq!(psi_from(vec![svi(&first, 30), svi(&second, 30)]), first.psi);
    assert_eq!(psi_from(vec![svi(&second, 30), svi(&first, 30)]), second.psi);
}

#[test]
fn a_bootstrap_too_wide_to_read_starts_off_fits_from_its_given_starts() {
    // a caller's start bootstraps a slice whose quotes are too loose to read one off, and an unfittable one is skipped
    let surface = Quotes::default().on_with_spread(&SMILE, 30, strikes(), 0.15).get_surface();
    let bootstrap = |given: Vec<SVIJWParams>| {
        let mut outcomes = surface.fit_multithread(&HashMap::from([(expiry(30), SliceStart::Bootstrap(given))]), 100);
        outcomes.as_mut().expect("a forward").remove(&expiry(30)).expect("an outcome")
    };
    let alone = bootstrap(vec![]);
    assert!(
        matches!(alone.status, SliceStatus::Failed(_)),
        "no tight quote to read a start off, nor another slice to interpolate from"
    );
    assert!(alone.fit.is_none(), "a failed bootstrap has nothing to quote off");
    let flat_put_wing = JWFit { p: 0.0, ..SMILE };
    let from_given = bootstrap(vec![svi(&flat_put_wing, 30), svi(&SMILE, 30)]);
    assert_eq!(from_given.status, SliceStatus::Bootstrapped);
    let (fit, _) = from_given.fit.expect("a bootstrap");
    assert!((fit.p - SMILE.p).abs() < 1e-9, "p {}", fit.p);
}
