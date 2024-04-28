use crate::market::core::{filter_open_ids, Balance, MarketState, OrderbookData, TickerData};
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use log::{debug, error, info, warn};
use lyra_client::json_rpc::{http_rpc, Response, WsClient, WsClientExt};
use lyra_client::orders::{
    Direction, LiquidityRole, OrderArgs, OrderTicker, OrderType, TimeInForce,
};
use orderbook_types::generated::channel_subaccount_id_orders;
use orderbook_types::types::orders::OrderResponse;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

/// this algo dynamically delta hedges a long gamma portfolio using post only limit orders
pub struct GammaDDHAlgo {
    pub subaccount_id: i64,
    pub perp_name: String,
    pub max_abs_delta: BigDecimal,

    pub action_wait_ms: u64,
    /// tolerance for replacing orders
    pub price_tol: BigDecimal,
    pub amount_tol: BigDecimal,
    /// max $ bid ask spread past which we do not trust the mid price
    pub mid_price_tol: BigDecimal,
}

#[derive(Debug)]
pub struct GammaDDHState {
    pub perp_ticker: TickerData,
    pub perp_best_bid: Option<BigDecimal>,
    pub perp_best_ask: Option<BigDecimal>,
    pub net_delta: BigDecimal,
    pub net_gamma: BigDecimal,
    pub bid_ids: Vec<(String, BigDecimal, BigDecimal)>,
    pub ask_ids: Vec<(String, BigDecimal, BigDecimal)>,
}

impl GammaDDHState {
    /// solves how much the index price needs to change to reach a target delta
    /// returns the $ change in the index price and the expected delta at that changed price
    pub fn get_index_change_to_target_delta(
        &self,
        target_delta: BigDecimal,
    ) -> (BigDecimal, BigDecimal) {
        let zero = BigDecimal::from(0);
        if target_delta > zero && self.net_delta > target_delta {
            (zero, self.net_delta.clone())
        } else if target_delta < zero && self.net_delta < target_delta {
            (zero, self.net_delta.clone())
        } else if self.net_gamma <= zero {
            (zero, self.net_delta.clone())
        } else {
            ((target_delta.clone() - &self.net_delta) / &self.net_gamma, target_delta)
        }
    }
    pub fn get_smooth_mid(&self, mid_price_tol: &BigDecimal) -> BigDecimal {
        match (&self.perp_best_bid, &self.perp_best_ask) {
            (Some(bid), Some(ask)) => {
                let spread = ask - bid;
                if &spread < mid_price_tol {
                    (bid + ask) / BigDecimal::from(2)
                } else {
                    self.perp_ticker.mark_price.clone()
                }
            }
            _ => self.perp_ticker.mark_price.clone(),
        }
    }
}

impl GammaDDHAlgo {
    async fn hedger_action(
        &self,
        state: &GammaDDHState,
        client: &WsClient,
        direction: Direction,
    ) -> Result<()> {
        let ticker = &state.perp_ticker;
        let target_delta = -self.max_abs_delta.clone() * direction.sign();
        let (d_index, delta) = state.get_index_change_to_target_delta(target_delta);
        let amount = -delta * direction.sign();
        if amount < ticker.minimum_amount {
            warn!("Amount calculated too small: {}", amount);
            return Ok(());
        }

        let limit_price = d_index + &ticker.mark_price;
        // prevent post only cross errors by clipping limit price to inside BBO
        let limit_price = match (direction, &state.perp_best_bid, &state.perp_best_ask) {
            (Direction::Buy, _, Some(ask)) => limit_price.min(ask - &ticker.tick_size),
            (Direction::Sell, Some(bid), _) => limit_price.max(bid + &ticker.tick_size),
            _ => limit_price,
        };
        // if limit price is too aggro (more aggro than mid) -> clip to mid (if mid is trustworthy)
        let mid_price = state.get_smooth_mid(&self.mid_price_tol);
        let limit_price = match direction {
            Direction::Buy => limit_price.min(mid_price),
            Direction::Sell => limit_price.max(mid_price),
        };

        let open_ids = match direction {
            Direction::Buy => &state.bid_ids,
            Direction::Sell => &state.ask_ids,
        };

        let order_args = OrderArgs {
            amount: amount.clone().round(ticker.amount_step.fractional_digit_count()),
            limit_price: limit_price.clone().round(ticker.tick_size.fractional_digit_count()),
            direction,
            time_in_force: TimeInForce::PostOnly,
            order_type: OrderType::Limit,
            label: "long-gamma-ddh".to_string(),
            mmp: false,
        };
        match open_ids.len() {
            0 => {
                let _ = client.send_order(ticker, self.subaccount_id, order_args).await?;
            }
            1 => {
                let is_price_new = (&open_ids[0].1 - &limit_price).abs() > self.price_tol;
                let is_amount_new = (&open_ids[0].2 - &amount).abs() > self.amount_tol;
                if !is_price_new && !is_amount_new {
                    client.ping().await?;
                    return Ok(());
                }
                let to_cancel = Uuid::from_str(&open_ids[0].0)?;
                let _ =
                    client.send_replace(ticker, self.subaccount_id, to_cancel, order_args).await?;
            }
            _ => {
                let _ = client.cancel_all(self.subaccount_id).await?;
                info!("Open orders: {:?}", open_ids);
            }
        }
        Ok(())
    }
    async fn get_algo_state(&self, market: MarketState) -> Result<GammaDDHState> {
        let data = market.read().await;
        let perp_ticker =
            data.get_ticker(&self.perp_name).ok_or(Error::msg("Perp ticker not found"))?;
        let perp_orderbook = data
            .get_orderbook_exclude_my_orders(&self.perp_name)
            .ok_or(Error::msg("Perp orderbook not found"))?;
        let orders = data.get_orders(&self.perp_name);
        let mut algo_state = GammaDDHState {
            perp_ticker: perp_ticker.clone(),
            perp_best_bid: perp_orderbook.bids.first().map(|x| x[0].clone()),
            perp_best_ask: perp_orderbook.asks.clone().first().map(|x| x[0].clone()),
            net_delta: BigDecimal::from(0),
            net_gamma: BigDecimal::from(0),
            bid_ids: filter_open_ids(orders, Direction::Buy),
            ask_ids: filter_open_ids(orders, Direction::Sell),
        };
        for position in data.iter_positions() {
            let ticker = data.get_ticker(&position.instrument_name);
            if let Some(ticker) = ticker {
                if let Some(pricing) = &ticker.option_pricing {
                    algo_state.net_gamma += &position.amount * &pricing.gamma;
                    algo_state.net_delta += &position.amount * &pricing.delta;
                } else if &ticker.instrument_name == &self.perp_name {
                    algo_state.net_delta += &position.amount;
                }
            }
        }
        info!("Net Delta: {}, Net Gamma: {}", algo_state.net_delta, algo_state.net_gamma);
        Ok(algo_state)
    }

    pub async fn start_hedger(&self, state: MarketState) -> Result<()> {
        info!("Starting maker task");
        let client = WsClient::new_client().await?;
        client.login().await?.into_result()?;
        client.enable_cancel_on_disconnect().await?.into_result()?;
        // TODO pings every x sec
        loop {
            let algo_state = self.get_algo_state(state.clone()).await?;
            let bid_action = self.hedger_action(&algo_state, &client, Direction::Buy);
            let ask_action = self.hedger_action(&algo_state, &client, Direction::Sell);
            let results = tokio::join!(bid_action, ask_action);
            if let Err(e) = results.0 {
                error!("Maker bid failed: {:?}", e);
            }
            if let Err(e) = results.1 {
                error!("Maker ask failed: {:?}", e);
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(self.action_wait_ms)).await;
        }
    }
}
