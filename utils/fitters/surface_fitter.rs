use crate::fitters::slice_fitter::{CalibrationLog, JWFitCostConfig, SliceFitter, LIQ_SCORE_COUNT};
use crate::fitters::svi::SVIJWParams;
use anyhow::{anyhow, bail, Result};
use ndarray::Array1;
use std::collections::HashMap;

const SLICE_FIT_MAX_ITERS: u64 = 500;

pub struct SurfaceFitter {
    ref_sec: i64,
    forwards: HashMap<i64, f64>,     // expiry (sec) -> forward price
    disc_factors: HashMap<i64, f64>, // expiry (sec) -> discount factor applied to option prices
    cost_cfg: JWFitCostConfig,
    slice_fitters: HashMap<i64, SliceFitter>, // expiry (sec) -> slice fitter
    calibration_log: CalibrationLog,
}

impl SurfaceFitter {
    /// `ref_sec` is the reference the slices measure tau from; `forwards` and `disc_factors`
    /// must be struck off that same reference.
    pub fn new(
        ref_sec: i64,
        forwards: HashMap<i64, f64>,
        disc_factors: HashMap<i64, f64>,
        cost_cfg: JWFitCostConfig,
    ) -> Self {
        Self {
            ref_sec,
            forwards,
            disc_factors,
            cost_cfg,
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

            let mut slice_fitter =
                SliceFitter::new(self.cost_cfg.clone(), init_svi, expiry, fwd, discount, self.ref_sec);
            slice_fitter = slice_fitter.with_calibration_log(self.calibration_log.clone());
            let slice_fitter = slice_fitter.build_contracts(slice_bids, slice_asks, slice_strikes, slice_is_calls);
            self.slice_fitters.insert(expiry, slice_fitter);
        }

        Ok(self)
    }

    /// Every quoted expiry needs a forward, a discount factor and an init SVI. Extra entries in
    /// those maps are ignored; missing ones are a caller bug, reported all at once.
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
