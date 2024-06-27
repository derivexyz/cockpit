use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct OptionAuctionParams {
    pub max_iv_spread: f64,
    pub init_iv_spread: f64,
    pub iv_spread_per_min: f64,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,

    pub spot_name: String,
}

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

#[derive(Debug, Clone, Deserialize)]
pub struct LRTCParams {
    pub env: String,        // Environment name (e.g. staging, prod)
    pub vault_name: String, // used as prefix for env vars, e.g. {vault_name}_TSA_ADDRESS
    pub subaccount_id: i64, // Subaccount ID for the vault
    pub currency: String,   // Currency of the options (e.g. ETH)
    pub spot_name: String,  // Spot asset name as in the orderbook API
    pub cash_name: String,  // Cash asset name as in the orderbook API
    pub expiry_days: u64,
    pub min_expiry_hours: u64, // Minimum expiry for options in hours, will remain in spot only stage until an option is available
    pub target_delta: BigDecimal,
    pub max_delta: BigDecimal,
    pub spot_auction_delay_min: i64, // Min delay after expiry before starting spot auctions
    pub option_auction_delay_min: i64, // Min Delay after expiry before starting option auctions

    pub option_auction_params: OptionAuctionParams,
    pub spot_auction_params: SpotAuctionParams,
}

impl LRTCParams {
    pub fn expiry_sec(&self) -> i64 {
        self.expiry_days as i64 * 86400
    }

    pub fn min_expiry_sec(&self) -> i64 {
        self.min_expiry_hours as i64 * 3600
    }

    pub fn spot_auction_delay_sec(&self) -> i64 {
        self.spot_auction_delay_min * 60
    }

    pub fn option_auction_delay_sec(&self) -> i64 {
        self.option_auction_delay_min * 60
    }

    pub fn spot_auction_start(&self, option_expiry: i64) -> i64 {
        option_expiry - self.expiry_sec() + self.spot_auction_delay_sec()
    }

    pub fn option_auction_start(&self, option_expiry: i64) -> i64 {
        option_expiry - self.expiry_sec() + self.option_auction_delay_sec()
    }
}

impl OptionAuctionParams {
    /// Returns an auction IV spread, starting from its init value and increasing per minute.
    /// Option selling auctions would subtract a spread, buying auctions would add a spread.
    pub fn get_iv_spread(&self, start_timestamp_sec: i64) -> f64 {
        let sec_since_start = chrono::Utc::now().timestamp() - start_timestamp_sec;
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.init_iv_spread + min_since_start * self.iv_spread_per_min;
        spread.min(self.max_iv_spread)
    }
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
}
