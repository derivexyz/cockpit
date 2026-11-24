use super::{expiry, REF_SEC};
use crate::fitters::slice_fitter::{default_cost_cfg, CalibrationLog, JWFit, JWFitCostConfig};
use std::collections::HashMap;

fn smile(v: f64) -> JWFit {
    JWFit {
        v,
        psi: -0.3,
        p: 1.0,
        c: 0.4,
        v_diff: 0.2 * v,
    }
}

#[test]
fn a_log_reads_one_slice_at_a_time_however_its_lines_interleave() {
    // slices fitted on threads of their own write at once, and their lines rarely name the slice
    let log = CalibrationLog::new();
    let (near, far) = (log.for_expiry(1_000), log.for_expiry(2_000));
    far.push("far 1");
    near.push("near 1");
    log.push("surface 1");
    far.push("far 2");
    near.push("near 2");
    log.push("surface 2");
    let contents = log.contents();
    let lines: Vec<&str> = contents.lines().map(|line| line.split_once(' ').expect("a timestamp").1).collect();
    assert_eq!(lines, ["surface 1", "surface 2", "near 1", "near 2", "far 1", "far 2"]);
}

#[test]
fn an_average_of_fits_leans_to_the_heavier_ones() {
    // a slice's init is its recent fits averaged with weights that halve with age
    let mean = JWFit::weighted_mean([(1.0, &smile(0.04)), (3.0, &smile(0.08))]);
    assert!((mean.v - 0.07).abs() < 1e-12, "v {}", mean.v);
    assert!((mean.v_diff - 0.014).abs() < 1e-12, "v_diff {}", mean.v_diff);
    assert!((mean.psi + 0.3).abs() < 1e-12, "psi {}", mean.psi);
}

#[test]
fn an_expiry_outside_the_fitted_ones_takes_the_nearest_fit() {
    // a new listing past the longest fitted expiry still gets a smile, held flat in tenor
    let surface = HashMap::from([(expiry(30), smile(0.04)), (expiry(60), smile(0.09))]);
    let v_at = |days: i64| JWFit::get_interpolated_or_nearest(&surface, expiry(days), REF_SEC).map(|fit| fit.v);
    // total variance linear in time: (0.04 * 30 + 0.09 * 60) / 2 over 45 days
    let between = v_at(45).expect("an interpolation");
    assert!((between - 3.3 / 45.0).abs() < 1e-12, "v {between}");
    assert_eq!(v_at(90), Some(0.09));
    assert_eq!(v_at(10), Some(0.04));
    assert!(JWFit::get_interpolated_or_nearest(&HashMap::new(), expiry(90), REF_SEC).is_none());
}

#[test]
fn the_v_regularization_scales_with_tau_within_its_bounds() {
    // the quotes' cost of a v move grows with tau: a fixed weight would freeze short slices, free long ones
    let base = default_cost_cfg();
    let min_tau = 1.0 / 365.0;
    let v_cost = |tau: f64| base.clone().time_scaled(tau, min_tau).v_cost;
    assert!(
        (v_cost(30.0 / 365.0) - base.v_cost).abs() < 1e-9,
        "a month keeps the default"
    );
    assert!(
        (v_cost(60.0 / 365.0) - 2.0 * base.v_cost).abs() < 1e-9,
        "two months double it"
    );
    assert_eq!(
        v_cost(2.0),
        v_cost(0.25),
        "long dated slices stop at a quarter's, so they do not freeze"
    );
    assert_eq!(v_cost(1e-6), v_cost(min_tau), "an expiring slice keeps min_tau's");
    let scaled = base.clone().time_scaled(60.0 / 365.0, min_tau);
    assert_eq!(
        (scaled.psi_cost, scaled.p_cost, scaled.c_cost),
        (base.psi_cost, base.p_cost, base.c_cost),
        "the skew and wings are not variance levels"
    );
}

#[test]
fn scaling_the_regularization_leaves_the_quote_constraint_and_arbitrage_costs() {
    // lighter regularization lets a slice follow its quotes, but must not loosen the quote or no-arb costs
    let base = default_cost_cfg();
    let scaled = base.clone().scale_regularization(0.1);
    let weights = |cfg: &JWFitCostConfig| [cfg.v_cost, cfg.psi_cost, cfg.p_cost, cfg.c_cost, cfg.v_diff_cost];
    assert_eq!(weights(&scaled), weights(&base).map(|weight| weight * 0.1));
    assert_eq!(
        (scaled.bbo_cost, scaled.constraints_cost, scaled.arb_cost),
        (base.bbo_cost, base.constraints_cost, base.arb_cost)
    );
}
