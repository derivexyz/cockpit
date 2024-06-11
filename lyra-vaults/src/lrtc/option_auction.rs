use crate::lrtc::auction::{LimitOrderAuction, OrderStrategy};
use crate::lrtc::params::OptionAuctionParams;
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, RoundingMode, ToPrimitive, Zero};
use log::{debug, info};
use lyra_client::actions::Direction;
use lyra_utils::black76::OptionContract;
use orderbook_types::types::tickers::OptionType;

impl OrderStrategy for OptionAuctionParams {
    async fn get_desired_price(&self, auction: &LimitOrderAuction) -> Result<BigDecimal> {
        let market = &auction.market;
        let reader = market.read().await;
        let ticker =
            reader.get_ticker(&auction.instrument_name).ok_or(Error::msg("Ticker not found"))?;
        let details = ticker.option_details.as_ref().unwrap();
        let pricing = ticker.option_pricing.as_ref().unwrap();
        let mark_iv: f64 = pricing.iv.to_f64().ok_or(Error::msg("IV cast to f64 failed"))?;
        let spread = self.get_iv_spread(auction.start_timestamp_sec);
        let iv = mark_iv * (1.0 - spread);

        let contract = OptionContract {
            strike: details.strike.to_f64().unwrap(),
            expiry_sec: (details.expiry - chrono::Utc::now().timestamp()) as f64,
            is_call: details.option_type == OptionType::C,
        };

        let fwd = pricing.forward_price.to_f64().ok_or(Error::msg("fwd cast to f64 failed"))?;

        debug!("OptionAuction mark_iv, spread, iv, fwd: {}, {}, {}, {}", mark_iv, spread, iv, fwd);

        let price = contract.price(fwd, iv);
        let price = BigDecimal::from_f64(price)
            .unwrap()
            .round(ticker.tick_size.fractional_digit_count())
            .max(ticker.min_price.clone());

        Ok(price)
    }
    async fn get_desired_amount(
        &self,
        auction: &LimitOrderAuction,
        price: &BigDecimal,
    ) -> Result<(Direction, BigDecimal)> {
        let market = &auction.market;
        let reader = market.read().await;
        let ticker =
            reader.get_ticker(&auction.instrument_name).ok_or(Error::msg("Ticker not found"))?;
        let lrt_pos = reader.get_position(&self.spot_name);
        let option_pos = reader.get_position(&auction.instrument_name);
        let amount = match (lrt_pos, option_pos) {
            (Some(lrt_pos), Some(option_pos)) => lrt_pos.amount.clone() + option_pos.amount.clone(),
            (Some(lrt_pos), None) => lrt_pos.amount.clone(),
            _ => {
                return Err(Error::msg("Zero LRT position during option auction"));
            }
        };
        let amount = amount
            .with_scale_round(ticker.amount_step.fractional_digit_count(), RoundingMode::Down);
        if amount < ticker.minimum_amount.clone() {
            return Ok((Direction::Sell, BigDecimal::zero()));
        }
        Ok((Direction::Sell, amount))
    }
}
