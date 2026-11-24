use crate::fitters::slice_fitter::{CalibrationLog, JWFit, JWFitCostConfig, SliceFitter, LIQ_SCORE_COUNT};
use crate::fitters::svi::SVIJWParams;
use anyhow::{anyhow, bail, Result};
use ndarray::Array1;
use std::collections::{HashMap, HashSet};
use std::thread;

const SLICE_FIT_MAX_ITERS: u64 = 500;

#[derive(Clone)]
pub struct SurfaceFitter {
    ref_sec: i64,
    forwards: HashMap<i64, f64>,              // expiry (sec) -> forward price
    disc_factors: HashMap<i64, f64>,          // expiry (sec) -> discount factor applied to option prices
    cost_cfgs: HashMap<i64, JWFitCostConfig>, // expiry (sec) -> cost config its slice is fitted with
    slice_fitters: HashMap<i64, SliceFitter>, // expiry (sec) -> slice fitter
    calibration_log: CalibrationLog,
}

/// How `fit_multithread` starts a slice.
#[derive(Clone, Debug)]
pub enum SliceStart {
    /// Fitted from this SVI, regularized towards it.
    Init(SVIJWParams),
    /// Not fitted: this SVI, priced on the slice's quotes.
    Hold(SVIJWParams),
    /// From its quotes alone (`SliceFitter::fit_bootstrap`).
    Bootstrap,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SliceStatus {
    Refit,
    Bootstrapped,
    Held,
    /// The fit erred, or panicked where panics unwind. An init or a hold returns its SVI.
    Failed(String),
}

#[derive(Clone, Debug)]
pub struct SliceOutcome {
    pub status: SliceStatus,
    /// (fit, its bbo cost on the slice's quotes); None only for a failed bootstrap
    pub fit: Option<(JWFit, f64)>,
    pub liquidity: f64,
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

    /// As [`SurfaceFitter::new`], with a cost config per expiry (sec), e.g. to scale regularization by tenor.
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
    /// `SliceFitter::fit_bootstrap`, with no regularization, and keeps its fit as its init, so that
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
            let log = surface.calibration_log.for_expiry(*expiry);
            match fitter.fit_bootstrap(SLICE_FIT_MAX_ITERS) {
                Ok((fit, _)) => {
                    log.push(format!("INFO Bootstrapped {:.2} DTE with {fit:?}", fitter.dte()));
                    fitted.insert(*expiry, fit);
                }
                Err(err) => log.push(format!(
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
            let fit =
                JWFit::get_interpolated_or_nearest(&fitted, *expiry, ref_sec).expect("some slice was bootstrapped");
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

    /// Slices are fitted independently of one another, one after another.
    pub fn fit_slices(mut self) -> Result<Self> {
        if self.slice_fitters.is_empty() {
            self.log("WARN empty liquidity_scores");
            return Ok(self);
        }

        let mut expiries: Vec<i64> = self.slice_fitters.keys().copied().collect();
        expiries.sort_unstable_by(|left, right| right.cmp(left));

        for expiry in expiries {
            let log = self.calibration_log.for_expiry(expiry);
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

    /// Quotes without inits, only for `hold_outliers` and `fit_multithread`: other methods read the all-zero inits.
    pub fn with_quotes(
        self,
        bids: Array1<f64>,
        asks: Array1<f64>,
        strikes: Array1<f64>,
        expiries: Array1<i64>,
        is_calls: Array1<bool>,
    ) -> Result<Self> {
        self.build_slice_fitters(None, bids, asks, strikes, expiries, is_calls)
    }

    /// Removes up to `n` outliers per slice in `inits` from a copy; returns it with the illiquid expiries to hold.
    pub fn hold_outliers(
        &self,
        inits: &HashMap<i64, SVIJWParams>,
        n: usize,
        factor: f64,
        min_score: f64,
    ) -> Result<(Self, HashSet<i64>)> {
        let mut cleaned = self.clone();
        let mut held = HashSet::new();
        for (expiry, init) in inits {
            let Some(fitter) = cleaned.slice_fitters.get_mut(expiry) else {
                continue;
            };
            let liquidity = fitter.get_liquidity_score();
            if liquidity > min_score {
                let line = format!(
                    "INFO {:.2} DTE held: liquidity score {liquidity} is past {min_score}",
                    fitter.dte()
                );
                self.calibration_log.for_expiry(*expiry).push(line);
                held.insert(*expiry);
                continue;
            }
            *fitter = fitter.clone().with_init_svi(init.clone());
            fitter.remove_outliers(n, factor, min_score)?;
        }
        Ok((cleaned, held))
    }

    /// Fits each slice in `starts` on its own thread. Errs only on a start for an unquoted expiry.
    pub fn fit_multithread(
        &self,
        starts: &HashMap<i64, SliceStart>,
        max_iters: u64,
    ) -> Result<HashMap<i64, SliceOutcome>> {
        let mut unquoted: Vec<i64> =
            starts.keys().copied().filter(|expiry| !self.slice_fitters.contains_key(expiry)).collect();
        if !unquoted.is_empty() {
            unquoted.sort_unstable();
            bail!("starts for expiries with no quotes: {unquoted:?}");
        }

        let outcomes: HashMap<i64, SliceOutcome> = thread::scope(|scope| {
            let fits: Vec<_> = starts
                .iter()
                .map(|(&expiry, start)| {
                    (
                        expiry,
                        start,
                        scope.spawn(move || self.fit_slice(expiry, start, max_iters)),
                    )
                })
                .collect();
            fits.into_iter()
                .map(|(expiry, start, fit)| {
                    let outcome = match fit.join() {
                        Ok(Ok(outcome)) => outcome,
                        Ok(Err(error)) => self.failed(expiry, start, format!("{error:#}")),
                        Err(_) => self.failed(expiry, start, "the fit panicked".to_string()),
                    };
                    (expiry, outcome)
                })
                .collect()
        });
        for (expiry, outcome) in &outcomes {
            let dte = self.slice_fitters[expiry].dte();
            let line = format!("INFO {dte:.2} DTE {:?} with {:?}", outcome.status, outcome.fit);
            self.calibration_log.for_expiry(*expiry).push(line);
        }
        Ok(outcomes)
    }

    fn fit_slice(&self, expiry: i64, start: &SliceStart, max_iters: u64) -> Result<SliceOutcome> {
        if let SliceStart::Init(svi) = start {
            // checked before the fitter asserts it, as a panic aborts a process built with panic = "abort"
            JWFit::from_svi_jw(svi).check_fittable()?;
        }
        let fitter = self.fitter_for(expiry, start);
        let (fit, status): (JWFit, SliceStatus) = match start {
            SliceStart::Init(_) => (fitter.fit_bfgs(max_iters)?.0, SliceStatus::Refit),
            SliceStart::Hold(_) => (fitter.init_fit(), SliceStatus::Held),
            SliceStart::Bootstrap => (fitter.fit_bootstrap(max_iters)?.0, SliceStatus::Bootstrapped),
        };
        let bbo_cost = fitter.bbo_cost_from_fit(&fit, false)?;
        Ok(SliceOutcome {
            status,
            fit: Some((fit, bbo_cost)),
            liquidity: fitter.get_liquidity_score(),
        })
    }

    fn failed(&self, expiry: i64, start: &SliceStart, error: String) -> SliceOutcome {
        let fitter = self.fitter_for(expiry, start);
        let fit = match start {
            SliceStart::Init(_) | SliceStart::Hold(_) => {
                let init = fitter.init_fit();
                fitter.bbo_cost_from_fit(&init, false).ok().map(|bbo_cost| (init, bbo_cost))
            }
            SliceStart::Bootstrap => None,
        };
        SliceOutcome {
            status: SliceStatus::Failed(error),
            fit,
            liquidity: fitter.get_liquidity_score(),
        }
    }

    fn fitter_for(&self, expiry: i64, start: &SliceStart) -> SliceFitter {
        let quoted = self.slice_fitters[&expiry].clone();
        let cost_cfg = self.cost_cfgs[&expiry].clone();
        match start {
            SliceStart::Init(svi) | SliceStart::Hold(svi) => quoted.with_cost_cfg(cost_cfg).with_init_svi(svi.clone()),
            SliceStart::Bootstrap => quoted.with_cost_cfg(cost_cfg.scale_regularization(0.0)),
        }
    }
}
