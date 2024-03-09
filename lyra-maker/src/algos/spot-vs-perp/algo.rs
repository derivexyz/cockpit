use std::str::FromStr;
use bigdecimal::BigDecimal;
use anyhow::{Error, Result};
use lyra_client::json_rpc::{http_rpc, Response, WsClient, WsClientExt};
use lyra_client::orders::{OrderType, OrderArgs, Direction, TimeInForce, OrderTicker, LiquidityRole};
use log::{info, warn, error, debug};
use uuid::Uuid;
use orderbook_types::generated::channel_subaccount_id_orders;
use crate::market::core::{MarketState, OrderbookData, TickerData};

/// Performs market making on the spot market and hedges via perps
/// The basic logic involves:

/// hedge costs -> this is a conservative estimate for how much the hedge would cost
/// picks "WORST" between perp + fees + slippage risk ("spread") and some index-based guard rails 
/// (in case perp trades very far away from spot)
/// perp            ------|--|-----
/// perp + spread   ----|------|---
/// index min       ---|-----|-----
/// pick worst:
/// hedge proxy     ---|-------|---

/// index max vs spot - dime -> pick "BEST" between the two, meant to represent true spot liquidity
/// this is used as "edge competitive bid / ask" - i.e. no reason to quote better than this 
/// index max       |-----------|--
/// spot - dime     ----|----------
/// pick best:
/// spot proxy:     ----|-------|--

/// finally, the algo quotes AT MOST as tight as spot proxy and AT LEAST as wide as hedge proxy
/// i.e. bid = min(spot bid, hedge bid), ask = max(spot ask, hedge ask)
/// hedge proxy     ---|-------|---
/// spot proxy:     ----|-------|--
/// quotes:         ---|--------|--


pub struct MakerAlgo {
    pub subaccount_id: i64,
    pub spot_name: String,
    pub perp_name: String,
    /// the quoted spot bid / ask will always be at least this far away from index,
    /// even if the perp bid / ask spread is very tight
    pub min_index_spread: BigDecimal,
    /// if spot is not quoted by other MMs and only perps are available, will be assuming that
    /// spot should be traded around max_index_spread
    pub max_index_spread: BigDecimal,
    /// spread to apply to perp prices (after fees) to account for slippage, risk and profit 
    pub hedge_spread: BigDecimal,
    /// try to quote this much size on both bid and ask (subject to hedge liquidity)
    /// in calculating perp bid / ask, will "walk the book" and compute avg fill based on this size
    pub size: BigDecimal,
    /// target delta to maintain (e.g. if ETH is borrowed, can set this to debt amount)
    pub target_delta: BigDecimal,
    /// attempt to execute the hedge over twap_ms milliseconds
    pub twap_ms: u64
    // todo asymmetric params (allow more bid size or different spread to bid or ask)
}

impl MakerAlgo {

    /// Returns (price, max size) for the hedge, incl. hedge spread and fees
    fn get_hedge(&self, orderbook: &OrderbookData, ticker: &TickerData, hedge_direction: Direction) -> (BigDecimal, BigDecimal) {
        let ticks = match hedge_direction {
            Direction::Buy => &orderbook.asks,
            Direction::Sell => &orderbook.bids
        };
        let mut total_cost = BigDecimal::from(0);
        let mut total_size = BigDecimal::from(0);
        for tick in ticks {
            if total_size >= self.size {
                break;
            }
            let remain_size = &self.size - &total_size;
            let price = &tick[0];
            let amount = if &remain_size < &tick[1] { &remain_size } else { &tick[1] };
            total_cost += price * amount;
            total_size += amount;
        }
        if total_size == BigDecimal::from(0) {
            return (BigDecimal::from(0), BigDecimal::from(0));
        }
        let total_fee = ticker.get_unit_fee(LiquidityRole::Taker) * &total_size;
        let dollar_spread = &ticker.index_price * &self.hedge_spread * &total_size;
        match hedge_direction {
            Direction::Buy => {total_cost += total_fee + dollar_spread},
            Direction::Sell => {total_cost -= total_fee + dollar_spread}
        }
        let price = &total_cost/&total_size;
        let price = match hedge_direction {
            Direction::Buy => {
                let price_ceil = &ticker.index_price * (BigDecimal::from(1) + &self.min_index_spread);
                if price < price_ceil {price_ceil} else {price}
            },
            Direction::Sell => {
                let price_floor = &ticker.index_price * (BigDecimal::from(1) - &self.min_index_spread);
                if price > price_floor {price_floor} else {price}
            }
        };
        (price.round(ticker.tick_size.digits() as i64), total_size)
    }

    async fn get_open_ids(&self, state: MarketState, direction: Direction) -> Vec<(String, BigDecimal, BigDecimal)> {
        let open_ids;
        let reader = state.read().await;
        let orders = reader.get_orders(&self.spot_name);
        if let Some(orders) = orders {
            open_ids = orders.values().filter(|o| o.direction == direction).map(|o| (o.order_id.clone(), o.limit_price.clone(), o.amount.clone())).collect();
        } else {
            open_ids = Vec::new();
        }
        open_ids
    }

    async fn maker_action(&self, state: MarketState, client: &WsClient, limit_price: BigDecimal, amount: BigDecimal, direction: Direction) -> Result<()> {
        if amount == BigDecimal::from(0) {
            return Ok(());
        }
        let order_args = OrderArgs {
            amount: amount.clone(),
            limit_price: limit_price.clone(),
            direction,
            time_in_force: TimeInForce::Gtc,
            order_type: OrderType::Limit,
            label: "spot-maker".to_string(),
            mmp: false
        };
        let open_ids = self.get_open_ids(state.clone(), direction).await;
        let data = state.read().await;
        let ticker = data.get_ticker(&self.spot_name).ok_or(Error::msg("Ticker not found"))?.clone();
        drop(data);
        match open_ids.len() {
            0 => {
                let _ = client.send_order(&ticker, self.subaccount_id, order_args).await?;
            },
            1 => {
                if open_ids[0].1 == limit_price && open_ids[0].2 == amount {
                    return Ok(());
                }
                let to_cancel = Uuid::from_str(&open_ids[0].0)?;
                let _ = client.send_replace(&ticker, self.subaccount_id, to_cancel, order_args).await?;
            },
            _ => {
                let _ = client.cancel_all(self.subaccount_id).await?;
                info!("Open orders: {:?}", open_ids);
            }
        }
        Ok(())
    }

    pub async fn start_maker(&self, state: MarketState) -> Result<()> {
        let client = WsClient::new_client().await?;
        client.login().await?;
        client.enable_cancel_on_disconnect().await?;
        loop {
            let (hedge_bid, hedge_bid_size);
            let (hedge_ask, hedge_ask_size);
            let reader = state.read().await;
            let perp_ticker = reader.get_ticker(&self.perp_name).expect("Perp ticker not found");
            let perp_ob = reader.get_orderbook(&self.perp_name).expect("Perp orderbook not found");
            (hedge_bid, hedge_bid_size) = self.get_hedge(&perp_ob, &perp_ticker, Direction::Sell);
            (hedge_ask, hedge_ask_size) = self.get_hedge(&perp_ob, &perp_ticker, Direction::Buy);
            drop(reader);
            info!("Hedge bid: {} @ {}", hedge_bid_size, hedge_bid);
            info!("Hedge ask: {} @ {}", hedge_ask_size, hedge_ask);

            let bid_action = self.maker_action(state.clone(), &client, hedge_bid, hedge_bid_size, Direction::Buy);
            let ask_action = self.maker_action(state.clone(), &client, hedge_ask, hedge_ask_size, Direction::Sell);
            tokio::join!(bid_action, ask_action);

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        Ok(())
    }

    pub async fn start_hedger(&self, state: MarketState) -> Result<()> {
        let client = WsClient::new_client().await?;
        let perp_ticker = state.read().await.get_ticker(&self.perp_name).expect("Perp ticker not found");

        Ok(())
    }

}
