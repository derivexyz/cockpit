use crate::fitters::slice_fitter::{CalibrationLog, JWFitCostConfig, SliceFitter, LIQ_SCORE_COUNT};
use crate::fitters::svi::SVIJWParams;
use anyhow::{anyhow, bail, Result};
use ndarray::Array1;
use std::collections::HashMap;

const SLICE_FIT_MAX_ITERS: u64 = 500;

pub struct SurfaceFitter {
    ref_sec: i64,
    forwards: HashMap<i64, f64>,              // expiry (sec) -> forward price
    disc_factors: HashMap<i64, f64>,          // expiry (sec) -> discount factor applied to option prices
    cost_cfgs: HashMap<i64, JWFitCostConfig>, // expiry (sec) -> cost config its slice is fitted with
    slice_fitters: HashMap<i64, SliceFitter>, // expiry (sec) -> slice fitter
    calibration_log: CalibrationLog,
}

impl SurfaceFitter {
    /// `ref_sec` is the reference the slices measure tau from; `forwards` and `disc_factors`
    /// must be struck off that same reference. Every slice is fitted with `cost_cfg`.
    pub fn new(
        ref_sec: i64,
        forwards: HashMap<i64, f64>,
        disc_factors: HashMap<i64, f64>,
        cost_cfg: JWFitCostConfig,
    ) -> Self {
        let cost_cfgs = forwards.keys().map(|&expiry| (expiry, cost_cfg.clone())).collect();
        Self::new_with_cfgs(ref_sec, forwards, disc_factors, cost_cfgs)
    }

    /// As [`SurfaceFitter::new`], with a cost config per expiry (sec), e.g. to weigh the
    /// parameter pulls by time to expiry.
    pub fn new_with_cfgs(
        ref_sec: i64,
        forwards: HashMap<i64, f64>,
        disc_factors: HashMap<i64, f64>,
        cost_cfgs: HashMap<i64, JWFitCostConfig>,
    ) -> Self {
        Self {
            ref_sec,
            forwards,
            disc_factors,
            cost_cfgs,
            slice_fitters: HashMap::new(),
            calibration_log: CalibrationLog::new(),
        }
    }

    /// expiry (sec) -> slice fitter, exposed so callers can report per-slice diagnostics.
    pub fn slices(&self) -> &HashMap<i64, SliceFitter> {
        &self.slice_fitters
    }

    pub fn with_calibration_log(mut self, calibration_log: CalibrationLog) -> Self {
        self.calibration_log = calibration_log;
        self
    }

    fn log(&self, line: impl Into<String>) {
        self.calibration_log.push(line);
    }

    /// expiry (sec) -> fitted JW params.
    pub fn into_svi_jw_params(self) -> HashMap<i64, SVIJWParams> {
        self.slice_fitters.into_iter().map(|(expiry, fitter)| (expiry, fitter.init_svi())).collect()
    }

    /// expiry (ms) -> fitted JW params, for the Haruko store which keys surfaces in milliseconds.
    pub fn into_svi_jw_params_ms(self) -> HashMap<i64, SVIJWParams> {
        self.into_svi_jw_params().into_iter().map(|(expiry, svi)| (expiry * 1000, svi)).collect()
    }

    pub fn build_slices(
        mut self,
        mut init_svis: HashMap<i64, SVIJWParams>,
        bids: Array1<f64>,
        asks: Array1<f64>,
        strikes: Array1<f64>,
        expiries: Array1<i64>,
        is_calls: Array1<bool>,
    ) -> Result<Self> {
        if !(bids.len() == asks.len()
            && asks.len() == strikes.len()
            && strikes.len() == expiries.len()
            && expiries.len() == is_calls.len())
        {
            bail!(
                "quote arrays must be the same length, got bids={}, asks={}, strikes={}, expiries={}, is_calls={}",
                bids.len(),
                asks.len(),
                strikes.len(),
                expiries.len(),
                is_calls.len()
            );
        }

        self.slice_fitters.clear();

        let mut indices_by_expiry: HashMap<i64, Vec<usize>> = HashMap::new();
        for (idx, &expiry) in expiries.iter().enumerate() {
            indices_by_expiry.entry(expiry).or_default().push(idx);
        }

        let quoted: Vec<i64> = indices_by_expiry.keys().copied().collect();
        self.check_expiries_covered(&quoted, &init_svis)?;

        for (expiry, indices) in indices_by_expiry {
            let fwd = self.forwards[&expiry];
            let discount = self.disc_factors[&expiry];
            let init_svi = init_svis.remove(&expiry).expect("init SVI presence is checked above");

            let slice_bids = Array1::from_iter(indices.iter().map(|&idx| bids[idx]));
            let slice_asks = Array1::from_iter(indices.iter().map(|&idx| asks[idx]));
            let slice_strikes = Array1::from_iter(indices.iter().map(|&idx| strikes[idx]));
            let slice_is_calls = Array1::from_iter(indices.iter().map(|&idx| is_calls[idx]));

            let cost_cfg = self.cost_cfgs[&expiry].clone();
            let mut slice_fitter = SliceFitter::new(cost_cfg, init_svi, expiry, fwd, discount, self.ref_sec);
            slice_fitter = slice_fitter.with_calibration_log(self.calibration_log.clone());
            let slice_fitter = slice_fitter.build_contracts(slice_bids, slice_asks, slice_strikes, slice_is_calls);
            self.slice_fitters.insert(expiry, slice_fitter);
        }

        Ok(self)
    }

    /// Every quoted expiry needs a forward, a discount factor, a cost config and an init SVI.
    /// Extra entries in those maps are ignored; missing ones are a caller bug, reported all at once.
    fn check_expiries_covered(&self, quoted: &[i64], init_svis: &HashMap<i64, SVIJWParams>) -> Result<()> {
        fn missing(quoted: &[i64], present: impl Fn(i64) -> bool) -> Vec<i64> {
            let mut absent: Vec<i64> = quoted.iter().copied().filter(|expiry| !present(*expiry)).collect();
            absent.sort_unstable();
            absent
        }

        let problems: Vec<String> = [
            ("forward", missing(quoted, |expiry| self.forwards.contains_key(&expiry))),
            (
                "discount factor",
                missing(quoted, |expiry| self.disc_factors.contains_key(&expiry)),
            ),
            (
                "cost config",
                missing(quoted, |expiry| self.cost_cfgs.contains_key(&expiry)),
            ),
            ("init SVI", missing(quoted, |expiry| init_svis.contains_key(&expiry))),
        ]
        .into_iter()
        .filter(|(_, absent)| !absent.is_empty())
        .map(|(name, absent)| format!("{name} missing for expiries {absent:?}"))
        .collect();

        if problems.is_empty() {
            Ok(())
        } else {
            bail!(
                "quoted expiries do not match the fitter inputs: {}",
                problems.join("; ")
            )
        }
    }

    pub fn best_liq(&self) -> Option<i64> {
        self.slice_fitters
            .iter()
            .filter_map(|(&expiry, fitter)| {
                let tau = fitter.tau();
                let score = fitter.get_liquidity_score();
                (score * tau.sqrt() < (LIQ_SCORE_COUNT as f64) * 0.5).then_some((expiry, score))
            })
            .min_by(|(left_expiry, left_score), (right_expiry, right_score)| {
                left_score.total_cmp(right_score).then_with(|| left_expiry.cmp(right_expiry))
            })
            .map(|(expiry, _)| expiry)
    }

    pub fn remove_outliers(mut self, n: usize, factor: f64, min_score: f64) -> Result<Self> {
        for fitter in self.slice_fitters.values_mut() {
            fitter.remove_outliers(n, factor, min_score)?;
        }
        Ok(self)
    }

    /// Slices are fitted independently of one another, so the order only decides the shape of the
    /// calibration log. Longest dated first keeps that log reading from the stablest slice down.
    pub fn fit_slices(mut self) -> Result<Self> {
        if self.slice_fitters.is_empty() {
            self.log("WARN empty liquidity_scores");
            return Ok(self);
        }

        let mut expiries: Vec<i64> = self.slice_fitters.keys().copied().collect();
        expiries.sort_unstable_by(|left, right| right.cmp(left));

        let log = self.calibration_log.clone();
        for expiry in expiries {
            let fitter = self
                .slice_fitters
                .get_mut(&expiry)
                .ok_or_else(|| anyhow!("missing slice fitter for expiry {expiry}"))?;

            log.push(format!(
                "INFO Fitting {:.2} DTE with init {:?}",
                fitter.dte(),
                fitter.init_svi()
            ));
            let (fit, _) = fitter.fit_bfgs(SLICE_FIT_MAX_ITERS)?;
            log.push(format!("INFO Fit {:.2} DTE with {:?}", fitter.dte(), fit));
            fitter.set_init_fit(&fit);
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::black76::OptionContract;
    use crate::fitters::slice_fitter::{default_cost_cfg, JWFit};
    use crate::SEC_PER_YEAR;

    const REF_SEC: i64 = 1_000_000;
    const FWD: f64 = 67.0;
    const DAY: i64 = 24 * 60 * 60;

    fn tau(expiry: i64) -> f64 {
        (expiry - REF_SEC) as f64 / SEC_PER_YEAR
    }

    fn init_svi(expiry: i64) -> SVIJWParams {
        JWFit {
            v: 1.5,
            psi: -0.001,
            p: 0.51,
            c: 0.49,
            v_diff: 0.001,
        }
        .to_svi_jw(FWD, tau(expiry))
    }

    fn fitter(expiries: &[i64], cost_cfgs: Option<HashMap<i64, JWFitCostConfig>>) -> SurfaceFitter {
        let forwards = expiries.iter().map(|&expiry| (expiry, FWD)).collect();
        let disc_factors = expiries.iter().map(|&expiry| (expiry, 1.0)).collect();
        match cost_cfgs {
            Some(cost_cfgs) => SurfaceFitter::new_with_cfgs(REF_SEC, forwards, disc_factors, cost_cfgs),
            None => SurfaceFitter::new(REF_SEC, forwards, disc_factors, default_cost_cfg()),
        }
    }

    /// One ATM call per expiry, bid 1% of the forward above its init smile's price, so a fit
    /// has to leave the init to get inside the book.
    fn build(fitter: SurfaceFitter, expiries: &[i64]) -> Result<SurfaceFitter> {
        let init_svis: HashMap<i64, SVIJWParams> = expiries.iter().map(|&expiry| (expiry, init_svi(expiry))).collect();
        let bids = expiries
            .iter()
            .map(|expiry| {
                let iv = init_svis[expiry].to_raw().get_vol_result(FWD).expect("ATM vol");
                let contract = OptionContract {
                    strike: FWD,
                    expiry_sec: (expiry - REF_SEC) as f64,
                    is_call: true,
                };
                contract.price(FWD, iv) + 0.01 * FWD
            })
            .collect();
        let n = expiries.len();
        fitter.build_slices(
            init_svis,
            Array1::from_vec(bids),
            Array1::from_elem(n, f64::MAX),
            Array1::from_elem(n, FWD),
            Array1::from(expiries.to_vec()),
            Array1::from_elem(n, true),
        )
    }

    /// The point of per-expiry configs: weaker parameter pulls on one slice let the book move
    /// that slice, and only that one.
    #[test]
    fn each_slice_is_fitted_with_its_own_cost_config() -> Result<()> {
        let (held, free) = (REF_SEC + 30 * DAY, REF_SEC + 31 * DAY);
        let mut weak_pulls = default_cost_cfg();
        for pull in [
            &mut weak_pulls.v_cost,
            &mut weak_pulls.psi_cost,
            &mut weak_pulls.p_cost,
            &mut weak_pulls.c_cost,
            &mut weak_pulls.v_diff_cost,
        ] {
            *pull *= 1e-6;
        }
        let cost_cfgs = HashMap::from([(held, default_cost_cfg()), (free, weak_pulls)]);
        let fitted = build(fitter(&[held, free], Some(cost_cfgs)), &[held, free])?.fit_slices()?;

        let bbo_cost = |expiry: i64| {
            let slice = &fitted.slices()[&expiry];
            slice.bbo_cost_from_fit(&slice.init_fit(), false).expect("bbo cost")
        };
        assert!(
            bbo_cost(free) < 0.01 * bbo_cost(held),
            "weak pulls fit the book, default ones hold the init: {} vs {}",
            bbo_cost(free),
            bbo_cost(held)
        );
        Ok(())
    }

    /// `new` keeps every existing caller working: one config covers every forward's expiry.
    #[test]
    fn a_quoted_expiry_needs_a_cost_config() {
        let (covered, uncovered) = (REF_SEC + 30 * DAY, REF_SEC + 31 * DAY);
        assert!(build(fitter(&[covered, uncovered], None), &[covered, uncovered]).is_ok());

        let cost_cfgs = HashMap::from([(covered, default_cost_cfg())]);
        let error = build(fitter(&[covered, uncovered], Some(cost_cfgs)), &[covered, uncovered])
            .err()
            .expect("an expiry without a cost config is a caller bug");
        assert!(
            error.to_string().contains(&format!("cost config missing for expiries [{uncovered}]")),
            "{error}"
        );
    }
}
