use crate::lrtc::params::LRTCParams;
use crate::lrtc::selector::select_option;
use crate::market::{new_market_state, MarketState};
use crate::shared::{subscribe_subaccount, subscribe_tickers, sync_subaccount, TickerInterval};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use log::{info, warn};
use lyra_client::json_rpc::{WsClient, WsClientExt};
use lyra_client::orders::{Direction, OrderArgs, OrderType, TimeInForce};
use lyra_utils::black76::OptionContract;
use orderbook_types::types::tickers::OptionType;
use std::str::FromStr;
use tokio::select;

/// State struct for the LRT-C option auction.
pub struct LRTCOptionExecutor {
    pub params: LRTCParams,
    pub option_name: String,
    pub market: MarketState,
    pub client: WsClient,
    pub start_timestamp_sec: i64,
}

impl LRTCOptionExecutor {
    pub async fn from_params(params: LRTCParams) -> Result<Self> {
        let option_name = select_option(&params).await?;
        info!("LRTCOptionExecutor selected option: {}", option_name);
        let market = new_market_state();
        let client = WsClient::new_client().await?;
        client.login().await?;
        client.enable_cancel_on_disconnect().await?;

        let start_timestamp_sec = chrono::Utc::now().timestamp();
        Ok(LRTCOptionExecutor { params, option_name, market, client, start_timestamp_sec })
    }

    pub async fn run(&self) -> Result<()> {
        let market_task = self.run_market();
        let auction_task = self.run_auction();
        let ping_task = self.client.ping_interval(15);
        let res = select! {
            _ = market_task => {Err(Error::msg("Market task exited early"))},
            _ = ping_task => {Err(Error::msg("Ping task exited early"))},
            auction_res = auction_task => { auction_res },
        };
        res
    }

    async fn run_market(&self) -> Result<()> {
        let market = &self.market;
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

    fn get_iv_spread(&self) -> f64 {
        let sec_since_start = chrono::Utc::now().timestamp() - self.start_timestamp_sec;
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.params.init_iv_spread + min_since_start * self.params.iv_spread_per_min;
        spread.min(self.params.max_iv_spread)
    }

    async fn get_desired_price(&self) -> Result<BigDecimal> {
        let market = &self.market;
        let reader = market.read().await;
        let ticker = reader.get_ticker(&self.option_name).ok_or(Error::msg("Ticker not found"))?;
        let details = ticker.option_details.as_ref().unwrap();
        let pricing = ticker.option_pricing.as_ref().unwrap();
        let mark_iv: f64 = pricing.iv.to_f64().ok_or(Error::msg("IV cast to f64 failed"))?;
        let spread = self.get_iv_spread();
        let iv = (mark_iv - spread).max(self.params.min_iv);
        info!("LRTCOptionExecutor mark_iv, spread, iv: {}, {}, {}", mark_iv, spread, iv);

        let contract = OptionContract {
            strike: details.strike.to_f64().unwrap(),
            expiry_sec: (details.expiry - chrono::Utc::now().timestamp()) as f64,
            is_call: details.option_type == OptionType::C,
        };

        let fwd = pricing.forward_price.to_f64().ok_or(Error::msg("fwd cast to f64 failed"))?;
        let price = contract.price(fwd, iv);
        let price = BigDecimal::from_f64(price)
            .unwrap()
            .round(ticker.tick_size.fractional_digit_count())
            .max(ticker.min_price.clone());

        Ok(price)
    }

    async fn get_desired_amount(&self) -> Result<BigDecimal> {
        let market = &self.market;
        let reader = market.read().await;
        let ticker = reader.get_ticker(&self.option_name).ok_or(Error::msg("Ticker not found"))?;
        let lrt_pos = reader.get_position(&self.params.spot_name);
        let option_pos = reader.get_position(&self.option_name);
        let amount = match (lrt_pos, option_pos) {
            (Some(lrt_pos), Some(option_pos)) => lrt_pos.amount.clone() + option_pos.amount.clone(),
            (Some(lrt_pos), None) => lrt_pos.amount.clone(),
            _ => {
                return Err(Error::msg("Zero LRT position during option auction"));
            }
        };
        let amount = amount.round(ticker.amount_step.fractional_digit_count());
        if amount < ticker.minimum_amount.clone() {
            return Ok(BigDecimal::zero());
        }
        Ok(amount)
    }
    /// Executes an option auction. Assumes market is already running and has correct state.
    async fn run_auction(&self) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // wait for tickers
        loop {
            let desired_price = self.get_desired_price().await?;
            info!("LRTCOptionExecutor run_auction desired price: {}", desired_price);

            if self.needs_update(&desired_price).await? {
                let amount = self.update_order(&desired_price).await?;
                if amount.is_zero() {
                    return Ok(());
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    async fn sync(&self) {
        loop {
            if self.is_synced().await {
                break;
            }
            warn!("LRTCOptionExecutor run_auction not synced, waiting for 1 sec");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn is_synced(&self) -> bool {
        let market = &self.market;
        let reader = market.read().await;
        reader.all_trades_confirmed(&self.option_name)
    }

    async fn get_open_order_price(&self) -> Result<Option<BigDecimal>> {
        let market = &self.market;
        let reader = market.read().await;
        let orders = reader.get_orders(&self.option_name);
        match orders {
            None => Ok(None),
            Some(orders) => {
                let mut prices = orders.values().map(|o| o.limit_price.clone()).collect::<Vec<_>>();
                match prices.len() {
                    0 => Ok(None),
                    1 => Ok(Some(prices[0].clone())),
                    _ => {
                        self.cancel_all().await?;
                        Ok(None)
                    }
                }
            }
        }
    }

    async fn cancel_all(&self) -> Result<()> {
        self.client.cancel_all(self.params.subaccount_id).await?.into_result()?;
        Ok(())
        // todo on-chain cancel signature
    }

    async fn needs_update(&self, desired_price: &BigDecimal) -> Result<bool> {
        let open_price = self.get_open_order_price().await?;
        info!("LRTCOptionExecutor run_auction open price: {:?}", open_price);
        match open_price {
            None => Ok(true),
            Some(open_price) => {
                Ok((&open_price - desired_price).abs() > self.params.option_price_change_tolerance)
            }
        }
    }

    async fn update_order(&self, desired_price: &BigDecimal) -> Result<BigDecimal> {
        self.cancel_all().await?;
        self.sync().await;
        let desired_amount = self.get_desired_amount().await?;
        info!("LRTCOptionExecutor run_auction desired amount: {}", desired_amount);
        if desired_amount.is_zero() {
            return Ok(desired_amount);
        }

        let order_args = OrderArgs {
            amount: desired_amount.clone(),
            limit_price: desired_price.clone(),
            direction: Direction::Sell,
            time_in_force: TimeInForce::Gtc,
            order_type: OrderType::Limit,
            mmp: false,
            label: "lrtc-option-auction".to_string(),
        };

        info!("LRTCOptionExecutor run_auction sending order: {:?}", order_args);
        let market = &self.market;
        let reader = market.read().await;
        let ticker = reader.get_ticker(&self.option_name).ok_or(Error::msg("Ticker not found"))?;
        self.client
            .send_order(ticker, self.params.subaccount_id, order_args)
            .await?
            .into_result()?;
        Ok(desired_amount)
    }
}
