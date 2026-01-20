use anyhow::Result;
use statrs::function::erf::erf;

const FRAC_1_SQRT_PI: f64 = 0.564189583547756286948079451560772586_f64;
const FRAC_1_SQRT_2_PI: f64 = FRAC_1_SQRT_PI * std::f64::consts::FRAC_1_SQRT_2;
const SEC_PER_YEAR: f64 = 365.0 * 24.0 * 60.0 * 60.0;

pub fn normcdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x * std::f64::consts::FRAC_1_SQRT_2))
}

pub fn normpdf(x: f64) -> f64 {
    (-0.5 * x.powi(2)).exp() * FRAC_1_SQRT_2_PI
}

fn d1(sigma: f64, strike: f64, fwd: f64, tau: f64) -> f64 {
    (-f64::ln(strike / fwd) + (0.5 * sigma.powi(2)) * tau) / (sigma * tau.sqrt())
}

fn d2(d1: f64, sigma: f64, tau: f64) -> f64 {
    d1 - sigma * tau.sqrt()
}

#[derive(Debug, Clone)]
pub struct OptionContract {
    pub strike: f64,
    pub expiry_sec: f64,
    pub is_call: bool,
}

impl OptionContract {
    pub fn price_expired(&self, fwd: f64) -> f64 {
        if self.is_call {
            (fwd - self.strike).max(0.0)
        } else {
            (self.strike - fwd).max(0.0)
        }
    }

    pub fn price(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return self.price_expired(fwd);
        }

        let d1 = d1(vol, self.strike, fwd, tau);
        let d2 = d2(d1, vol, tau);
        if self.is_call {
            fwd * normcdf(d1) - self.strike * normcdf(d2)
        } else {
            self.strike * normcdf(-d2) - fwd * normcdf(-d1)
        }
    }

    pub fn delta(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        if self.is_call {
            normcdf(d1)
        } else {
            normcdf(d1) - 1.0
        }
    }

    pub fn vega(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        fwd * normpdf(d1) * tau.sqrt()
    }

    pub fn gamma(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        normpdf(d1) / (fwd * vol * tau.sqrt())
    }

    // note no discounting here, can add those terms later via (-r * premium)
    pub fn theta(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        -fwd * normpdf(d1) * vol / (2.0 * tau.sqrt())
    }
}
