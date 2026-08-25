use anyhow::Result;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

fn bool_true() -> bool {
    true
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SVIRawParams {
    #[serde(rename = "SVI_a", deserialize_with = "serde_f64")]
    pub a: f64,
    #[serde(rename = "SVI_b", deserialize_with = "serde_f64")]
    pub b: f64,
    #[serde(rename = "SVI_rho", deserialize_with = "serde_f64")]
    pub rho: f64,
    #[serde(rename = "SVI_m", deserialize_with = "serde_f64")]
    pub m: f64,
    #[serde(rename = "SVI_sigma", deserialize_with = "serde_f64")]
    pub sigma: f64,
    #[serde(rename = "SVI_fwd", deserialize_with = "serde_f64")]
    pub fwd: f64,
    #[serde(rename = "SVI_refTau", deserialize_with = "serde_f64")]
    pub reftau: f64,
    // for marks, we want the cap; for risk we keep uncapped to make sure gradients don't vanish
    #[serde(default = "bool_true")]
    pub is_capped: bool,
}

impl SVIRawParams {
    pub fn get_vol_result(&self, strike: f64) -> Result<f64> {
        svi_to_vol(
            self.a,
            self.b,
            self.rho,
            self.m,
            self.sigma,
            self.fwd,
            self.reftau,
            strike,
            self.is_capped,
        )
    }
    pub fn get_vol(&self, strike: f64) -> f64 {
        self.get_vol_result(strike).expect("SVI to vol failed")
    }
    pub fn get_vol_moneyness(&self, k: f64) -> f64 {
        let strike = self.fwd * k.exp();
        self.get_vol(strike)
    }
    pub fn with_fwd(&self, fwd: f64) -> Self {
        Self { fwd, ..*self }
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub struct SVIJWParams {
    pub v: f64,
    pub psi: f64,
    pub p: f64,
    pub c: f64,
    pub v_diff: f64, // v_t - v_tilde_t, less correlated than v_tilde

    pub fwd: f64,
    pub reftau: f64,
}

impl SVIJWParams {
    pub fn from_raw(raw: &SVIRawParams) -> Self {
        let a = raw.a;
        let b = raw.b;
        let rho = raw.rho;
        let m = raw.m;
        let sigma = raw.sigma;
        let t = raw.reftau;

        let sqrt_term = (m * m + sigma * sigma).sqrt();

        let v_t = (a + b * (-rho * m + sqrt_term)) / t;

        let w_t = v_t * t; // w_t := v_t * t

        let sqrt_w_t = w_t.sqrt();

        let psi_t = (1.0 / sqrt_w_t) * (b / 2.0) * (-m / sqrt_term + rho);

        let p_t = (1.0 / sqrt_w_t) * b * (1.0 - rho);

        let c_t = (1.0 / sqrt_w_t) * b * (1.0 + rho);

        let v_tilde_t = (1.0 / t) * (a + b * sigma * (1.0 - rho * rho).sqrt());

        Self {
            v: v_t,
            psi: psi_t,
            p: p_t,
            c: c_t,
            v_diff: v_t - v_tilde_t,
            fwd: raw.fwd,
            reftau: raw.reftau,
        }
    }
    pub fn with_fwd(&self, fwd: f64) -> Self {
        Self {
            v: self.v,
            psi: self.psi,
            c: self.c,
            p: self.p,
            v_diff: self.v_diff,
            reftau: self.reftau,
            fwd,
        }
    }
    #[inline]
    fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    pub fn to_raw(&self) -> SVIRawParams {
        let t = self.reftau;

        let v_t = self.v;
        let w_t = v_t * t;
        let sqrt_w_t = w_t.sqrt();

        let v_tilde_t = v_t - self.v_diff;

        let b = (sqrt_w_t / 2.0) * (self.c + self.p);
        let rho = 1.0 - (self.p * sqrt_w_t) / b;

        let beta = (rho - (2.0 * self.psi * sqrt_w_t) / b).clamp(-1.0, 1.0);

        // beta = m / sqrt(m^2 + sigma^2). Therefore beta ~ 0 is the m = 0 branch.
        // This is distinct from v_diff ~ 0, which can occur for m != 0.
        let beta_eps = 1e-12;
        let rho_eps = 1e-12;
        let v_diff = self.v_diff;
        let w_tilde = v_tilde_t * t;
        let sqrt_1_minus_rho2 = (1.0 - rho * rho).max(0.0).sqrt();

        let (m, sigma, a) = if beta.abs() > beta_eps {
            // Regular m != 0 branch. Lemma 3.2 uses alpha = sigma / m.
            // Since beta = m / sqrt(m^2 + sigma^2), alpha = sign(beta) * sqrt(1 / beta^2 - 1).
            let alpha = Self::sign(beta) * ((1.0 / (beta * beta) - 1.0).max(0.0)).sqrt();
            let sign_alpha = Self::sign(alpha);
            let sqrt_1_plus_alpha2 = (1.0 + alpha * alpha).sqrt();

            // denom = b { -rho + sign(alpha)*sqrt(1+alpha^2) - alpha*sqrt(1-rho^2) }
            let denom = b * (-rho + sign_alpha * sqrt_1_plus_alpha2 - alpha * sqrt_1_minus_rho2);

            if denom.abs() > 1e-14 || v_diff.abs() > 1e-14 {
                let m = (v_diff * t) / denom;
                let sigma = alpha * m;
                // a = v_tilde_t * t - b * sigma * sqrt(1 - rho^2)
                let a = w_tilde - b * sigma * sqrt_1_minus_rho2;
                (m, sigma, a)
            } else {
                // v_diff ~ 0 and denom ~ 0 is another non-unique inverse case.
                // Pick a non-degenerate representative that preserves beta = m/sqrt(m^2 + sigma^2)
                // and a + b*sigma*sqrt(1-rho^2) = w_tilde, so JW -> raw -> JW is stable.
                let sigma = if b.abs() > 1e-14 { w_t / b } else { 0.0 };
                let m = if (1.0 - beta * beta).abs() > 1e-14 {
                    beta * sigma / (1.0 - beta * beta).sqrt()
                } else {
                    0.0
                };
                let a = w_tilde - b * sigma * sqrt_1_minus_rho2;
                (m, sigma, a)
            }
        } else {
            // m = 0 branch. Here:
            //   w       = a + b*sigma
            //   w_tilde = a + b*sigma*sqrt(1-rho^2)
            // so, if rho != 0:
            //   sigma = (w - w_tilde) / (b * (1 - sqrt(1-rho^2)))
            let m = 0.0;
            let one_minus_q = 1.0 - sqrt_1_minus_rho2;

            if one_minus_q.abs() > rho_eps {
                let sigma = (w_t - w_tilde) / (b * one_minus_q);
                let a = w_t - b * sigma;
                (m, sigma, a)
            } else {
                // beta ~ 0 and rho ~ 0 is non-unique: JW only identifies a + b*sigma = w.
                // Pick a non-degenerate convention rather than collapsing to sigma = 0.
                let sigma = if b.abs() > 1e-14 { w_t / b } else { 0.0 };
                let a = w_t - b * sigma;
                (m, sigma, a)
            }
        };

        SVIRawParams {
            a,
            b,
            rho,
            m,
            sigma,
            fwd: self.fwd,
            reftau: self.reftau,
            is_capped: false,
        }
    }

    pub fn to_raw_capped(&self) -> SVIRawParams {
        let mut raw = self.to_raw();
        raw.is_capped = true;
        raw
    }
    pub fn get_vol_result(&self, strike: f64) -> Result<f64> {
        let raw = self.to_raw();
        raw.get_vol_result(strike)
    }
    pub fn get_vol(&self, strike: f64) -> f64 {
        let raw = self.to_raw();
        raw.get_vol(strike)
    }

    pub fn get_vol_capped(&self, strike: f64) -> f64 {
        let raw = self.to_raw_capped();
        raw.get_vol(strike)
    }

    pub fn get_vol_moneyness(&self, k: f64) -> f64 {
        let raw = self.to_raw();
        raw.get_vol_moneyness(k)
    }
}

fn serde_f64<'de, D>(deserializer: D) -> anyhow::Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f64,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

const VOLCONSTUP: f64 = 4.0;
const VOLCONSTDOWN: f64 = 4.0;
const MAX_TOTAL_VAR: f64 = 2.0;

#[allow(clippy::too_many_arguments)]
pub fn svi_to_vol(
    a: f64,
    b: f64,
    rho: f64,
    m: f64,
    sigma: f64,
    fwd: f64,
    reftau: f64,
    strike: f64,
    is_capped: bool,
) -> anyhow::Result<f64> {
    let k = if is_capped {
        let k = (strike / fwd).ln();
        let k_static = (a + b * sigma).sqrt();
        if k > 0.0 {
            f64::min(k, VOLCONSTUP * k_static)
        } else {
            f64::max(k, -VOLCONSTDOWN * k_static)
        }
    } else {
        (strike / fwd).ln()
    };

    let sqrt_term = ((k - m).powi(2) + sigma.powi(2)).sqrt();
    let linear_term = rho * (k - m);

    let total_implied_variance = if is_capped {
        f64::min(MAX_TOTAL_VAR, a + b * (linear_term + sqrt_term))
    } else {
        a + b * (linear_term + sqrt_term)
    };
    let time_scaled_total_implied_variance = (1.0 / reftau) * total_implied_variance;

    if time_scaled_total_implied_variance < 0.0 {
        return Err(anyhow::anyhow!(
            "Negative total variance with {:?}",
            (a, b, rho, m, sigma, fwd, reftau, strike, is_capped)
        ));
    }

    Ok(time_scaled_total_implied_variance.sqrt())
}

#[cfg(test)]
mod tests {
    use crate::SEC_PER_YEAR;
    use crate::black76::OptionContract;

    fn assert_jw_approx(original: &SVIJWParams, recovered: &SVIJWParams, tol: f64) {
        assert!(
            (original.v - recovered.v).abs() < tol,
            "v mismatch: {} vs {}",
            original.v,
            recovered.v
        );
        assert!(
            (original.psi - recovered.psi).abs() < tol,
            "psi mismatch: {} vs {}",
            original.psi,
            recovered.psi
        );
        assert!(
            (original.p - recovered.p).abs() < tol,
            "p mismatch: {} vs {}",
            original.p,
            recovered.p
        );
        assert!(
            (original.c - recovered.c).abs() < tol,
            "c mismatch: {} vs {}",
            original.c,
            recovered.c
        );
        assert!(
            (original.v_diff - recovered.v_diff).abs() < tol,
            "v_diff mismatch: {} vs {}",
            original.v_diff,
            recovered.v_diff
        );
        assert!(
            (original.fwd - recovered.fwd).abs() < tol,
            "fwd mismatch: {} vs {}",
            original.fwd,
            recovered.fwd
        );
        assert!(
            (original.reftau - recovered.reftau).abs() < tol,
            "reftau mismatch: {} vs {}",
            original.reftau,
            recovered.reftau
        );
    }
    use super::*;

    fn assert_raw_approx(original: &SVIRawParams, recovered: &SVIRawParams, tol: f64) {
        assert!(
            (original.a - recovered.a).abs() < tol,
            "a mismatch: {} vs {}",
            original.a,
            recovered.a
        );
        assert!(
            (original.b - recovered.b).abs() < tol,
            "b mismatch: {} vs {}",
            original.b,
            recovered.b
        );
        assert!(
            (original.rho - recovered.rho).abs() < tol,
            "rho mismatch: {} vs {}",
            original.rho,
            recovered.rho
        );
        assert!(
            (original.m - recovered.m).abs() < tol,
            "m mismatch: {} vs {}",
            original.m,
            recovered.m
        );
        assert!(
            (original.sigma - recovered.sigma).abs() < tol,
            "sigma mismatch: {} vs {}",
            original.sigma,
            recovered.sigma
        );
        assert!(
            (original.fwd - recovered.fwd).abs() < tol,
            "fwd mismatch: {} vs {}",
            original.fwd,
            recovered.fwd
        );
        assert!(
            (original.reftau - recovered.reftau).abs() < tol,
            "reftau mismatch: {} vs {}",
            original.reftau,
            recovered.reftau
        );
    }

    /// Round-trip test with m != 0.
    /// Constraints: rho in (-1, 1), b >= 0, sigma > 0, reftau > 0.
    #[test]
    fn test_roundtrip_m_nonzero() {
        let raw = SVIRawParams {
            a: 0.04,
            b: 0.2,
            rho: -0.3,
            m: 0.1,
            sigma: 0.15,
            fwd: 3000.0,
            reftau: 0.5,
            is_capped: true,
        };

        let jw = SVIJWParams::from_raw(&raw);
        let recovered = jw.to_raw();

        println!("{:?} raw", raw);
        println!("{:?} recovered", recovered);

        assert_raw_approx(&raw, &recovered, 1e-9);
    }

    /// Round-trip test with m = 0.
    /// Constraints: rho in (-1, 1), b >= 0, sigma > 0, reftau > 0.
    #[test]
    fn test_roundtrip_m_zero() {
        let raw = SVIRawParams {
            a: 0.02,
            b: 0.15,
            rho: 0.1,
            m: 0.0,
            sigma: 0.2,
            fwd: 50000.0,
            reftau: 0.25,
            is_capped: true,
        };

        let jw = SVIJWParams::from_raw(&raw);
        let recovered = jw.to_raw();

        println!("{:?} raw", raw);
        println!("{:?} jw", jw);
        println!("{:?} recovered", recovered);

        assert_raw_approx(&raw, &recovered, 1e-9);
    }

    #[test]
    fn print_xaut_jw_params() {
        let vol_data = serde_json::json!({
            "SVI_a": "0.000390959670979341",
            "SVI_b": "0.013212514095684939",
            "SVI_rho": "-0.21429759874369006",
            "SVI_m": "-0.018971165984531226",
            "SVI_sigma": "0.014120710431484882",
            "SVI_fwd": "4366.54784427419",
            "SVI_refTau": "0.00943049213597"
        });
        let raw: SVIRawParams = serde_json::from_value(vol_data).expect("parse XAUT raw SVI params");
        let jw = SVIJWParams::from_raw(&raw);

        println!("{jw:#?}");
    }

    #[test]
    fn print_xaut_jw_call_prices() {
        const FWD: f64 = 4_366.0;
        const RISK_FREE_RATE: f64 = 0.03;

        let svi = SVIJWParams {
            v: 0.06949485600386272,
            psi: 0.12221432547824267,
            p: 0.6139639169794924,
            c: 0.42019312865316444,
            v_diff: 0.004906027422175038,
            fwd: FWD,
            reftau: 0.009369984525621511,
        };
        let expiry_sec = svi.reftau * SEC_PER_YEAR;
        let discount = (-RISK_FREE_RATE * svi.reftau).exp();
        let mut previous_price = f64::INFINITY;

        println!(
            "fwd={FWD}, rate={RISK_FREE_RATE}, tau={}, discount={discount}",
            svi.reftau
        );
        for strike in [4_400.0, 4_500.0, 4_600.0] {
            let iv = svi.get_vol_capped(strike);
            let contract = OptionContract {
                strike,
                expiry_sec,
                is_call: true,
            };
            let call_price = contract.price(FWD, iv) * discount;

            println!("strike={strike}, iv={iv}, call_price={call_price}");
            assert!(call_price.is_finite() && call_price >= 0.0);
            assert!(call_price < previous_price);
            previous_price = call_price;
        }
    }

    #[test]
    fn test_roundtrip_m_zero_negative_rho() {
        let raw = SVIRawParams {
            a: 0.018,
            b: 0.12,
            rho: -0.45,
            m: 0.0,
            sigma: 0.17,
            fwd: 2500.0,
            reftau: 0.4,
            is_capped: true,
        };

        let jw = SVIJWParams::from_raw(&raw);
        let recovered = jw.to_raw();

        println!("{:?} raw", raw);
        println!("{:?} jw", jw);
        println!("{:?} recovered", recovered);

        assert_raw_approx(&raw, &recovered, 1e-9);
    }

    #[test]
    fn test_roundtrip_m_zero_rho_zero_back_to_jw() {
        let raw = SVIRawParams {
            a: 0.01,
            b: 0.2,
            rho: 0.0,
            m: 0.0,
            sigma: 0.25,
            fwd: 3000.0,
            reftau: 0.5,
            is_capped: true,
        };

        let jw = SVIJWParams::from_raw(&raw);
        let recovered_raw = jw.to_raw();
        let recovered_jw = SVIJWParams::from_raw(&recovered_raw);

        println!("{:?} raw", raw);
        println!("{:?} jw", jw);
        println!("{:?} recovered_raw", recovered_raw);
        println!("{:?} recovered_jw", recovered_jw);

        // Raw params are non-unique when m = 0 and rho = 0, so only JW should round-trip.
        assert_jw_approx(&jw, &recovered_jw, 1e-9);
    }

    #[test]
    fn test_roundtrip_jw_v_diff_zero_back_to_jw() {
        let jw = SVIJWParams {
            v: 0.36,
            psi: 0.0,
            p: 0.7,
            c: 1.3,
            v_diff: 0.0,
            fwd: 2500.0,
            reftau: 0.25,
        };

        let recovered_raw = jw.to_raw();
        let recovered_jw = SVIJWParams::from_raw(&recovered_raw);

        println!("{:?} jw", jw);
        println!("{:?} recovered_raw", recovered_raw);
        println!("{:?} recovered_jw", recovered_jw);

        assert_jw_approx(&jw, &recovered_jw, 1e-9);
    }
}
