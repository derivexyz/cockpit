use crate::fitters::slice_fitter::{CalibrationLog, JWFit, JWFitCostConfig, SliceFitter, LIQ_SCORE_COUNT};
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
        self,
        init_svis: HashMap<i64, SVIJWParams>,
        bids: Array1<f64>,
        asks: Array1<f64>,
        strikes: Array1<f64>,
        expiries: Array1<i64>,
        is_calls: Array1<bool>,
    ) -> Result<Self> {
        self.build_slice_fitters(Some(init_svis), bids, asks, strikes, expiries, is_calls)
    }

    /// For a surface with no fit yet, in place of `build_slices` and `fit_slices`: each slice is fitted by
    /// `SliceFitter::fit_bootstrap`, with every pull off, and keeps its fit as its init, so that
    /// `remove_outliers` and `fit_slices` can follow. A slice too thin to bootstrap takes the other
    /// slices' fits interpolated to its expiry, or the nearest one's beyond either end.
    pub fn bootstrap_slices(
        self,
        bids: Array1<f64>,
        asks: Array1<f64>,
        strikes: Array1<f64>,
        expiries: Array1<i64>,
        is_calls: Array1<bool>,
    ) -> Result<Self> {
        let mut surface = self.build_slice_fitters(None, bids, asks, strikes, expiries, is_calls)?;
        let mut expiries: Vec<i64> = surface.slice_fitters.keys().copied().collect();
        expiries.sort_unstable_by(|left, right| right.cmp(left));

        let mut fitted: HashMap<i64, JWFit> = HashMap::new();
        for expiry in &expiries {
            let fitter = &surface.slice_fitters[expiry];
            match fitter.fit_bootstrap(SLICE_FIT_MAX_ITERS) {
                Ok((fit, _)) => {
                    surface.log(format!("INFO Bootstrapped {:.2} DTE with {fit:?}", fitter.dte()));
                    fitted.insert(*expiry, fit);
                }
                Err(err) => surface.log(format!(
                    "WARN {:.2} DTE takes the other slices' fits, its bootstrap failed: {err}",
                    fitter.dte()
                )),
            }
        }
        if fitted.is_empty() && !expiries.is_empty() {
            bail!("no slice could be bootstrapped");
        }

        let ref_sec = surface.ref_sec;
        for (expiry, fitter) in surface.slice_fitters.iter_mut() {
            let fit = fitted.get(expiry).cloned().or_else(|| JWFit::get_interpolated_at(&fitted, *expiry, ref_sec));
            let fit = fit.unwrap_or_else(|| {
                let nearest = fitted.keys().min_by_key(|fitted_expiry| (*fitted_expiry - expiry).abs());
                fitted[nearest.expect("some slice was bootstrapped")].clone()
            });
            fitter.set_init_fit(&fit);
        }
        Ok(surface)
    }

    /// With no init SVIs, every slice is built by `SliceFitter::new_bootstrap`.
    fn build_slice_fitters(
        mut self,
        mut init_svis: Option<HashMap<i64, SVIJWParams>>,
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
        self.check_expiries_covered(&quoted, init_svis.as_ref())?;

        for (expiry, indices) in indices_by_expiry {
            let fwd = self.forwards[&expiry];
            let discount = self.disc_factors[&expiry];

            let slice_bids = Array1::from_iter(indices.iter().map(|&idx| bids[idx]));
            let slice_asks = Array1::from_iter(indices.iter().map(|&idx| asks[idx]));
            let slice_strikes = Array1::from_iter(indices.iter().map(|&idx| strikes[idx]));
            let slice_is_calls = Array1::from_iter(indices.iter().map(|&idx| is_calls[idx]));

            let cost_cfg = self.cost_cfgs[&expiry].clone();
            let mut slice_fitter = match init_svis.as_mut() {
                Some(init_svis) => {
                    let init_svi = init_svis.remove(&expiry).expect("init SVI presence is checked above");
                    SliceFitter::new(cost_cfg, init_svi, expiry, fwd, discount, self.ref_sec)
                }
                None => SliceFitter::new_bootstrap(cost_cfg, expiry, fwd, discount, self.ref_sec),
            };
            slice_fitter = slice_fitter.with_calibration_log(self.calibration_log.clone());
            let slice_fitter = slice_fitter.build_contracts(slice_bids, slice_asks, slice_strikes, slice_is_calls);
            self.slice_fitters.insert(expiry, slice_fitter);
        }

        Ok(self)
    }

    /// Every quoted expiry needs a forward, a discount factor, a cost config and, unless bootstrapped, an
    /// init SVI. Extra entries in those maps are ignored; missing ones are a caller bug, reported all at once.
    fn check_expiries_covered(&self, quoted: &[i64], init_svis: Option<&HashMap<i64, SVIJWParams>>) -> Result<()> {
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
            (
                "init SVI",
                missing(quoted, |expiry| {
                    init_svis.is_none_or(|init_svis| init_svis.contains_key(&expiry))
                }),
            ),
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
    use crate::fitters::slice_fitter::default_cost_cfg;

    #[test]
    fn bootstrap_slices_interpolates_a_slice_too_thin_to_bootstrap() {
        // one thinly quoted expiry still gets a smile, from its neighbours, instead of failing the surface
        let (ref_sec, fwd) = (1_000_000, 100.0);
        let smile = JWFit {
            v: 0.04,
            psi: -0.3,
            p: 1.0,
            c: 0.4,
            v_diff: 0.01,
        };
        let [near, thin, far] = [7, 14, 30].map(|days| ref_sec + days * 86_400);
        let (mut bids, mut asks, mut strikes, mut expiries, mut is_calls) = (vec![], vec![], vec![], vec![], vec![]);
        for expiry in [near, thin, far] {
            let expiry_sec = (expiry - ref_sec) as f64;
            let raw = smile.to_svi_jw(fwd, expiry_sec / crate::SEC_PER_YEAR).to_raw();
            let quoted: Vec<f64> = if expiry == thin {
                vec![99.0, 100.0, 101.0]
            } else {
                (0..15).map(|i| 86.0 + 2.0 * i as f64).collect()
            };
            for strike in quoted {
                let contract = OptionContract {
                    strike,
                    expiry_sec,
                    is_call: strike >= fwd,
                };
                bids.push(contract.price(fwd, raw.get_vol(strike) - 0.005));
                asks.push(contract.price(fwd, raw.get_vol(strike) + 0.005));
                strikes.push(strike);
                expiries.push(expiry);
                is_calls.push(contract.is_call);
            }
        }
        let forwards = HashMap::from([(near, fwd), (thin, fwd), (far, fwd)]);
        let discounts = HashMap::from([(near, 1.0), (thin, 1.0), (far, 1.0)]);

        let surface = SurfaceFitter::new(ref_sec, forwards, discounts, default_cost_cfg())
            .bootstrap_slices(
                Array1::from(bids),
                Array1::from(asks),
                Array1::from(strikes),
                Array1::from(expiries),
                Array1::from(is_calls),
            )
            .expect("bootstrap a surface with one thin slice");

        let fit = |expiry: i64| surface.slices()[&expiry].init_fit();
        let neighbours = HashMap::from([(near, fit(near)), (far, fit(far))]);
        let expected = JWFit::get_interpolated_at(&neighbours, thin, ref_sec).expect("thin lies between its neighbours");
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
}
