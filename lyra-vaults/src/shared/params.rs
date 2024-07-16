use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SpotAuctionParams {
    pub max_spot_spread: f64,
    pub init_spot_spread: f64,
    pub spot_spread_per_min: f64,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,

    // Stopping criteria for a spot auction is USDC >=0 and USDC < max_cash
    pub cash_name: String,
    pub max_cash: BigDecimal,
}

impl SpotAuctionParams {
    /// Returns an auction spot spread, starting from its init value and increasing per minute.
    /// Spot selling auctions would subtract a spread, buying auctions would add a spread.
    pub fn get_spot_spread(&self, start_timestamp_sec: i64) -> f64 {
        let sec_since_start = chrono::Utc::now().timestamp() - start_timestamp_sec;
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.init_spot_spread + min_since_start * self.spot_spread_per_min;
        spread.min(self.max_spot_spread)
    }

    pub fn is_cash_within_threshold(&self, cash_bal: &BigDecimal) -> bool {
        let cash_threshold = self.max_cash.clone();
        cash_bal.abs() < cash_threshold
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionRFQParams {
    pub max_cost: f64,
    pub max_premium_spread: f64,
    pub init_premium_spread: f64,
    pub premium_spread_per_min: f64,

    pub lot_size: BigDecimal,
    pub lot_rounding: BigDecimal,
    pub lot_init_sleep_sec: u64,
    pub auction_sec: i64,

    pub collat_name: String,
}
