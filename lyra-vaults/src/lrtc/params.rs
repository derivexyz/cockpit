use bigdecimal::BigDecimal;
use std::str::FromStr;
/// Expiry selector will iterate from the largest expiry and pick the first one <= target
pub enum TargetExpiry {
    ZeroDTE,
    OneDTE,
    Weekly,
}

impl TargetExpiry {
    pub fn to_expiry_sec(&self) -> i64 {
        match self {
            TargetExpiry::ZeroDTE => 86400,
            TargetExpiry::OneDTE => 172800,
            TargetExpiry::Weekly => 604800,
        }
    }
}

/// Delta selector will iterate from the largest delta and pick the first one <= target
pub enum TargetDelta {
    Ten,
    Fifteen,
}

impl TargetDelta {
    pub fn to_decimal(&self) -> BigDecimal {
        match self {
            TargetDelta::Ten => BigDecimal::from_str("0.1").unwrap(),
            TargetDelta::Fifteen => BigDecimal::from_str("0.15").unwrap(),
        }
    }
}

pub struct LRTCParams {
    pub subaccount_id: i64,
    pub currency: String,
    pub spot_name: String,
    pub cash_name: String,
    pub expiry: TargetExpiry,
    pub delta: TargetDelta,

    pub min_iv: f64,
    pub max_iv_spread: f64,
    pub init_iv_spread: f64,
    pub iv_spread_per_hour: f64,
    pub option_auction_sec: u64,
    // todo similar spread params for spot buy/sell
}

pub struct LRTCSpotExecutor {
    // TODO
}
