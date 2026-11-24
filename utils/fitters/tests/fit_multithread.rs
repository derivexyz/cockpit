//! `fit_multithread` on three recorded surfaces, `a`, `b` and `c`: each slice's bids and asks, scaled
//! so the front forward is 100, with expiries in seconds from the pass. A slice's `good_init` is its
//! fit from the pass before. The fits use a production runner's settings: a tenth of the default
//! regularization, its v terms floored at 15 minutes to expiry, and 100 BFGS iterations.
//!
//! What these settings handle:
//! - A refit from a good init, and a bootstrap, on every slice. Near the money the fits miss `a`'s and
//!   `b`'s tight books by at most about 25 vol bp, and sit inside most of `c`'s wider ones.
//! - An init interpolated between close neighbours, e.g. 8 days between 7 and 9.
//!
//! What they fail at, `#[ignore]`d below until the default config is revisited:
//! - A flat init: the at-the-money level, no skew, even wings. The fits keep that shape and miss near
//!   the money by up to 5.6 vol pts, at 20 to 6,500 times a good refit's bbo cost. The skew and wing
//!   regularization is not scaled by time to expiry, and it outweighs the quotes. More iterations do
//!   not help. On five slices every BFGS attempt runs off or finds no descent, and the fitter returns
//!   the init unchanged.
//! - An init interpolated across far neighbours, e.g. 1 day between 2 hours and 7 days. The fits miss
//!   near the money by up to 2.3 vol pts, at up to 156 times a good refit's bbo cost.
//! - Not tested: an init 20% above the at-the-money vol, with the right shape. `a` and `b` recover from
//!   it, but on `c`'s wide books the fits stay at the edge nearest the init, up to 2.2 vol pts high.
//!
//! Repeated fits converge on some slices, slowly. Refitting from the last fit, most of `a` from 9 days
//! out and `b`'s 7 to 9 days get there within 15 passes. From a runner's init, the average of the last
//! 7 minutes' fits, the same takes 25 to 135 passes of 15 s. The shortest slices do not get there
//! within an hour, and some flat fits stay stuck, e.g. at 87 days with the skew near 0.
use crate::black76::OptionContract;
use crate::fitters::slice_fitter::{default_cost_cfg, JWFit, JWFitCostConfig};
use crate::fitters::surface_fitter::{SliceOutcome, SliceStart, SliceStatus, SurfaceFitter};
use crate::fitters::svi::SVIJWParams;
use crate::SEC_PER_YEAR;
use ndarray::Array1;
use serde::Deserialize;
use std::collections::HashMap;

const PASS_SEC: i64 = 1_000_000;
const REGULARIZATION_SCALE: f64 = 0.1;
const MIN_TAU: f64 = 15.0 * 60.0 / SEC_PER_YEAR;
const MAX_ITERS: u64 = 100;

/// (days to expiry, at-the-money vol, largest miss (vol bp) within one at-the-money standard
/// deviation of the forward, bbo cost)
type Expected = (f64, f64, f64, f64);

/// `a` and `b` quote tighter books than an SVI can thread: their fits miss some quotes near the
/// money by up to about 25 vol bp. `c`'s wider books hold most of its fits.
const GOOD_INIT: [(&str, &[Expected]); 3] = [
    (
        "a",
        &[
            (0.08, 0.3020, 13.0, 1.039e-5),
            (1.08, 0.2900, 8.6, 4.662e-4),
            (7.08, 0.2236, 16.4, 1.345e-3),
            (8.08, 0.2300, 19.4, 2.449e-3),
            (9.08, 0.2348, 18.9, 3.602e-3),
            (35.08, 0.2132, 17.5, 5.973e-3),
            (66.08, 0.2057, 13.9, 1.460e-2),
            (87.08, 0.2022, 11.4, 2.296e-2),
            (178.04, 0.1992, 15.1, 3.928e-2),
        ],
    ),
    (
        "b",
        &[
            (0.08, 0.4040, 24.6, 1.416e-4),
            (1.08, 0.4398, 10.1, 1.039e-3),
            (7.08, 0.3317, 12.4, 9.849e-4),
            (8.08, 0.3413, 6.1, 8.025e-4),
            (9.08, 0.3468, 14.0, 9.840e-4),
            (37.08, 0.3064, 6.0, 8.599e-4),
            (66.08, 0.2891, 9.6, 1.021e-3),
            (87.08, 0.2839, 7.3, 2.609e-3),
        ],
    ),
    (
        "c",
        &[
            (1.02, 0.4687, 0.0, 1.574e-5),
            (7.02, 0.3990, 0.0, 1.889e-5),
            (8.02, 0.4176, 0.0, 0.0),
            (9.02, 0.4310, 0.0, 2.058e-9),
            (22.02, 0.4699, 14.1, 2.430e-3),
            (51.02, 0.4622, 3.6, 9.958e-5),
            (85.02, 0.4419, 1.3, 1.666e-4),
            (173.98, 0.3946, 10.2, 7.438e-3),
        ],
    ),
];

/// Within a few tenths of a vol of the refits' at-the-money vols, and as close to the books near the
/// money, closer on the shortest expiries, which regularization holds back.
const BOOTSTRAP: [(&str, &[Expected]); 3] = [
    (
        "a",
        &[
            (0.08, 0.3022, 11.4, 9.320e-6),
            (1.08, 0.2896, 16.7, 4.384e-4),
            (7.08, 0.2235, 9.2, 8.785e-4),
            (8.08, 0.2301, 13.1, 1.972e-3),
            (9.08, 0.2350, 13.4, 3.280e-3),
            (35.08, 0.2136, 9.9, 5.435e-3),
            (66.08, 0.2058, 12.2, 1.452e-2),
            (87.08, 0.2021, 12.0, 2.429e-2),
            (178.04, 0.1992, 12.9, 3.853e-2),
        ],
    ),
    (
        "b",
        &[
            (0.08, 0.4046, 5.7, 1.427e-5),
            (1.08, 0.4397, 11.2, 1.009e-3),
            (7.08, 0.3318, 4.7, 9.470e-4),
            (8.08, 0.3410, 10.4, 7.436e-4),
            (9.08, 0.3472, 5.7, 8.947e-4),
            (37.08, 0.3066, 4.0, 7.949e-4),
            (66.08, 0.2892, 7.9, 9.950e-4),
            (87.08, 0.2840, 7.2, 2.386e-3),
        ],
    ),
    (
        "c",
        &[
            (1.02, 0.4726, 0.0, 1.211e-10),
            (7.02, 0.3969, 0.0, 1.090e-5),
            (8.02, 0.4193, 0.3, 5.897e-8),
            (9.02, 0.4329, 0.0, 4.667e-7),
            (22.02, 0.4699, 13.5, 2.529e-3),
            (51.02, 0.4620, 2.5, 1.107e-4),
            (85.02, 0.4419, 0.8, 1.137e-4),
            (173.98, 0.3943, 9.9, 1.145e-2),
        ],
    ),
];

/// Each surface's 8-day slice, from an init interpolated between its 7- and 9-day neighbours: within
/// a tenth of a vol of the refit from a good init.
const BETWEEN_CLOSE_NEIGHBOURS: [(&str, Expected); 3] = [
    ("a", (8.08, 0.2300, 15.9, 2.219e-3)),
    ("b", (8.08, 0.3403, 12.0, 1.030e-3)),
    ("c", (8.02, 0.4173, 0.0, 3.146e-11)),
];

#[derive(Deserialize)]
struct Recorded {
    surfaces: Vec<RecordedSurface>,
}

#[derive(Deserialize)]
struct RecordedSurface {
    name: String,
    slices: Vec<RecordedSlice>,
}

#[derive(Deserialize)]
struct RecordedSlice {
    /// seconds from the pass
    expiry_sec: i64,
    forward: f64,
    discount: f64,
    /// (strike, is_call, bid, ask)
    quotes: Vec<(f64, bool, Option<f64>, Option<f64>)>,
    /// v, psi, p, c, v_diff
    good_init: [f64; 5],
}

/// A production runner's settings: a tenth of the default regularization, its v terms floored at
/// 15 minutes to expiry, and no buffer on the bid and ask.
fn cost_cfg(tau: f64) -> JWFitCostConfig {
    JWFitCostConfig {
        buffer_bp: 0.0,
        ..default_cost_cfg().time_scaled(tau, MIN_TAU).scale_regularization(REGULARIZATION_SCALE)
    }
}

fn recorded() -> Vec<RecordedSurface> {
    let recorded: Recorded =
        serde_json::from_str(include_str!("fixtures/recorded_surfaces.json")).expect("the fixture parses");
    recorded.surfaces
}

impl RecordedSlice {
    fn expiry(&self) -> i64 {
        PASS_SEC + self.expiry_sec
    }

    fn days(&self) -> f64 {
        self.expiry_sec as f64 / 86_400.0
    }

    fn tau(&self) -> f64 {
        self.expiry_sec as f64 / SEC_PER_YEAR
    }

    fn good_init(&self) -> JWFit {
        let [v, psi, p, c, v_diff] = self.good_init;
        JWFit { v, psi, p, c, v_diff }
    }

    /// The good init's at-the-money level with no skew and even wings.
    fn flat_init(&self) -> JWFit {
        let v = self.good_init().v;
        JWFit {
            v,
            psi: 0.0,
            p: 0.5,
            c: 0.5,
            v_diff: 0.05 * v,
        }
    }

    fn svi(&self, fit: &JWFit) -> SVIJWParams {
        fit.to_svi_jw(self.forward, self.tau())
    }

    /// Each quote's miss in vol bp: how far `fit`'s vol lies below its bid's or above its ask's.
    fn misses_bp(&self, fit: &JWFit) -> Vec<f64> {
        let raw = self.svi(fit).to_raw();
        self.quotes
            .iter()
            .map(|&(strike, is_call, bid, ask)| {
                let contract = OptionContract {
                    strike,
                    expiry_sec: self.expiry_sec as f64,
                    is_call,
                };
                let vol_at = |price: Option<f64>| {
                    price.and_then(|price| contract.implied_vol(self.forward, price / self.discount))
                };
                let fitted = raw.get_vol(strike);
                let below_bid = vol_at(bid).map_or(0.0, |bid_vol| bid_vol - fitted);
                let above_ask = vol_at(ask).map_or(0.0, |ask_vol| fitted - ask_vol);
                1e4 * below_bid.max(above_ask).max(0.0)
            })
            .collect()
    }

    /// The largest miss (vol bp) among the quotes within one at-the-money standard deviation of the
    /// forward, where a fit's vols matter most. Further out, a vol miss costs little in price.
    fn near_money_max_miss_bp(&self, fit: &JWFit) -> f64 {
        let atm_sd = (self.good_init().v * self.tau()).sqrt();
        let near_money = self.quotes.iter().map(|quote| (quote.0 / self.forward).ln().abs() < atm_sd);
        near_money
            .zip(self.misses_bp(fit))
            .filter(|(near, _)| *near)
            .map(|(_, miss)| miss)
            .fold(0.0, f64::max)
    }

    fn assert_fit(&self, surface: &str, outcome: &SliceOutcome, status: SliceStatus, expected: Expected) {
        let (days, atm_vol, max_miss_bp, bbo_cost) = expected;
        let label = format!("surface {surface} at {days} days");
        assert!((self.days() - days).abs() < 0.005, "{label}: rows follow the slices");
        assert_eq!(outcome.status, status, "{label}");
        let (fit, actual_bbo_cost) = outcome.fit.as_ref().expect("a fit");
        assert!(
            (fit.v.sqrt() - atm_vol).abs() < 1e-4,
            "{label}: at-the-money vol {:.4}",
            fit.v.sqrt()
        );
        let miss_bp = self.near_money_max_miss_bp(fit);
        assert!(
            (miss_bp - max_miss_bp).abs() < 0.2,
            "{label}: near-the-money miss {miss_bp:.1} bp"
        );
        let bbo_cost_off = (actual_bbo_cost - bbo_cost).abs();
        assert!(
            bbo_cost_off <= 1e-3 * bbo_cost + 1e-9,
            "{label}: bbo cost {actual_bbo_cost:.3e}"
        );
    }
}

impl RecordedSurface {
    fn fitter(&self) -> SurfaceFitter {
        let by_expiry = |value: fn(&RecordedSlice) -> f64| -> HashMap<i64, f64> {
            self.slices.iter().map(|slice| (slice.expiry(), value(slice))).collect()
        };
        let cost_cfgs: HashMap<i64, JWFitCostConfig> =
            self.slices.iter().map(|slice| (slice.expiry(), cost_cfg(slice.tau()))).collect();
        let (mut bids, mut asks, mut strikes, mut expiries, mut is_calls) = (vec![], vec![], vec![], vec![], vec![]);
        for slice in &self.slices {
            for &(strike, is_call, bid, ask) in &slice.quotes {
                bids.push(bid.unwrap_or(0.0));
                asks.push(ask.unwrap_or(f64::MAX));
                strikes.push(strike);
                expiries.push(slice.expiry());
                is_calls.push(is_call);
            }
        }
        SurfaceFitter::new_with_cfgs(
            PASS_SEC,
            by_expiry(|slice| slice.forward),
            by_expiry(|slice| slice.discount),
            cost_cfgs,
        )
        .with_quotes(
            Array1::from(bids),
            Array1::from(asks),
            Array1::from(strikes),
            Array1::from(expiries),
            Array1::from(is_calls),
        )
        .expect("every quoted expiry has a forward, a discount and a cost config")
    }

    fn fit(&self, start: impl Fn(&RecordedSlice) -> SliceStart) -> HashMap<i64, SliceOutcome> {
        let starts: HashMap<i64, SliceStart> = self.slices.iter().map(|slice| (slice.expiry(), start(slice))).collect();
        self.fitter().fit_multithread(&starts, MAX_ITERS).expect("every start is quoted")
    }

    /// The other slices' good inits interpolated to `slice`'s expiry, or the nearest beyond either end.
    fn interpolated_init(&self, slice: &RecordedSlice) -> JWFit {
        let others: HashMap<i64, JWFit> = self
            .slices
            .iter()
            .filter(|other| other.expiry() != slice.expiry())
            .map(|other| (other.expiry(), other.good_init()))
            .collect();
        JWFit::get_interpolated_or_nearest(&others, slice.expiry(), PASS_SEC).expect("other slices")
    }

    fn assert_fits(&self, outcomes: &HashMap<i64, SliceOutcome>, status: SliceStatus, expected: &[Expected]) {
        assert_eq!(self.slices.len(), expected.len(), "surface {}", self.name);
        for (slice, &expected) in self.slices.iter().zip(expected) {
            slice.assert_fit(&self.name, &outcomes[&slice.expiry()], status.clone(), expected);
        }
    }

    /// Each slice within twice the bbo cost of the refit from its good init (`GOOD_INIT`), and within
    /// 5 vol bp of its near-the-money miss.
    fn assert_as_close_as_a_good_refit(&self, outcomes: &HashMap<i64, SliceOutcome>, good: &[Expected]) {
        for (slice, &(days, _, good_miss_bp, good_bbo_cost)) in self.slices.iter().zip(good) {
            let label = format!("surface {} at {days} days", self.name);
            let (fit, bbo_cost) = outcomes[&slice.expiry()].fit.as_ref().expect("a fit");
            assert!(
                *bbo_cost <= 2.0 * good_bbo_cost + 1e-6,
                "{label}: bbo cost {bbo_cost:.3e}"
            );
            let miss_bp = slice.near_money_max_miss_bp(fit);
            assert!(
                miss_bp <= good_miss_bp + 5.0,
                "{label}: near-the-money miss {miss_bp:.1} bp"
            );
        }
    }
}

#[test]
fn a_good_init_refits_every_slice_where_its_book_is() {
    // a slice's fit from the pass before is the usual init: a refit from it lands where the market is
    for (surface, (name, expected)) in recorded().iter().zip(GOOD_INIT) {
        assert_eq!(surface.name, name);
        let outcomes = surface.fit(|slice| SliceStart::Init(slice.svi(&slice.good_init())));
        surface.assert_fits(&outcomes, SliceStatus::Refit, expected);
    }
}

#[test]
fn every_slice_bootstraps_from_its_quotes_alone() {
    // a new listing has no fit to start from, and its first fit is the init of every later one
    for (surface, (name, expected)) in recorded().iter().zip(BOOTSTRAP) {
        assert_eq!(surface.name, name);
        let outcomes = surface.fit(|_| SliceStart::Bootstrap);
        surface.assert_fits(&outcomes, SliceStatus::Bootstrapped, expected);
    }
}

#[test]
fn an_init_interpolated_between_close_neighbours_refits_where_its_book_is() {
    // a new listing between two fitted expiries can start from them
    for (surface, (name, expected)) in recorded().iter().zip(BETWEEN_CLOSE_NEIGHBOURS) {
        assert_eq!(surface.name, name);
        let slice = surface.slices.iter().find(|slice| (slice.days() - expected.0).abs() < 0.005).expect("the slice");
        let start = SliceStart::Init(slice.svi(&surface.interpolated_init(slice)));
        let outcomes = surface.fitter().fit_multithread(&HashMap::from([(slice.expiry(), start)]), MAX_ITERS);
        let outcome = &outcomes.expect("the start is quoted")[&slice.expiry()];
        slice.assert_fit(name, outcome, SliceStatus::Refit, expected);
    }
}

#[test]
#[ignore = "TODO: revisit the default config: from a flat init the fits keep its shape (see the summary above)"]
fn a_flat_init_refits_every_slice_where_its_book_is() {
    // the at-the-money level with no shape is the generic start for a slice with nothing better
    for (surface, (name, good)) in recorded().iter().zip(GOOD_INIT) {
        assert_eq!(surface.name, name);
        let outcomes = surface.fit(|slice| SliceStart::Init(slice.svi(&slice.flat_init())));
        surface.assert_as_close_as_a_good_refit(&outcomes, good);
    }
}

#[test]
#[ignore = "TODO: revisit the default config: across far neighbours the fits keep the interpolated shape (see the summary above)"]
fn an_init_interpolated_from_the_other_slices_refits_every_slice_where_its_book_is() {
    // a new listing between or beyond fitted expiries starts from them, however far they are
    for (surface, (name, good)) in recorded().iter().zip(GOOD_INIT) {
        assert_eq!(surface.name, name);
        let outcomes = surface.fit(|slice| SliceStart::Init(slice.svi(&surface.interpolated_init(slice))));
        surface.assert_as_close_as_a_good_refit(&outcomes, good);
    }
}
