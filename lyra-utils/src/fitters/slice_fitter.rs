use crate::fitters::svi::{SVIJWParams, SVIRawParams};
use crate::SEC_PER_YEAR;
use anyhow::{anyhow, bail, Result};
use argmin::core::{
    observers::{Observe, ObserverMode},
    CostFunction, Executor, Gradient, State, KV,
};
use argmin::solver::linesearch::MoreThuenteLineSearch;
use argmin::solver::quasinewton::BFGS;
use crate::black76::OptionContract;
use nalgebra::SMatrix;
use ndarray::{arr1, Array1, Array2};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

const JW_PARAM_LEN: usize = 5;
const FINITE_DIFF_STEP: f64 = 1e-5;
const INITIAL_HESSIAN_STEP: f64 = 1e-3;
const INVALID_COST: f64 = 1e50;
const MIN_INITIAL_INV_HESSIAN: f64 = 1e-8;
const MAX_INITIAL_INV_HESSIAN: f64 = 10.0;
const INV_HESSIAN_ATTEMPT_COUNT: usize = 4;
/// Violations below this are treated as noise and rounded away. Must stay below the smallest
/// violation the tests rely on detecting (`test_arb_cost` trips at 2.772e-5), or they silently
/// start asserting on a cost of exactly zero.
const ARB_COST_ROUND: f64 = 1e-5;
pub const LIQ_SCORE_COUNT: usize = 8;
const REG_MAX_V_DIFF: f64 = 0.05;

#[derive(Clone, Default)]
pub struct CalibrationLog {
    lines: Arc<Mutex<Vec<String>>>,
}

impl CalibrationLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&self, line: impl Into<String>) {
        let ts = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        let mut lines = self.lines.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.push(format!("{ts} {}", line.into()));
    }

    pub fn contents(&self) -> String {
        let lines = self.lines.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if lines.is_empty() {
            String::new()
        } else {
            format!("{}\n", lines.join("\n"))
        }
    }

    pub fn write_to_path(&self, path: impl AsRef<Path>) -> Result<()> {
        Ok(std::fs::write(path, self.contents())?)
    }
}

/// Local copy of the shared helper so this module carries no crate-wide imports.
fn format_expiry(expiry_sec: i64) -> String {
    chrono::DateTime::<chrono::Utc>::from_timestamp(expiry_sec, 0)
        .unwrap_or_default()
        .format("%Y-%m-%d")
        .to_string()
}

fn smooth_pos(x: f64, eps: f64) -> f64 {
    if x > 0.0 {
        x + eps * (-x / eps).exp().ln_1p()
    } else {
        // eps * (x / eps).exp().ln_1p()
        0.0
    }
}

#[derive(Clone, Debug)]
pub struct JWFit {
    pub v: f64,
    pub psi: f64,
    pub p: f64,
    pub c: f64,
    pub v_diff: f64,
}

#[derive(Clone)]
pub struct JWFitCostConfig {
    pub v_cost: f64,
    pub psi_cost: f64,
    pub p_cost: f64,
    pub c_cost: f64,
    pub v_diff_cost: f64,
    pub bbo_cost: f64,
    pub constraints_cost: f64,
    pub arb_cost: f64,
    pub smoothing_eps: f64,
}

pub fn default_cost_cfg() -> JWFitCostConfig {
    JWFitCostConfig {
        v_cost: 1000.0,
        psi_cost: 2_000.0,
        p_cost: 100.0,
        c_cost: 100.0,
        v_diff_cost: 10_000.0,
        bbo_cost: 10_000.0,
        constraints_cost: 10_000_000.0,
        arb_cost: 100.0,
        smoothing_eps: 1e-8,
    }
}

impl JWFit {
    pub fn to_param(&self) -> Array1<f64> {
        assert!(self.v > 0.0);
        assert!(self.p > 0.0);
        assert!(self.c > 0.0);
        assert!(self.v_diff > 0.0);

        arr1(&[self.v.ln(), self.psi, self.p.ln(), self.c.ln(), self.v_diff.ln()])
    }

    pub fn try_from_param(param: &Array1<f64>) -> Result<Self> {
        if param.len() != JW_PARAM_LEN {
            return Err(anyhow!("expected {JW_PARAM_LEN} JW params, got {}", param.len()));
        }

        Ok(Self {
            v: param[0].exp(),
            psi: param[1],
            p: param[2].exp(),
            c: param[3].exp(),
            v_diff: param[4].exp(),
        })
    }

    pub fn from_svi_jw(svi: &SVIJWParams) -> Self {
        Self {
            v: svi.v,
            psi: svi.psi,
            p: svi.p,
            c: svi.c,
            v_diff: svi.v_diff,
        }
    }

    pub fn to_svi_jw(&self, fwd: f64, tau: f64) -> SVIJWParams {
        SVIJWParams {
            v: self.v,
            psi: self.psi,
            p: self.p,
            c: self.c,
            v_diff: self.v_diff,
            fwd,
            reftau: tau,
        }
    }

    fn finite_diff_step(value: f64) -> f64 {
        FINITE_DIFF_STEP * value.abs().max(1.0)
    }

    fn w_avg(&self, other: &JWFit, w: f64) -> Self {
        assert!(w > 0.0);
        assert!(w <= 1.0);

        let other_w = 1.0 - w;
        Self {
            v: self.v * w + other.v * other_w,
            psi: self.psi * w + other.psi * other_w,
            p: self.p * w + other.p * other_w,
            c: self.c * w + other.c * other_w,
            v_diff: self.v_diff * w + other.v_diff * other_w,
        }
    }

    pub fn scale_v(mut self, ratio: f64) -> Self {
        self.v *= ratio;
        self
    }

    // takes 2 JW params and returns their weighted avg based on cost. Lower cost -> higher weight
    pub fn cost_avg(&self, other: &JWFit, cost: f64, other_cost: f64) -> Self {
        if (cost - other_cost).abs() < 1e-9 {
            // note in the event of missing liquidity, bbo costs are always 0
            // hence the choice of weight here determines where the illiquid vols would get fit
            self.w_avg(other, 0.5)
        } else if cost < 1e-9 {
            self.clone()
        } else if other_cost < 1e-9 {
            other.clone()
        } else {
            let w = other_cost / (cost + other_cost);
            self.w_avg(other, w)
        }
    }

    // returns the ratio of the v parameter of expiry / other
    pub fn get_ratio(surface: &HashMap<i64, JWFit>, expiry: i64, other: i64, now_sec: i64) -> Option<f64> {
        let this_interpolated = JWFit::get_interpolated_at(surface, expiry, now_sec);
        let other_interpolated = JWFit::get_interpolated_at(surface, other, now_sec);

        match (this_interpolated, other_interpolated) {
            (Some(this), Some(other)) => Some(this.v / other.v),
            _ => None,
        }
    }

    pub fn get_damp_ratio(
        surface: &HashMap<i64, JWFit>,
        expiry: i64,
        other: i64,
        now_sec: i64,
        damp: f64,
    ) -> Option<f64> {
        assert!(damp >= 0.0 && damp <= 1.0);
        Self::get_ratio(surface, expiry, other, now_sec).map(|v| (v.ln() * damp).exp())
    }

    pub fn get_interpolated(surface: &HashMap<i64, JWFit>, expiry: i64) -> Option<JWFit> {
        Self::get_interpolated_at(surface, expiry, chrono::Utc::now().timestamp())
    }

    pub fn get_interpolated_at(surface: &HashMap<i64, JWFit>, expiry: i64, now_sec: i64) -> Option<JWFit> {
        if surface.is_empty() {
            return None;
        }

        if surface.len() == 1 {
            return surface.values().next().cloned();
        }

        if let Some(fit) = surface.get(&expiry) {
            return Some(fit.clone());
        }

        let mut expiries: Vec<i64> = surface.keys().copied().collect();
        expiries.sort_unstable();

        let target_time = (expiry - now_sec) as f64;
        if target_time <= 0.0 {
            return None;
        }

        for window in expiries.windows(2) {
            let lower_expiry = window[0];
            let upper_expiry = window[1];
            if !(lower_expiry < expiry && expiry < upper_expiry) {
                continue;
            }

            let lower = surface.get(&lower_expiry)?;
            let upper = surface.get(&upper_expiry)?;
            let lower_time = (lower_expiry - now_sec) as f64;
            let upper_time = (upper_expiry - now_sec) as f64;
            if lower_time <= 0.0 || upper_time <= 0.0 {
                return None;
            }
            let weight = (target_time - lower_time) / (upper_time - lower_time);

            let interp = |lower_param: f64, upper_param: f64| {
                let lower_total = lower_param * lower_time;
                let upper_total = upper_param * upper_time;
                (lower_total + (upper_total - lower_total) * weight) / target_time
            };

            return Some(Self {
                v: interp(lower.v, upper.v),
                psi: interp(lower.psi, upper.psi),
                p: interp(lower.p, upper.p),
                c: interp(lower.c, upper.c),
                v_diff: interp(lower.v_diff, upper.v_diff),
            });
        }

        None
    }
}

#[derive(Clone)]
pub struct SliceFitter {
    pub ref_sec: i64,

    cost_cfg: JWFitCostConfig,
    init_svi: SVIJWParams,
    bids: Array1<f64>,
    asks: Array1<f64>,
    strikes: Array1<f64>,
    contracts: Vec<OptionContract>,
    expiry: i64,
    fwd: f64,
    discount: f64,
    calibration_log: Option<CalibrationLog>,
}

struct InvHessianAttempt {
    name: &'static str,
    inv_hessian: Array2<f64>,
}

#[derive(Clone)]
struct BfgsBestParam {
    param: Array1<f64>,
    cost: f64,
}

struct BfgsAttemptOutput {
    result: Result<(JWFit, f64)>,
    best_param: Option<BfgsBestParam>,
}

struct BfgsTracingObserver {
    attempt: &'static str,
    calibration_log: Option<CalibrationLog>,
    best_param: Arc<Mutex<Option<BfgsBestParam>>>, // used to pass to further re-attempts
}

impl BfgsTracingObserver {
    fn log(&self, line: impl Into<String>) {
        if let Some(calibration_log) = &self.calibration_log {
            calibration_log.push(line);
        }
    }

    fn format_fit(param: Option<&Array1<f64>>) -> String {
        match param {
            Some(param) => match JWFit::try_from_param(param) {
                Ok(fit) => format!("{fit:?}"),
                Err(error) => format!("invalid_jw_fit error={error}, raw_param={param:?}"),
            },
            None => "None".to_string(),
        }
    }

    fn record_best<I>(&self, state: &I)
    where
        I: State<Param = Array1<f64>, Float = f64>,
    {
        let Some((param, cost)) = state
            .get_best_param()
            .map(|param| (param, state.get_best_cost()))
            .or_else(|| state.get_param().map(|param| (param, state.get_cost())))
        else {
            return;
        };

        if !SliceFitter::is_valid_cost(cost) || !param.iter().all(|value| value.is_finite()) {
            return;
        }

        let mut best_param = self.best_param.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if best_param.as_ref().is_none_or(|best| cost < best.cost) {
            *best_param = Some(BfgsBestParam {
                param: param.clone(),
                cost,
            });
        }
    }
}

impl<I> Observe<I> for BfgsTracingObserver
where
    I: State<Param = Array1<f64>, Float = f64>,
{
    fn observe_init(&mut self, optimizer: &str, state: &I, _kv: &KV) -> Result<()> {
        self.record_best(state);
        let current_fit = Self::format_fit(state.get_param());
        let best_fit = Self::format_fit(state.get_best_param());
        self.log(format!(
            "INFO BFGS optimizer initialized optimizer={optimizer}, attempt={}, iter={}, cost={}, current_fit={}, best_cost={}, best_fit={}",
            self.attempt,
            state.get_iter(),
            state.get_cost(),
            current_fit,
            state.get_best_cost(),
            best_fit
        ));
        Ok(())
    }

    fn observe_iter(&mut self, state: &I, _kv: &KV) -> Result<()> {
        self.record_best(state);
        let current_fit = Self::format_fit(state.get_param());
        let best_fit = Self::format_fit(state.get_best_param());
        self.log(format!(
            "INFO BFGS optimizer iteration attempt={}, iter={}, cost={}, current_fit={}, best_cost={}, best_fit={}, func_counts={:?}",
            self.attempt,
            state.get_iter(),
            state.get_cost(),
            current_fit,
            state.get_best_cost(),
            best_fit,
            state.get_func_counts()
        ));
        Ok(())
    }

    fn observe_final(&mut self, state: &I) -> Result<()> {
        self.record_best(state);
        let current_fit = Self::format_fit(state.get_param());
        let best_fit = Self::format_fit(state.get_best_param());
        self.log(format!(
            "INFO BFGS optimizer finished attempt={}, iter={}, cost={}, current_fit={}, best_cost={}, best_fit={}, termination_status={:?}, termination_reason={:?}",
            self.attempt,
            state.get_iter(),
            state.get_cost(),
            current_fit,
            state.get_best_cost(),
            best_fit,
            state.get_termination_status(),
            state.get_termination_reason()
        ));
        Ok(())
    }
}

impl SliceFitter {
    pub fn new(
        cost_cfg: JWFitCostConfig,
        init_svi: SVIJWParams,
        expiry: i64,
        fwd: f64,
        discount: f64,
        ref_sec: i64,
    ) -> Self {
        Self {
            cost_cfg,
            init_svi,
            bids: Array1::zeros(0),
            asks: Array1::zeros(0),
            strikes: Array1::zeros(0),
            contracts: vec![],
            expiry,
            ref_sec,
            fwd,
            discount,
            calibration_log: None,
        }
    }

    pub fn with_calibration_log(mut self, calibration_log: CalibrationLog) -> Self {
        self.calibration_log = Some(calibration_log);
        self
    }

    fn log(&self, line: impl Into<String>) {
        if let Some(calibration_log) = &self.calibration_log {
            calibration_log.push(line);
        }
    }

    pub fn build_contracts(
        mut self,
        bids: Array1<f64>,
        asks: Array1<f64>,
        strikes: Array1<f64>,
        is_calls: Array1<bool>,
    ) -> Self {
        assert!(bids.len() == asks.len() && asks.len() == strikes.len() && strikes.len() == is_calls.len());
        let expiry_sec = (self.expiry - self.ref_sec) as f64;
        let contracts: Vec<OptionContract> = strikes
            .iter()
            .zip(is_calls)
            .map(|(&k, is_call)| OptionContract {
                strike: k,
                expiry_sec,
                is_call,
            })
            .collect();
        self.contracts = contracts;
        self.bids = bids;
        self.asks = asks;
        self.strikes = strikes;
        self
    }

    pub fn remove_outliers(&mut self, n: usize, factor: f64, min_score: f64) -> Result<()> {
        if !min_score.is_finite() || min_score < 0.0 {
            return Err(anyhow!(
                "minimum liquidity score must be finite and non-negative, got {min_score}"
            ));
        }

        let liquidity_score = self.get_liquidity_score();
        if liquidity_score > min_score {
            self.log(format!(
                "INFO remove_outliers clearing books because liquidity_score={} exceeds min_score={}, dte={:.2}",
                liquidity_score,
                min_score,
                self.dte()
            ));
            self.bids = Array1::zeros(0);
            self.asks = Array1::zeros(0);
            self.strikes = Array1::zeros(0);
            self.contracts.clear();
            return Ok(());
        }

        if n == 0 || self.contracts.is_empty() {
            return Ok(());
        }
        if !factor.is_finite() || factor <= 0.0 {
            return Err(anyhow!("outlier factor must be finite and positive, got {factor}"));
        }

        let residuals = self.bbo_residual_costs_from_fit(&self.init_fit(), false)?;
        if residuals.len() != self.contracts.len() {
            bail!("Unexpected difference in contracts and residuals")
        }

        let avg_residual = residuals.iter().sum::<f64>() / residuals.len() as f64;
        if !avg_residual.is_finite() || avg_residual <= 0.0 {
            return Ok(());
        }

        let threshold = factor * avg_residual;
        let mut outliers: Vec<(usize, f64)> =
            residuals.iter().copied().enumerate().filter(|(_, residual)| *residual > threshold).collect();
        outliers.sort_by(|left, right| right.1.total_cmp(&left.1));
        outliers.truncate(n);

        if outliers.is_empty() {
            return Ok(());
        }

        let mut remove = vec![false; self.contracts.len()];
        for (idx, _) in &outliers {
            remove[*idx] = true;
        }
        let keep_indices: Vec<usize> = (0..self.contracts.len()).filter(|idx| !remove[*idx]).collect();

        let bids = Array1::from_iter(keep_indices.iter().map(|&idx| self.bids[idx]));
        let asks = Array1::from_iter(keep_indices.iter().map(|&idx| self.asks[idx]));
        let strikes = Array1::from_iter(keep_indices.iter().map(|&idx| self.strikes[idx]));
        let contracts = keep_indices
            .iter()
            .map(|&idx| {
                let contract = &self.contracts[idx];
                OptionContract {
                    strike: contract.strike,
                    expiry_sec: contract.expiry_sec,
                    is_call: contract.is_call,
                }
            })
            .collect();

        let removed = Vec::from_iter(
            outliers.iter().map(|(idx, _)| (self.strikes[*idx], self.contracts[*idx].is_call, residuals[*idx])),
        );

        self.log(format!(
            "INFO remove_outliers removed_count={}, avg_residual={}, expiry={} removed={:?}",
            outliers.len(),
            avg_residual,
            format_expiry(self.expiry),
            removed
        ));

        self.bids = bids;
        self.asks = asks;
        self.strikes = strikes;
        self.contracts = contracts;
        Ok(())
    }

    pub fn with_init_svi(mut self, init_svi: SVIJWParams) -> Self {
        self.init_svi = init_svi;
        self
    }

    pub fn init_fit(&self) -> JWFit {
        JWFit::from_svi_jw(&self.init_svi)
    }

    pub fn init_svi(&self) -> SVIJWParams {
        self.init_svi.clone()
    }

    pub fn set_init_fit(&mut self, fit: &JWFit) {
        self.init_svi = fit.to_svi_jw(self.fwd, self.tau());
    }

    pub fn fit_bfgs(&self, max_iters: u64) -> Result<(JWFit, f64)> {
        let initial_guess = JWFit::from_svi_jw(&self.init_svi);
        let _ = self.cost_from_fit(&initial_guess, true)?; // for logging

        let init_arb_cost = self.arb_cost_from_fit(&initial_guess)?;
        if init_arb_cost == 0.0 {
            self.log("INFO BFGS zero init arb cost, running fast fitter".to_string());
            let mut fast_fitter = self.clone();
            fast_fitter.cost_cfg.arb_cost = 0.0;

            let (optimized_fit, _) = fast_fitter.fit_bfgs_with_guess(&initial_guess, max_iters, 0)?;
            let optimized_arb_cost = self.arb_cost_from_fit(&optimized_fit)?;
            if optimized_arb_cost == 0.0 {
                let optimized_cost = self.cost_from_fit(&optimized_fit, true)?;
                return Ok((optimized_fit, optimized_cost));
            }

            self.log(format!(
                "WARN BFGS introduced arbitrage with arb cost disabled; rerunning from init with full arb cost init_arb_cost={init_arb_cost}, optimized_arb_cost={optimized_arb_cost}"
            ));
            return self.fit_bfgs_with_guess(&initial_guess, max_iters, 0);
        }
        self.log(format!("INFO BFGS {init_arb_cost} init arb cost, running slow fitter",));
        let (opt_fit, opt_cost) = self.fit_bfgs_with_guess(&initial_guess, max_iters, 0)?;
        let _ = self.cost_from_fit(&opt_fit, true)?; // logs the costs breakdown
        Ok((opt_fit, opt_cost))
    }

    fn fit_bfgs_with_guess(&self, initial_guess: &JWFit, max_iters: u64, attempt_idx: usize) -> Result<(JWFit, f64)> {
        let initial_param = initial_guess.to_param();
        let initial_cost = self.cost(&initial_param)?;
        if !Self::is_valid_cost(initial_cost) {
            return Err(anyhow!("initial JW fit cost is invalid: {initial_cost}"));
        }

        let initial_gradient = match self.gradient(&initial_param) {
            Ok(gradient) if gradient.iter().all(|v| v.is_finite()) => gradient,
            Ok(gradient) => {
                self.log(format!(
                    "WARN BFGS initial gradient is not finite; using initial JW params gradient={gradient:?}"
                ));
                return Ok((initial_guess.clone(), initial_cost));
            }
            Err(err) => {
                self.log(format!(
                    "WARN BFGS initial gradient failed; using initial JW params error={err}"
                ));
                return Ok((initial_guess.clone(), initial_cost));
            }
        };
        let gradient_norm_sq = initial_gradient.dot(&initial_gradient);
        if !gradient_norm_sq.is_finite() || gradient_norm_sq <= 1e-24 {
            return Ok((initial_guess.clone(), initial_cost));
        }

        if attempt_idx >= INV_HESSIAN_ATTEMPT_COUNT {
            self.log(format!(
                "WARN all BFGS fallback attempts exhausted; using current JW params attempt_idx={attempt_idx}, current_cost={initial_cost}, current_fit={initial_guess:?}"
            ));
            return Ok((initial_guess.clone(), initial_cost));
        }

        let attempt = match self.inv_hessian_attempt(attempt_idx, &initial_param, initial_guess) {
            Ok(attempt) => attempt,
            Err(err) => {
                self.log(format!(
                    "WARN failed to build inverse Hessian attempt; trying next fallback attempt_idx={attempt_idx}, error={err}"
                ));
                return self.fit_bfgs_with_guess(initial_guess, max_iters, attempt_idx + 1);
            }
        };

        let search_direction = attempt.inv_hessian.dot(&initial_gradient).mapv(|v| -v);
        let directional_derivative = initial_gradient.dot(&search_direction);
        if !directional_derivative.is_finite() || directional_derivative >= 0.0 {
            self.log(format!(
                "WARN skipping BFGS fallback because initial direction is not descent attempt={}, directional_derivative={directional_derivative}",
                attempt.name
            ));
            return self.fit_bfgs_with_guess(initial_guess, max_iters, attempt_idx + 1);
        }

        let attempt_name = attempt.name;
        // note not as bad to clone - only one clone here but the optimizer won't re-clone
        let output = self.clone().run_bfgs_attempt(attempt_name, &initial_param, attempt.inv_hessian, max_iters);
        match output.result {
            Ok((optimized_fit, optimized_cost)) if Self::is_valid_cost(optimized_cost) => {
                if (optimized_cost < initial_cost) || optimized_cost < 1e-12 {
                    return Ok((optimized_fit, optimized_cost));
                }
                self.log(format!(
                    "WARN BFGS did not improve cost; trying next fallback attempt={attempt_name}, initial_cost={initial_cost}, optimized_cost={optimized_cost}"
                ));
            }
            Ok((_, optimized_cost)) => {
                self.log(format!(
                    "WARN BFGS returned invalid cost; trying next fallback attempt={attempt_name}, optimized_cost={optimized_cost}"
                ));
            }
            Err(err) => {
                self.log(format!(
                    "WARN BFGS fallback failed; trying next fallback attempt={attempt_name}, error={err}"
                ));
            }
        }

        let next_guess = self
            .bfgs_retry_guess(attempt_name, output.best_param.as_ref(), initial_cost)
            .unwrap_or_else(|| initial_guess.clone());
        self.fit_bfgs_with_guess(&next_guess, max_iters, attempt_idx + 1)
    }

    pub fn arb_cost_from_fit(&self, fit: &JWFit) -> Result<f64> {
        let jw = fit.to_svi_jw(self.fwd, self.tau());
        let raw = jw.to_raw();
        self.arb_cost(&raw, true)
    }

    pub fn constraints_cost_from_fit(&self, fit: &JWFit) -> Result<f64> {
        let jw = fit.to_svi_jw(self.fwd, self.tau());
        let raw = jw.to_raw();
        self.constraints_cost(&raw, &jw)
    }

    fn bfgs_retry_guess(
        &self,
        attempt_name: &'static str,
        best_param: Option<&BfgsBestParam>,
        initial_cost: f64,
    ) -> Option<JWFit> {
        let best_param = best_param?;
        if !Self::is_valid_cost(best_param.cost) || best_param.cost >= initial_cost {
            return None;
        }

        match JWFit::try_from_param(&best_param.param) {
            Ok(fit) => {
                self.log(format!(
                    "INFO carrying BFGS observed best params into next fallback attempt={attempt_name}, initial_cost={initial_cost}, best_cost={}, best_fit={fit:?}",
                    best_param.cost
                ));
                Some(fit)
            }
            Err(err) => {
                self.log(format!(
                    "WARN BFGS observed best params are not valid JW params attempt={attempt_name}, best_cost={}, error={err}",
                    best_param.cost
                ));
                None
            }
        }
    }

    fn run_bfgs_attempt(
        self,
        attempt_name: &'static str,
        initial_param: &Array1<f64>,
        initial_inv_hessian: Array2<f64>,
        max_iters: u64,
    ) -> BfgsAttemptOutput {
        let best_param = Arc::new(Mutex::new(None));
        let observed_best = best_param.clone();
        let result = (|| -> Result<(JWFit, f64)> {
            let linesearch = MoreThuenteLineSearch::<Array1<f64>, Array1<f64>, f64>::new().with_bounds(1e-6, 0.1)?;
            let solver = BFGS::new(linesearch);
            let calibration_log = self.calibration_log.clone();

            let result = Executor::new(self, solver)
                .configure(|state| {
                    state.param(initial_param.clone()).inv_hessian(initial_inv_hessian).max_iters(max_iters)
                })
                .add_observer(
                    BfgsTracingObserver {
                        attempt: attempt_name,
                        calibration_log,
                        best_param: observed_best,
                    },
                    ObserverMode::Every(1),
                )
                .run()?;

            let (best_param, best_cost) = result
                .state
                .get_best_param()
                .map(|param| (param, result.state.get_best_cost()))
                .or_else(|| result.state.get_param().map(|param| (param, result.state.get_cost())))
                .ok_or_else(|| anyhow!("BFGS did not return a JW parameter vector"))?;

            Ok((JWFit::try_from_param(best_param)?, best_cost))
        })();

        let best_param = best_param.lock().unwrap_or_else(|p| p.into_inner()).clone();
        BfgsAttemptOutput { result, best_param }
    }

    fn inv_hessian_attempt(
        &self,
        index: usize,
        initial_param: &Array1<f64>,
        initial_guess: &JWFit,
    ) -> Result<InvHessianAttempt> {
        let (name, inv_hessian) = match index {
            0 => ("reg_diagonal", self.initial_inv_hessian(initial_param, initial_guess)?),
            1 => ("identity", Self::identity_inv_hessian(1.0)),
            2 => ("small_identity", Self::identity_inv_hessian(1e-3)),
            3 => ("numerical_full", self.numerical_inv_hessian(initial_param)?),
            _ => return Err(anyhow!("unknown inverse Hessian attempt index: {index}")),
        };

        Ok(InvHessianAttempt { name, inv_hessian })
    }

    fn initial_inv_hessian(&self, initial_param: &Array1<f64>, initial_guess: &JWFit) -> Result<Array2<f64>> {
        let mut curvatures = arr1(&[
            Self::regularization_curvature(self.cost_cfg.v_cost, initial_guess.v),
            Self::regularization_curvature(self.cost_cfg.psi_cost, 1.0),
            Self::regularization_curvature(self.cost_cfg.p_cost, initial_guess.p),
            Self::regularization_curvature(self.cost_cfg.c_cost, initial_guess.c),
            Self::regularization_curvature(self.cost_cfg.v_diff_cost, initial_guess.v_diff.min(REG_MAX_V_DIFF)),
        ]);

        let base_cost = self.cost(initial_param)?;
        if Self::is_valid_cost(base_cost) {
            for i in 0..JW_PARAM_LEN {
                let step = Self::initial_hessian_step(initial_param[i]);
                let mut p_up = initial_param.clone();
                let mut p_down = initial_param.clone();
                p_up[i] += step;
                p_down[i] -= step;

                let cost_up = self.cost(&p_up)?;
                let cost_down = self.cost(&p_down)?;
                if Self::is_valid_cost(cost_up) && Self::is_valid_cost(cost_down) {
                    let curvature = (cost_up + cost_down - 2.0 * base_cost) / step.powi(2);
                    if curvature.is_finite() && curvature > curvatures[i] {
                        curvatures[i] = curvature;
                    }
                }
            }
        }

        let diag = curvatures.mapv(Self::curvature_to_inv);
        Ok(Array2::from_diag(&diag))
    }

    fn numerical_inv_hessian(&self, initial_param: &Array1<f64>) -> Result<Array2<f64>> {
        let hessian = self.numerical_hessian(initial_param)?;
        let regularized_hessian = Self::regularize_hessian(hessian);
        let inv_hessian = Self::invert_matrix(&regularized_hessian)?;
        Ok(Self::scale_inv_hessian(inv_hessian))
    }

    fn numerical_hessian(&self, initial_param: &Array1<f64>) -> Result<Array2<f64>> {
        if initial_param.len() != JW_PARAM_LEN {
            return Err(anyhow!(
                "expected {JW_PARAM_LEN} JW params, got {}",
                initial_param.len()
            ));
        }

        let base_cost = self.cost(initial_param)?;
        if !Self::is_valid_cost(base_cost) {
            return Err(anyhow!(
                "cannot build numerical Hessian from invalid base cost: {base_cost}"
            ));
        }

        let steps = initial_param.mapv(Self::initial_hessian_step);
        let mut hessian = Array2::<f64>::zeros((JW_PARAM_LEN, JW_PARAM_LEN));

        for i in 0..JW_PARAM_LEN {
            let step = steps[i];
            let mut p_up = initial_param.clone();
            let mut p_down = initial_param.clone();
            p_up[i] += step;
            p_down[i] -= step;

            let cost_up = self.cost(&p_up)?;
            let cost_down = self.cost(&p_down)?;
            if Self::is_valid_cost(cost_up) && Self::is_valid_cost(cost_down) {
                hessian[[i, i]] = (cost_up + cost_down - 2.0 * base_cost) / step.powi(2);
            }
        }

        for i in 0..JW_PARAM_LEN {
            for j in i + 1..JW_PARAM_LEN {
                let step_i = steps[i];
                let step_j = steps[j];
                let mut p_pp = initial_param.clone();
                let mut p_pm = initial_param.clone();
                let mut p_mp = initial_param.clone();
                let mut p_mm = initial_param.clone();

                p_pp[i] += step_i;
                p_pp[j] += step_j;
                p_pm[i] += step_i;
                p_pm[j] -= step_j;
                p_mp[i] -= step_i;
                p_mp[j] += step_j;
                p_mm[i] -= step_i;
                p_mm[j] -= step_j;

                let cost_pp = self.cost(&p_pp)?;
                let cost_pm = self.cost(&p_pm)?;
                let cost_mp = self.cost(&p_mp)?;
                let cost_mm = self.cost(&p_mm)?;

                if Self::is_valid_cost(cost_pp)
                    && Self::is_valid_cost(cost_pm)
                    && Self::is_valid_cost(cost_mp)
                    && Self::is_valid_cost(cost_mm)
                {
                    let curvature = (cost_pp - cost_pm - cost_mp + cost_mm) / (4.0 * step_i * step_j);
                    hessian[[i, j]] = curvature;
                    hessian[[j, i]] = curvature;
                }
            }
        }

        Ok(hessian)
    }

    fn regularize_hessian(mut hessian: Array2<f64>) -> Array2<f64> {
        for i in 0..JW_PARAM_LEN {
            for j in i + 1..JW_PARAM_LEN {
                let left = hessian[[i, j]];
                let right = hessian[[j, i]];
                let value = if left.is_finite() && right.is_finite() {
                    0.5 * (left + right)
                } else {
                    0.0
                };
                hessian[[i, j]] = value;
                hessian[[j, i]] = value;
            }
        }

        let diagonal_floor = 1.0 / MAX_INITIAL_INV_HESSIAN;
        for i in 0..JW_PARAM_LEN {
            let mut off_diag_sum = 0.0;
            for j in 0..JW_PARAM_LEN {
                if i == j {
                    continue;
                }
                let value = hessian[[i, j]];
                if value.is_finite() {
                    off_diag_sum += value.abs();
                } else {
                    hessian[[i, j]] = 0.0;
                }
            }

            let min_diag = off_diag_sum + diagonal_floor;
            if !hessian[[i, i]].is_finite() || hessian[[i, i]] < min_diag {
                hessian[[i, i]] = min_diag;
            }
        }

        hessian
    }

    fn invert_matrix(matrix: &Array2<f64>) -> Result<Array2<f64>> {
        if matrix.nrows() != JW_PARAM_LEN || matrix.ncols() != JW_PARAM_LEN {
            return Err(anyhow!(
                "expected {JW_PARAM_LEN}x{JW_PARAM_LEN} matrix, got {}x{}",
                matrix.nrows(),
                matrix.ncols()
            ));
        }

        let matrix = SMatrix::<f64, JW_PARAM_LEN, JW_PARAM_LEN>::from_fn(|row, col| matrix[[row, col]]);
        let inverse = matrix.try_inverse().ok_or_else(|| anyhow!("numerical Hessian is singular"))?;
        let inverse = Array2::from_shape_fn((JW_PARAM_LEN, JW_PARAM_LEN), |(row, col)| inverse[(row, col)]);

        if inverse.iter().all(|v| v.is_finite()) {
            Ok(inverse)
        } else {
            Err(anyhow!("numerical inverse Hessian contains non-finite values"))
        }
    }

    fn identity_inv_hessian(scale: f64) -> Array2<f64> {
        Array2::from_diag(&Array1::from_elem(JW_PARAM_LEN, scale))
    }

    fn scale_inv_hessian(mut inv_hessian: Array2<f64>) -> Array2<f64> {
        let max_abs = inv_hessian.iter().fold(0.0_f64, |acc, value| acc.max(value.abs()));
        if !max_abs.is_finite() || max_abs <= 0.0 {
            return Self::identity_inv_hessian(1.0);
        }
        if max_abs > MAX_INITIAL_INV_HESSIAN {
            inv_hessian.mapv_inplace(|value| value * MAX_INITIAL_INV_HESSIAN / max_abs);
        }
        inv_hessian
    }

    fn is_valid_cost(cost: f64) -> bool {
        cost.is_finite() && cost < INVALID_COST
    }

    fn regularization_curvature(weight: f64, local_scale: f64) -> f64 {
        2.0 * weight * local_scale.powi(2)
    }

    fn curvature_to_inv(curvature: f64) -> f64 {
        if !curvature.is_finite() || curvature <= 0.0 {
            return 1.0;
        }
        (1.0 / curvature).clamp(MIN_INITIAL_INV_HESSIAN, MAX_INITIAL_INV_HESSIAN)
    }

    fn initial_hessian_step(value: f64) -> f64 {
        INITIAL_HESSIAN_STEP * value.abs().max(1.0)
    }

    pub fn tau(&self) -> f64 {
        let expiry_sec = (self.expiry - self.ref_sec) as f64;
        expiry_sec / SEC_PER_YEAR
    }

    pub fn dte(&self) -> f64 {
        self.tau() * 365.
    }

    pub fn get_liquidity_score(&self) -> f64 {
        let tau_sqrt = self.tau().sqrt();
        if !self.fwd.is_finite() || self.fwd <= 0.0 || !tau_sqrt.is_finite() || tau_sqrt <= 0.0 {
            return LIQ_SCORE_COUNT as f64;
        }

        let mut spreads_by_distance: Vec<(f64, f64)> = self
            .contracts
            .iter()
            .zip(self.bids.iter().zip(self.asks.iter()))
            .filter_map(|(contract, (&bid, &ask))| {
                let distance_to_fwd = (contract.strike - self.fwd).abs();
                let spread_over_fwd = ((ask - bid) / self.fwd).clamp(0.0, 1.0);
                let normalized_spread = spread_over_fwd / tau_sqrt;

                if distance_to_fwd.is_finite() && normalized_spread.is_finite() {
                    Some((distance_to_fwd, normalized_spread))
                } else {
                    None
                }
            })
            .collect();

        spreads_by_distance.sort_by(|left, right| left.0.total_cmp(&right.0));
        let mut closest_spreads: Vec<f64> =
            spreads_by_distance.into_iter().take(LIQ_SCORE_COUNT * 2).map(|(_, spread)| spread).collect();
        closest_spreads.sort_by(|left, right| left.total_cmp(right));

        let valid_spread_count = closest_spreads.len();
        let missing_penalty = LIQ_SCORE_COUNT.saturating_sub(valid_spread_count) as f64 / tau_sqrt;
        closest_spreads.into_iter().take(LIQ_SCORE_COUNT).sum::<f64>() + missing_penalty
    }

    pub fn bbo_cost_from_fit(&self, fit: &JWFit, verbose: bool) -> Result<f64> {
        let jw = fit.to_svi_jw(self.fwd, self.tau());
        let raw = jw.to_raw();
        self.bbo_cost(&raw, verbose)
    }

    fn bbo_cost(&self, raw: &SVIRawParams, verbose: bool) -> Result<f64> {
        let cost: f64 = match self.bbo_residual_costs(raw, verbose) {
            Ok(residuals) => residuals.into_iter().sum(),
            Err(_) => INVALID_COST,
        };
        Ok(if cost.is_finite() { cost } else { INVALID_COST })
    }

    pub fn bbo_residual_costs_from_fit(&self, fit: &JWFit, verbose: bool) -> Result<Vec<f64>> {
        let jw = fit.to_svi_jw(self.fwd, self.tau());
        let raw = jw.to_raw();
        self.bbo_residual_costs(&raw, verbose)
    }

    fn bbo_residual_costs(&self, raw: &SVIRawParams, verbose: bool) -> Result<Vec<f64>> {
        if !raw.a.is_finite()
            || !raw.b.is_finite()
            || !raw.rho.is_finite()
            || !raw.m.is_finite()
            || !raw.sigma.is_finite()
        {
            return Err(anyhow!("SVI raw params contain non-finite values"));
        }

        let mut residuals = Vec::with_capacity(self.contracts.len());
        for (i, c) in self.contracts.iter().enumerate() {
            let iv = match raw.get_vol_result(c.strike) {
                Ok(iv) => iv,
                Err(_) => {
                    residuals.push(0.0);
                    continue;
                }
            };
            if !iv.is_finite() {
                residuals.push(0.0);
                continue;
            }
            let px = c.price(raw.fwd, iv) * self.discount;
            if !px.is_finite() {
                return Err(anyhow!("BBO residual price is not finite"));
            }
            let bid_resid = smooth_pos(self.bids[i] - px, self.cost_cfg.smoothing_eps);
            let ask_resid = smooth_pos(px - self.asks[i], self.cost_cfg.smoothing_eps);
            let relative_resid = (bid_resid + ask_resid) / self.fwd;
            let residual_cost = relative_resid.powi(2) * self.cost_cfg.bbo_cost;
            if !residual_cost.is_finite() {
                return Err(anyhow!("BBO residual cost is not finite"));
            }
            residuals.push(residual_cost);
            if verbose && (bid_resid > 0.0 || ask_resid > 0.0) {
                let ask_f = if self.asks[i] == f64::MAX { 0.0 } else { self.asks[i] };
                self.log(format!(
                    "INFO bbo_residual_costs strike={}, bid={}, ask={}, price={}, bid_resid={}, ask_resid={}, residual_cost={}",
                    c.strike, self.bids[i], ask_f, px, bid_resid, ask_resid, residual_cost
                ));
            }
        }
        Ok(residuals)
    }

    fn regularization_cost(&self, jwp: &SVIJWParams) -> Result<f64> {
        let mut cost: f64 = 0.0;
        cost += (jwp.v - self.init_svi.v).powi(2) * self.cost_cfg.v_cost;
        cost += (jwp.psi - self.init_svi.psi).powi(2) * self.cost_cfg.psi_cost;
        cost += (jwp.p - self.init_svi.p).powi(2) * self.cost_cfg.p_cost;
        cost += (jwp.c - self.init_svi.c).powi(2) * self.cost_cfg.c_cost;
        cost += (jwp.v_diff - self.init_svi.v_diff.min(REG_MAX_V_DIFF)).powi(2) * self.cost_cfg.v_diff_cost;

        Ok(if cost.is_finite() { cost } else { INVALID_COST })
    }
    fn constraints_cost(&self, raw: &SVIRawParams, jwp: &SVIJWParams) -> Result<f64> {
        let mut cost: f64 = 0.0;
        // abs(rho) <= 1.0
        cost += smooth_pos(raw.rho.abs() - 1.0, self.cost_cfg.smoothing_eps);
        // b > 0 and sigma > 0
        cost += smooth_pos(-raw.b, self.cost_cfg.smoothing_eps);
        cost += smooth_pos(-raw.sigma, self.cost_cfg.smoothing_eps);
        // smile convexity
        cost += smooth_pos(-jwp.p - 2.0 * jwp.psi, self.cost_cfg.smoothing_eps);
        cost += smooth_pos(2.0 * jwp.psi - jwp.c, self.cost_cfg.smoothing_eps);
        // iv positivity
        if raw.rho.abs() <= 1.0 {
            let pos_iv_cond = raw.a + raw.b * raw.sigma * (1.0 - raw.rho.powi(2)).sqrt();
            cost += smooth_pos(-pos_iv_cond, self.cost_cfg.smoothing_eps);
        }
        Ok(if cost.is_finite() {
            cost * self.cost_cfg.constraints_cost
        } else {
            INVALID_COST
        })
    }
    fn arb_cost(&self, raw: &SVIRawParams, verbose: bool) -> Result<f64> {
        if self.cost_cfg.arb_cost == 0.0 {
            return Ok(0.0);
        }

        let mut cost: f64 = 0.0;
        let std_ks = Array1::linspace(-4.0, 4.0, 50);
        if !raw.reftau.is_finite() || raw.reftau <= 0.0 {
            return Ok(INVALID_COST);
        }

        // override fwd with a "standard" 100.0 to normalize cost across currencies
        let raw = raw.with_fwd(100.0);
        let atm = raw.get_vol_result(100.0);
        let atm = match atm {
            Ok(v) => v,
            Err(_) => return Ok(INVALID_COST),
        };

        let ks = std_ks.mapv(|std_k| (std_k * atm * raw.reftau.sqrt()).exp() * raw.fwd);
        let expiry_sec = raw.reftau * SEC_PER_YEAR;
        let mut call_prices = Vec::with_capacity(ks.len());

        for &strike in ks.iter() {
            let iv = match raw.get_vol_result(strike) {
                Ok(iv) if iv.is_finite() => iv,
                _ => return Ok(0.0),
            };

            let contract = OptionContract {
                strike,
                expiry_sec,
                is_call: true,
            };
            // note suuuuuper deep otm gets f64 cancellation issues and may be -ve
            let px = (contract.price(raw.fwd, iv) * self.discount).max(0.0);
            let px = if px < ARB_COST_ROUND { 0.0 } else { px };
            if !px.is_finite() {
                return Ok(INVALID_COST);
            }
            call_prices.push(px);
        }

        for i in 0..call_prices.len() - 1 {
            let c = smooth_pos(call_prices[i + 1] - call_prices[i], self.cost_cfg.smoothing_eps);
            if c > ARB_COST_ROUND && verbose {
                self.log(format!(
                    "WARN arb_cost: call price is decreasing between strikes {} and {}: {} < {}",
                    ks[i],
                    ks[i + 1],
                    call_prices[i],
                    call_prices[i + 1]
                ));
            }
            if c > ARB_COST_ROUND {
                cost += c;
            }
        }

        for i in 0..call_prices.len() - 2 {
            // call descent already checked so can floor at 0.0 to avoid f64 cancellations
            let dc_i = (call_prices[i] - call_prices[i + 1]).max(0.0);
            let dc_next = (call_prices[i + 1] - call_prices[i + 2]).max(0.0);

            let dc_next = if dc_next < ARB_COST_ROUND { 0.0 } else { dc_next };

            let d_i = dc_i / (ks[i + 1] - ks[i]);
            let d_next = dc_next / (ks[i + 2] - ks[i + 1]);

            let c = smooth_pos(d_next - d_i, self.cost_cfg.smoothing_eps);
            if c > ARB_COST_ROUND && verbose {
                self.log(format!(
                    "WARN arb_cost: call price is not convex between strikes {}, {}, {}: {} > {}",
                    ks[i],
                    ks[i + 1],
                    ks[i + 2],
                    d_next,
                    d_i
                ));
            }
            if c > ARB_COST_ROUND {
                cost += c;
            }
        }

        let cost = if cost > ARB_COST_ROUND { cost } else { 0.0 };

        Ok(if cost.is_finite() {
            cost * self.cost_cfg.arb_cost
        } else {
            INVALID_COST
        })
    }
    fn cost_from_fit(&self, p: &JWFit, verbose: bool) -> Result<f64> {
        let jw_new = p.to_svi_jw(self.fwd, self.tau());
        let raw = jw_new.to_raw();
        let bbo_cost = self.bbo_cost(&raw, verbose)?;
        let constraints_cost = self.constraints_cost(&raw, &jw_new)?;
        let arb_cost = if self.cost_cfg.arb_cost == 0.0 {
            0.0
        } else {
            self.arb_cost(&raw, false)?
        };
        let reg_cost = self.regularization_cost(&jw_new)?;
        if verbose {
            self.log(format!(
                "cost_from_fit: bbo_cost: {}, reg_cost: {}, constraints_cost: {}, arb_cost: {}",
                bbo_cost, reg_cost, constraints_cost, arb_cost
            ));
        }
        Ok(bbo_cost + reg_cost + constraints_cost + arb_cost)
    }
}

impl CostFunction for SliceFitter {
    type Param = Array1<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output> {
        let fit = JWFit::try_from_param(p)?;
        self.cost_from_fit(&fit, false)
    }
}

impl Gradient for SliceFitter {
    type Param = Array1<f64>;
    type Gradient = Array1<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient> {
        if p.len() != JW_PARAM_LEN {
            return Err(anyhow!("expected {JW_PARAM_LEN} JW params, got {}", p.len()));
        }

        let mut grad = Array1::<f64>::zeros(JW_PARAM_LEN);
        for i in 0..JW_PARAM_LEN {
            let step = JWFit::finite_diff_step(p[i]);
            let mut p_up = p.clone();
            let mut p_down = p.clone();
            p_up[i] += step;
            p_down[i] -= step;

            let cost_up = self.cost(&p_up)?;
            let cost_down = self.cost(&p_down)?;
            grad[i] = (cost_up - cost_down) / (2.0 * step);
        }

        Ok(grad)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    struct MockFitterFixture {
        fitter: SliceFitter,
        initial_guess: JWFit,
        fwd: f64,
        tau: f64,
    }

    fn mock_fitter_fixture() -> MockFitterFixture {
        let fwd = 67.0;
        let discount = 1.0;
        let now = chrono::Utc::now();
        let expiry = (now + Duration::days(30)).timestamp();
        let tau = (expiry - now.timestamp()) as f64 / SEC_PER_YEAR;

        let initial_guess = JWFit {
            v: 1.5,
            psi: -0.001,
            p: 0.51,
            c: 0.49,
            v_diff: 0.001,
        };

        let cost_cfg = default_cost_cfg();
        let now = chrono::Utc::now().timestamp();
        let init_svi = initial_guess.to_svi_jw(fwd, tau);

        let mut bids = Array1::<f64>::zeros(10);
        bids[0] = 3.18;
        bids[1] = 3.66;
        bids[2] = 4.50;
        bids[3] = 4.94;
        bids[4] = 5.40;
        bids[5] = 5.80;
        bids[6] = 4.75;
        bids[7] = 4.01;
        bids[8] = 3.71;
        bids[9] = 3.43;

        let mut asks = Array1::<f64>::zeros(10);
        asks[0] = 4.34;
        asks[1] = 5.18;
        asks[2] = 6.12;
        asks[3] = 6.63;
        asks[4] = 7.16;
        asks[5] = 7.28;
        asks[6] = 6.45;
        asks[7] = 5.70;
        asks[8] = 5.08;
        asks[9] = 4.77;

        let mut strikes = Array1::<f64>::zeros(10);
        strikes[0] = 60.0;
        strikes[1] = 62.0;
        strikes[2] = 64.0;
        strikes[3] = 65.0;
        strikes[4] = 66.0;
        strikes[5] = 68.0;
        strikes[6] = 70.0;
        strikes[7] = 72.0;
        strikes[8] = 74.0;
        strikes[9] = 75.0;

        let mut is_calls = Array1::from_elem(10, false);
        is_calls[0] = false;
        is_calls[1] = false;
        is_calls[2] = false;
        is_calls[3] = false;
        is_calls[4] = false;
        is_calls[5] = true;
        is_calls[6] = true;
        is_calls[7] = true;
        is_calls[8] = true;
        is_calls[9] = true;
        let fitter = SliceFitter::new(cost_cfg, init_svi, expiry, fwd, discount, now)
            .build_contracts(bids, asks, strikes, is_calls);

        MockFitterFixture {
            fitter,
            initial_guess,
            fwd,
            tau,
        }
    }

    fn assert_fit_approx(actual: &JWFit, expected: &JWFit, tol: f64) {
        assert!((actual.v - expected.v).abs() < tol);
        assert!((actual.psi - expected.psi).abs() < tol);
        assert!((actual.p - expected.p).abs() < tol);
        assert!((actual.c - expected.c).abs() < tol);
        assert!((actual.v_diff - expected.v_diff).abs() < tol);
    }

    #[test]
    fn get_interpolated_handles_empty_and_singleton_surfaces() {
        let now = 1_000;
        let empty = HashMap::new();
        assert!(JWFit::get_interpolated_at(&empty, now + 20, now).is_none());

        let fit = JWFit {
            v: 1.0,
            psi: -0.1,
            p: 0.2,
            c: 0.3,
            v_diff: 0.4,
        };
        let singleton = HashMap::from([(now + 10, fit.clone())]);
        let interpolated =
            JWFit::get_interpolated_at(&singleton, now + 20, now).expect("singleton surface returns only fit");
        assert_fit_approx(&interpolated, &fit, 1e-12);
    }

    #[test]
    fn get_interpolated_uses_total_variance_interpolation() {
        let now = 1_000;
        let lower = JWFit {
            v: 1.0,
            psi: -0.1,
            p: 0.2,
            c: 0.3,
            v_diff: 0.4,
        };
        let upper = JWFit {
            v: 3.0,
            psi: -0.3,
            p: 0.6,
            c: 0.9,
            v_diff: 1.2,
        };
        let surface = HashMap::from([(now + 10, lower), (now + 30, upper)]);

        let interpolated = JWFit::get_interpolated_at(&surface, now + 20, now).expect("expiry is bracketed");
        let expected = JWFit {
            v: 2.5,
            psi: -0.25,
            p: 0.5,
            c: 0.75,
            v_diff: 1.0,
        };

        assert_fit_approx(&interpolated, &expected, 1e-12);
    }

    #[test]
    fn get_interpolated_returns_exact_match_and_none_outside_bounds() {
        let now = 1_000;
        let lower = JWFit {
            v: 1.0,
            psi: -0.1,
            p: 0.2,
            c: 0.3,
            v_diff: 0.4,
        };
        let upper = JWFit {
            v: 3.0,
            psi: -0.3,
            p: 0.6,
            c: 0.9,
            v_diff: 1.2,
        };
        let surface = HashMap::from([(now + 10, lower.clone()), (now + 30, upper)]);

        let exact = JWFit::get_interpolated_at(&surface, now + 10, now).expect("exact expiry returns exact fit");
        assert_fit_approx(&exact, &lower, 1e-12);
        assert!(JWFit::get_interpolated_at(&surface, now + 5, now).is_none());
        assert!(JWFit::get_interpolated_at(&surface, now + 35, now).is_none());
    }

    #[test]
    fn bbo_cost_is_invariant_to_forward_price_scale() {
        fn cost_for_forward(fwd: f64) -> f64 {
            let ref_sec = 1_000_000;
            let expiry_sec = 30.0 * 24.0 * 60.0 * 60.0;
            let expiry = ref_sec + expiry_sec as i64;
            let tau = expiry_sec / SEC_PER_YEAR;
            let strike = fwd;
            let fit = JWFit {
                v: 1.5,
                psi: -0.001,
                p: 0.51,
                c: 0.49,
                v_diff: 0.001,
            };
            let init_svi = fit.to_svi_jw(fwd, tau);
            let raw = init_svi.to_raw();
            let contract = OptionContract {
                strike,
                expiry_sec,
                is_call: true,
            };
            let iv = raw.get_vol_result(strike).expect("calculate ATM volatility");
            let model_price = contract.price(fwd, iv);
            let bid = model_price + 0.01 * fwd;
            let fitter = SliceFitter::new(default_cost_cfg(), init_svi, expiry, fwd, 1.0, ref_sec).build_contracts(
                arr1(&[bid]),
                arr1(&[f64::MAX]),
                arr1(&[strike]),
                Array1::from_elem(1, true),
            );

            fitter.bbo_cost_from_fit(&fit, false).expect("calculate BBO cost")
        }

        let low_forward_cost = cost_for_forward(2_000.0);
        let high_forward_cost = cost_for_forward(60_000.0);

        assert!((low_forward_cost - 1.0).abs() < 1e-10);
        assert!((high_forward_cost - 1.0).abs() < 1e-10);
        assert!((low_forward_cost - high_forward_cost).abs() < 1e-10);
    }

    #[test]
    fn test_arb_cost() {
        let MockFitterFixture {
            fitter,
            initial_guess,
            fwd,
            tau,
        } = mock_fitter_fixture();
        let mut raw = initial_guess.to_svi_jw(fwd, tau).to_raw();
        raw.a = -0.0410;
        raw.b = 0.1331;
        raw.m = 0.3586;
        raw.rho = 0.3060;
        raw.sigma = 0.4153;

        let arb_cost = fitter.arb_cost(&raw, true).expect("calculate mock arb cost");
        println!("mock JW fit arb cost: {arb_cost}");

        assert!(arb_cost > 0.0);
        let jw = SVIJWParams::from_raw(&raw);
        println!("{:?}", jw);
        let fitter_bfgs = fitter.clone();
        let (optimized_fit, optimized_cost) = fitter_bfgs.with_init_svi(jw).fit_bfgs(1000).expect("run BFGS");
        let optimized_raw = optimized_fit.to_svi_jw(fwd, tau).to_raw();
        let optimized_arb_cost = fitter.arb_cost(&optimized_raw, true).expect("calculate optimized mock arb cost");
        println!("optimized mock JW fit cost: {optimized_cost}");
        println!("optimized JW fit: {optimized_fit:?}");
        println!("optimized mock JW fit arb cost: {optimized_arb_cost}");

        assert!(optimized_arb_cost < arb_cost);
    }

    #[test]
    fn fit_bfgs_unarbs_with_empty_books() {
        let MockFitterFixture {
            mut fitter,
            initial_guess,
            fwd,
            tau,
        } = mock_fitter_fixture();
        let mut raw = initial_guess.to_svi_jw(fwd, tau).to_raw();
        raw.a = -0.0410;
        raw.b = 0.1331;
        raw.m = 0.3586;
        raw.rho = 0.3060;
        raw.sigma = 0.4153;

        fitter = fitter.with_init_svi(SVIJWParams::from_raw(&raw));
        let liquidity_score = fitter.get_liquidity_score();
        assert!(liquidity_score > 0.0);
        fitter.remove_outliers(1, 10.0, liquidity_score / 2.0).expect("clear illiquid books");
        assert!(fitter.bids.is_empty());
        assert!(fitter.asks.is_empty());
        assert!(fitter.strikes.is_empty());
        assert!(fitter.contracts.is_empty());

        let initial_fit = fitter.init_fit();
        let initial_arb_cost = fitter.arb_cost_from_fit(&initial_fit).expect("calculate initial arb cost");
        assert!(initial_arb_cost > 0.0);

        let (optimized_fit, _) = fitter.fit_bfgs(1000).expect("fit empty books");
        let optimized_arb_cost = fitter.arb_cost_from_fit(&optimized_fit).expect("calculate optimized arb cost");
        assert_eq!(optimized_arb_cost, 0.0);
    }

    #[test]
    fn print_xaut_50_64_dte_call_prices() {
        // The calibration log's initial SVI has fwd=0, but arb_cost_from_fit replaces
        // fwd and reftau with the SliceFitter values. These are reconstructed from
        // the logged 100-point strike grid for the 50.64 DTE slice.
        let expiry_sec = 4_375_288.0;
        let tau = expiry_sec / SEC_PER_YEAR;
        let fwd = 4255.961507535542;
        let discount = (-0.03 * tau).exp();
        let svi = SVIJWParams {
            v: 0.04574385033901009,
            psi: -0.025773773026465427,
            p: 0.5449816978067556,
            c: 0.24343654302773765,
            v_diff: 0.0003134275128493236,
            fwd,
            reftau: tau,
        };
        let raw = svi.to_raw();
        let strikes = [
            15585.613636684662,
            16183.177451282267,
            16803.65229914684,
            17447.916606034007,
            18116.882477184678,
            18811.49698862111,
            19532.743527952287,
        ];

        println!("fwd={fwd:.15}, tau={tau:.17}, discount={discount:.17}");
        println!("raw_svi={raw:?}");
        for strike in strikes {
            let iv = raw.get_vol_result(strike).expect("calculate logged XAUT SVI volatility");
            let contract = OptionContract {
                strike,
                expiry_sec,
                is_call: true,
            };
            let call_price = contract.price(fwd, iv);
            println!(
                "strike={strike:.15}, iv={iv:.17}, call_price={call_price:.17e}, discounted_call_price={:.17e}",
                call_price * discount
            );
        }
    }

    #[test]
    fn calculate_and_print_mock_cost() {
        let MockFitterFixture {
            fitter,
            initial_guess,
            fwd,
            tau,
        } = mock_fitter_fixture();

        let liq = fitter.get_liquidity_score();
        println!("liq={liq:.9} at tau {tau:.9}");

        let cost = fitter.cost_from_fit(&initial_guess, true).expect("calculate mock cost");
        println!("mock JW fit cost: {cost}");

        let gradient = fitter.gradient(&initial_guess.to_param()).expect("calculate mock gradient");
        println!("mock JW fit gradient: {gradient:?}");

        let fitter_bfgs = fitter.clone();
        let (optimized_fit, optimized_cost) = fitter_bfgs.fit_bfgs(1000).expect("run BFGS");
        println!("optimized mock JW fit cost: {optimized_cost}");

        assert!(optimized_cost < cost);
        println!("optimized JW fit: {optimized_fit:?}");
        let optimized_jw = optimized_fit.to_svi_jw(fwd, tau).to_raw();

        let bbo_cost = fitter.bbo_cost(&optimized_jw, false).expect("calculate optimized BBO cost");
        println!("optimized BBO cost: {bbo_cost}");
    }
}
