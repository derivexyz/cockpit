use crate::shared::rfq::RFQAuction;
use crate::web3::yields::get_growth_between;
use bigdecimal::RoundingMode::Down;
use bigdecimal::{BigDecimal, Zero};
use log::info;
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
    // todo add max_cash_pct_tvl
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
    pub max_cost: BigDecimal,
    pub min_cost: BigDecimal,
    pub max_premium_spread: f64,
    pub init_premium_spread: f64,
    pub premium_spread_per_min: f64,

    pub sizing_type: String,          // "pp" or "covered"
    pub allowed_drawdown: BigDecimal, // max dip into principal to add to yield calculated (only PP)
    pub lot_size: BigDecimal,
    pub lot_rounding: BigDecimal,
    pub lot_init_sleep_sec: u64,
    pub auction_sec: i64,

    // asset name for the collateral as defined in the backend
    pub collat_name: String,
    // currency in which collateral earns interest (e.g. USDE for SUSDE or ETH for LRTs)
    pub quote_name: String,
}

impl OptionRFQParams {
    pub fn get_premium_spread(&self, start_timestamp_sec: i64) -> f64 {
        let sec_since_start = chrono::Utc::now().timestamp() - start_timestamp_sec;
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.init_premium_spread + min_since_start * self.premium_spread_per_min;
        spread.min(self.max_premium_spread)
    }
    pub async fn get_pp_lot_size(
        &self,
        auction: &RFQAuction,
        unit_cost: &BigDecimal,
    ) -> anyhow::Result<BigDecimal> {
        let now = chrono::Utc::now().timestamp();
        let market = &auction.market;
        let reader = market.read().await;
        let option_names = auction.instrument_names();
        let ticker = reader.get_ticker(&option_names[0]).unwrap();
        let sec_to_expiry = ticker.option_details.as_ref().unwrap().expiry - now;
        let spread_balance = match reader.get_position(&option_names[0]) {
            Some(pos) => pos.amount.clone().abs(),
            None => BigDecimal::zero(),
        };

        // todo would be nice to just cache this in the auction state
        // or make a trait function get_total_budget and pass a new arg here
        let collat_balance = reader.get_amount(&self.collat_name);
        let dollar_growth = get_growth_between(
            &self.collat_name,
            &self.quote_name,
            &collat_balance,
            now - sec_to_expiry,
            now,
            &self.allowed_drawdown,
        )
        .await?;

        let size = (dollar_growth / unit_cost) - spread_balance;
        let num_rounded = (&size / &self.lot_rounding).with_scale_round(0, Down);
        let round_size = num_rounded * &self.lot_rounding;
        let lot_size = round_size.clone().min(self.lot_size.clone());
        info!("Desired size: {}, round size: {}, lot_size: {}", size, round_size, lot_size.clone());
        Ok(lot_size)
    }

    pub async fn get_covered_lot_size(&self, auction: &RFQAuction) -> anyhow::Result<BigDecimal> {
        // TODO: this only works delta-1 collateral, stable collat with covered spreads is different
        let market = &auction.market;
        let reader = market.read().await;
        let option_names = auction.instrument_names();
        let ticker = reader.get_ticker(&option_names[0]).unwrap();
        let spread_balance = match reader.get_position(&option_names[0]) {
            Some(pos) => pos.amount.clone().abs(),
            None => BigDecimal::zero(),
        };
        let lrt_pos = reader.get_position(&self.collat_name);
        if lrt_pos.is_none() {
            return Ok(BigDecimal::zero());
        }
        let lrt_pos = lrt_pos.unwrap();
        let size = &lrt_pos.amount - &spread_balance;
        let num_rounded = (&size / &self.lot_rounding).with_scale_round(0, Down);
        let round_size = num_rounded * &self.lot_rounding;
        let lot_size = round_size.clone().min(self.lot_size.clone());
        info!("Desired size: {}, round size: {}, lot_size: {}", size, round_size, lot_size.clone());
        Ok(lot_size)
    }
}
