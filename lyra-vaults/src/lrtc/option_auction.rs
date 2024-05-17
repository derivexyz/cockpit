use crate::lrtc::params::LRTCParams;
use crate::lrtc::selector::select_option;
use crate::market::{new_market_state, MarketState};
use crate::shared::{subscribe_subaccount, subscribe_tickers, sync_subaccount, TickerInterval};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use log::{info, warn};
use lyra_utils::black76::OptionContract;
use orderbook_types::types::tickers::OptionType;
use std::str::FromStr;
use tokio::select;

/// State struct for the LRT-C vault.
pub struct LRTCOptionExecutor {
    pub params: LRTCParams,
    pub option_name: String,
    pub desired_price: BigDecimal,
    pub market: MarketState,
    pub start_timestamp_sec: i64,
}

impl LRTCOptionExecutor {
    pub async fn from_params(params: LRTCParams) -> Result<Self> {
        let option_name = select_option(&params).await?;
        info!("LRTCOptionExecutor selected option: {}", option_name);
        let market = new_market_state();
        let desired_price = BigDecimal::from_str("0.0").unwrap();
        let start_timestamp_sec = chrono::Utc::now().timestamp();
        Ok(LRTCOptionExecutor { params, option_name, desired_price, market, start_timestamp_sec })
    }

    pub async fn run(&self) -> Result<()> {
        let market = new_market_state();
        let market_task = self.run_market(market.clone());
        let auction_task = self.run_auction(market.clone());
        let res = select! {
            _ = market_task => {Err(Error::msg("Market task exited early"))},
            auction_res = auction_task => {auction_res},
        };
        res
    }

    pub async fn run_market(&self, market: MarketState) -> Result<()> {
        let sync_instruments = vec![self.option_name.clone()];
        sync_subaccount(market.clone(), self.params.subaccount_id, sync_instruments).await?;

        let subacc_sub = subscribe_subaccount(market.clone(), self.params.subaccount_id);
        let ticker_sub = subscribe_tickers(
            market.clone(),
            vec![self.option_name.clone()],
            TickerInterval::_1000Ms,
        );
        let res = select! {
            _ = ticker_sub => {Err(Error::msg("Market subscription exited early"))},
            _ = subacc_sub => {Err(Error::msg("Subaccount subscription exited early"))},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(self.params.option_auction_sec)) => {Ok(())},
        };

        warn!("LRTCOptionExecutor run_market finished with {:?}", res);
        res
    }

    async fn get_desired_price(&self, market: MarketState) -> Result<BigDecimal> {
        let reader = market.read().await;
        let ticker = reader.get_ticker(&self.option_name).ok_or(Error::msg("Ticker not found"))?;
        let details = ticker.option_details.as_ref().unwrap();
        let pricing = ticker.option_pricing.as_ref().unwrap();
        let iv: f64 = pricing.iv.to_f64().ok_or(Error::msg("IV cast to f64 failed"))?;

        let contract = OptionContract {
            strike: details.strike.to_f64().unwrap(),
            expiry_sec: (details.expiry - chrono::Utc::now().timestamp()) as f64,
            is_call: details.option_type == OptionType::C,
        };
        // todo spread
        let fwd = pricing.forward_price.to_f64().ok_or(Error::msg("fwd cast to f64 failed"))?;
        let price = contract.price(fwd, iv);
        let price =
            BigDecimal::from_f64(price).unwrap().round(ticker.tick_size.fractional_digit_count());
        Ok(price)
    }

    /// Executes an option auction. Assumes market is already running and has correct state.
    pub async fn run_auction(&self, market: MarketState) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // wait for tickers
        let desired_price = self.get_desired_price(market.clone()).await?;
        warn!("LRTCOptionExecutor run_auction desired price: {}", desired_price);
        Ok(())
    }

    pub async fn next(&mut self) {}
}
