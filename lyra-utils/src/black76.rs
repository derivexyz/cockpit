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

#[derive(Debug, Clone, Default)]
pub struct PriceAndGreeks {
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub vanna: f64,
    pub volga: f64,
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

    pub fn vanna(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        let d2 = d2(d1, vol, tau);
        -normpdf(d1) * d2 / vol
    }

    pub fn volga(&self, fwd: f64, vol: f64) -> f64 {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            return 0.0;
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        let d2 = d2(d1, vol, tau);
        fwd * normpdf(d1) * tau.sqrt() * (d1 * d2 / vol)
    }

    pub fn price_and_greeks(&self, fwd: f64, vol: f64) -> PriceAndGreeks {
        let tau = self.expiry_sec / SEC_PER_YEAR;
        if tau <= 0.0 {
            let mark = self.price_expired(fwd);
            return PriceAndGreeks { price: mark, ..PriceAndGreeks::default() };
        }
        let d1 = d1(vol, self.strike, fwd, tau);
        let d2 = d2(d1, vol, tau);
        let pdf_d1 = normpdf(d1);
        let cdf_d1 = normcdf(d1);
        let cdf_d2 = normcdf(d2);
        let call_price = fwd * cdf_d1 - self.strike * cdf_d2;
        let price = match self.is_call {
            true => call_price,
            // call_price - (fwd - strike)
            false => call_price - fwd + self.strike,
        };
        let call_delta = cdf_d1;
        let delta = match self.is_call {
            true => call_delta,
            false => call_delta - 1.0,
        };
        let gamma = pdf_d1 / (fwd * vol * tau.sqrt());
        let theta = -fwd * pdf_d1 * vol / (2.0 * tau.sqrt());
        let vega = fwd * pdf_d1 * tau.sqrt();
        let vanna = -pdf_d1 * d2 / vol;
        let volga = fwd * pdf_d1 * tau.sqrt() * (d1 * d2 / vol);
        PriceAndGreeks { price, delta, gamma, theta, vega, vanna, volga }
    }
}

// write a simple test here to make sure the price_and_greeks returns the same
// values as calling the formulas separately
#[cfg(test)]
mod tests {
    use super::*;
    use statrs::assert_almost_eq;
    #[test]
    fn test_price_and_greeks() {
        let fwd = 100.0;
        let vol = 0.2;
        let tol = 1e-9;
        let contract =
            OptionContract { strike: 105.0, expiry_sec: 3600.0 * 24.0 * 7.0, is_call: true };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);
        assert_almost_eq!(price_and_greeks.price, contract.price(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.delta, contract.delta(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.gamma, contract.gamma(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.theta, contract.theta(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.vega, contract.vega(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.vanna, contract.vanna(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.volga, contract.volga(fwd, vol), tol);

        // now for put
        let contract =
            OptionContract { strike: 105.0, expiry_sec: 3600.0 * 24.0 * 7.0, is_call: false };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);
        assert_almost_eq!(price_and_greeks.price, contract.price(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.delta, contract.delta(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.gamma, contract.gamma(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.theta, contract.theta(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.vega, contract.vega(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.vanna, contract.vanna(fwd, vol), tol);
        assert_almost_eq!(price_and_greeks.volga, contract.volga(fwd, vol), tol);

        // now for an expired contract
        let contract = OptionContract { strike: 105.0, expiry_sec: 0.0, is_call: true };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);
        assert_almost_eq!(price_and_greeks.price, contract.price_expired(fwd), tol);
        assert_almost_eq!(price_and_greeks.delta, 0.0, tol);
        assert_almost_eq!(price_and_greeks.gamma, 0.0, tol);
        assert_almost_eq!(price_and_greeks.theta, 0.0, tol);
        assert_almost_eq!(price_and_greeks.vega, 0.0, tol);
        assert_almost_eq!(price_and_greeks.vanna, 0.0, tol);
        assert_almost_eq!(price_and_greeks.volga, 0.0, tol);

        // now expired put
        let contract = OptionContract { strike: 105.0, expiry_sec: 0.0, is_call: false };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);
        assert_almost_eq!(price_and_greeks.price, contract.price_expired(fwd), tol);
    }
    #[test]
    fn test_volga_numerically() {
        // make sure volga is approx the first deriv of vega w.r.t vol;
        let fwd = 100.0;
        let vol = 0.2;
        let tol = 1e-6;
        let contract =
            OptionContract { strike: 105.0, expiry_sec: 3600.0 * 24.0 * 7.0, is_call: true };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);
        let d_vol = 0.000001;

        let up_greeks = contract.price_and_greeks(fwd, vol + d_vol);
        let down_greeks = contract.price_and_greeks(fwd, vol - d_vol);

        let approx_volga = (up_greeks.vega - down_greeks.vega) / (2.0 * d_vol);
        assert_almost_eq!(price_and_greeks.volga, approx_volga, tol);
    }
    #[test]
    fn test_vanna_numerically() {
        // make sure vanna is approx first deriv of vega w.r.t. spot AND
        // first deriv of delta w.r.t vol;
        let fwd = 100.0;
        let vol = 0.2;
        let tol = 1e-6;
        let contract =
            OptionContract { strike: 105.0, expiry_sec: 3600.0 * 24.0 * 7.0, is_call: true };
        let price_and_greeks = contract.price_and_greeks(fwd, vol);

        let d_vol = 0.000001;
        let d_fwd = 0.000001;

        let up_vol_greeks = contract.price_and_greeks(fwd, vol + d_vol);
        let down_vol_greeks = contract.price_and_greeks(fwd, vol - d_vol);
        let approx_vanna_vol = (up_vol_greeks.delta - down_vol_greeks.delta) / (2.0 * d_vol);

        assert_almost_eq!(price_and_greeks.vanna, approx_vanna_vol, tol);

        let up_fwd_greeks = contract.price_and_greeks(fwd + d_fwd, vol);
        let down_fwd_greeks = contract.price_and_greeks(fwd - d_fwd, vol);
        let approx_vanna_fwd = (up_fwd_greeks.vega - down_fwd_greeks.vega) / (2.0 * d_fwd);

        assert_almost_eq!(price_and_greeks.vanna, approx_vanna_fwd, tol);
    }
}
