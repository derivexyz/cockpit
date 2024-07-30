use crate::shared::params::{OptionRFQParams, SpotAuctionParams};
use crate::shared::rfq::{RFQAuction, RFQLot, RFQStrategy};
use crate::web3::yields::{get_growth_between, get_price_at_timestamp};
use anyhow::Result;
use bigdecimal::RoundingMode::{Down, HalfEven};
use bigdecimal::{BigDecimal, Zero};
use log::info;
use rust_decimal::prelude::FromPrimitive;
use serde::Deserialize;
use std::env;
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize)]
pub struct LongPPParams {
    pub env: String,             // Environment name (e.g. staging, prod)
    pub vault_name: String,      // used as prefix for env vars, e.g. {SUSDE}_TSA_ADDRESS
    pub option_currency: String, // Currency of the options (e.g. ETH)
    pub expiry_days: u64,
    pub min_expiry_hours: u64, // Minimum expiry for options in hours, will remain in spot only stage until an option is available
    pub strike_diff: BigDecimal,
    pub min_premium_to_strike_ratio: BigDecimal,
    pub max_premium_to_strike_ratio: BigDecimal,
    pub target_premium_to_strike_ratio: BigDecimal,
    /// todo how to fetch interest
    pub spot_auction_delay_min: i64, // Min delay after expiry before starting spot auctions
    pub option_auction_delay_min: i64, // Min Delay after expiry before starting option auctions

    pub option_auction_params: OptionRFQParams,
    pub spot_auction_params: SpotAuctionParams,
}

impl LongPPParams {
    pub fn expiry_sec(&self) -> i64 {
        self.expiry_days as i64 * 86400
    }

    pub fn min_expiry_sec(&self) -> i64 {
        self.min_expiry_hours as i64 * 3600
    }

    pub fn option_auction_delay_sec(&self) -> i64 {
        self.option_auction_delay_min * 60
    }

    pub fn option_auction_start(&self, option_expiry: i64) -> i64 {
        option_expiry - self.expiry_sec() + self.option_auction_delay_sec()
    }

    pub fn spot_instrument_name(&self) -> String {
        let spot_name = &self.option_auction_params.collat_name;
        let cash_name = &self.spot_auction_params.cash_name;
        format!("{}-{}", spot_name, cash_name)
    }
}

impl RFQStrategy for OptionRFQParams {
    async fn get_desired_unit_cost(
        &self,
        auction: &RFQAuction,
        start_sec: i64,
    ) -> Result<BigDecimal> {
        let spread = self.get_premium_spread(start_sec);
        let mark = auction.get_mark_unit_cost().await?;
        // TODO currently assumes we are long the call spreads, will generalize later
        let factor = BigDecimal::from_f64(1.0 + spread).unwrap();
        let uncapped_cost = mark * factor;
        let capped_cost = uncapped_cost.max(self.min_cost.clone()).min(self.max_cost.clone());
        Ok(capped_cost.with_scale_round(6, HalfEven))
    }

    async fn get_desired_lot_size(
        &self,
        auction: &RFQAuction,
        unit_cost: &BigDecimal,
    ) -> Result<BigDecimal> {
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
        let collat_balance = reader.get_amount(&self.collat_name);
        let dollar_growth = get_growth_between(
            &self.collat_name,
            &self.quote_name,
            &collat_balance,
            now - sec_to_expiry,
            now,
        )
        .await?;

        let size = (dollar_growth / unit_cost) - spread_balance;
        let num_rounded = (&size / &self.lot_rounding).with_scale_round(0, Down);
        let round_size = num_rounded * &self.lot_rounding;
        let lot_size = round_size.clone().min(self.lot_size.clone());
        info!("Desired size: {}, round size: {}, lot_size: {}", size, round_size, lot_size.clone());
        Ok(lot_size)
    }
}
