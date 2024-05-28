use bigdecimal::BigDecimal;
use std::str::FromStr;
/// Expiry selector will iterate from the largest expiry and pick the first one <= target
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct OptionAuctionParams {
    pub min_iv: f64,
    pub max_iv_spread: f64,
    pub init_iv_spread: f64,
    pub iv_spread_per_min: f64,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,

    pub spot_name: String,
}

#[derive(Debug, Clone)]
pub struct SpotAuctionParams {
    pub max_spot_spread: f64,
    pub init_spot_spread: f64,
    pub spot_spread_per_min: f64,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,

    // Stopping criteria for a spot auction is USDC >=0 and USDC < max_cash
    pub max_cash: BigDecimal,
}

#[derive(Debug, Clone)]
pub struct LRTCParams {
    pub subaccount_id: i64,
    pub currency: String,
    pub spot_name: String,
    pub cash_name: String,
    pub expiry: TargetExpiry,
    pub target_delta: BigDecimal,
    pub max_delta: BigDecimal,

    pub option_auction_params: OptionAuctionParams,
    pub spot_auction_params: SpotAuctionParams,
}

impl OptionAuctionParams {
    pub fn get_iv_spread(&self, start_timestamp_sec: i64) -> f64 {
        let sec_since_start = chrono::Utc::now().timestamp() - start_timestamp_sec;
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.init_iv_spread + min_since_start * self.iv_spread_per_min;
        spread.min(self.max_iv_spread)
    }
}
