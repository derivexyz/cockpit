/*
Defines core shared state of the market.
Public and private modules define logic for ws subscriptions that update the shared state.
*/
use bigdecimal::{BigDecimal, Zero};
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use lyra_client::actions::{Direction, OrderResponse, OrderStatus};
use orderbook_types::generated::channel_orderbook_instrument_name_group_depth::OrderbookInstrumentNameGroupDepthPublisherDataSchema;
use orderbook_types::types::orders::{TradeResponse, TxStatus};
use orderbook_types::types::tickers::result::InstrumentTicker;

pub type OrderbookData = OrderbookInstrumentNameGroupDepthPublisherDataSchema;

#[derive(Serialize, Deserialize, Debug)]
pub struct Balance {
    pub instrument_name: String,
    pub amount: BigDecimal,
    pub timestamp: i64,
}

pub type MarketState = Arc<RwLock<MarketData>>;

pub struct MarketData {
    tickers: HashMap<String, InstrumentTicker>,
    orderbooks: HashMap<String, OrderbookData>,
    positions: HashMap<String, Balance>,
    orders: HashMap<String, HashMap<String, OrderResponse>>,
    trades: HashMap<String, HashMap<String, TradeResponse>>,
}

const STALENESS_MS: i64 = 2_000; // todo ideally want to log the staleness

impl MarketData {
    pub fn new() -> Self {
        MarketData {
            tickers: HashMap::new(),
            orderbooks: HashMap::new(),
            positions: HashMap::new(),
            orders: HashMap::new(),
            trades: HashMap::new(),
        }
    }
    pub fn get_orderbook(&self, instrument_name: &str) -> Option<&OrderbookData> {
        let orderbook = self.orderbooks.get(instrument_name);
        let is_stale = orderbook
            .map_or(true, |o| chrono::Utc::now().timestamp_millis() - o.timestamp > STALENESS_MS);
        match is_stale {
            true => None,
            false => orderbook,
        }
    }
    pub fn insert_orderbook(&mut self, orderbook: OrderbookData) {
        self.orderbooks.insert(orderbook.instrument_name.clone(), orderbook);
    }
    pub fn iter_orderbooks(&self) -> impl Iterator<Item = &OrderbookData> {
        self.orderbooks.values()
    }
    pub fn get_ticker(&self, instrument_name: &str) -> Option<&InstrumentTicker> {
        let ticker = self.tickers.get(instrument_name);
        let is_stale = ticker
            .map_or(true, |t| chrono::Utc::now().timestamp_millis() - t.timestamp > STALENESS_MS);
        match is_stale {
            true => None,
            false => ticker,
        }
    }
    pub fn insert_ticker(&mut self, ticker: InstrumentTicker) {
        self.tickers.insert(ticker.instrument_name.clone(), ticker);
    }
    pub fn iter_tickers(&self) -> impl Iterator<Item = &InstrumentTicker> {
        self.tickers.values()
    }
    pub fn get_position(&self, instrument_name: &str) -> Option<&Balance> {
        self.positions.get(instrument_name)
    }
    pub fn get_amount(&self, instrument_name: &str) -> BigDecimal {
        self.positions.get(instrument_name).map_or(BigDecimal::zero(), |p| p.amount.clone())
    }
    pub fn insert_position(&mut self, position: Balance) {
        self.positions.insert(position.instrument_name.clone(), position);
    }
    pub fn iter_positions(&self) -> impl Iterator<Item = &Balance> {
        self.positions.values()
    }
    pub fn get_orders(&self, instrument_name: &str) -> Option<&HashMap<String, OrderResponse>> {
        self.orders.get(instrument_name)
    }
    pub fn insert_order(&mut self, order: OrderResponse) {
        let orders = self.orders.entry(order.instrument_name.clone()).or_default();
        let order_id = order.order_id.clone();
        let existing = orders.remove(&order_id);
        if let Some(existing) = existing {
            // insert if new is newer and status is open
            let is_newer = existing.last_update_timestamp < order.last_update_timestamp;
            if (order.order_status == OrderStatus::Open) && is_newer {
                orders.insert(order_id, order);
            } else if is_newer {
                return; // received filled, expired or cancelled - so keep the order removed
            } else {
                orders.insert(order_id, existing);
            }
        } else if order.order_status == OrderStatus::Open {
            orders.insert(order_id, order);
        }
    }
    pub fn iter_orders(&self) -> impl Iterator<Item = &HashMap<String, OrderResponse>> {
        self.orders.values()
    }
    pub fn get_orderbook_exclude_my_orders(&self, instrument_name: &str) -> Option<OrderbookData> {
        let ob = self.get_orderbook(instrument_name)?;
        let orders = self.get_orders(instrument_name);
        let mut ob = ob.clone();
        if orders.is_none() {
            return Some(ob);
        }
        let orders = orders.unwrap();
        for order in orders.values() {
            let bids_or_asks = match order.direction {
                Direction::Buy => &mut ob.bids,
                Direction::Sell => &mut ob.asks,
            };
            for level in bids_or_asks.iter_mut() {
                if level[0] != order.limit_price {
                    continue;
                }
                let remain_amount = &order.amount - &order.filled_amount;
                if level[1] > remain_amount {
                    level[1] -= &remain_amount;
                } else {
                    level[1] = BigDecimal::zero();
                }
            }
            bids_or_asks.retain(|x| x[1] > BigDecimal::zero());
        }
        Some(ob)
    }
    pub fn get_trades(&self, instrument_name: &str) -> Option<&HashMap<String, TradeResponse>> {
        self.trades.get(instrument_name)
    }
    pub fn insert_trade(&mut self, trade: TradeResponse) {
        let trades = self.trades.entry(trade.instrument_name.clone()).or_default();
        trades.insert(trade.trade_id.clone(), trade);
    }
    pub fn all_trades_confirmed(&self, instrument_name: &str) -> bool {
        let trades = self.get_trades(instrument_name);
        match trades {
            Some(trades) => trades
                .values()
                .all(|t| t.tx_status == TxStatus::Settled || t.tx_status == TxStatus::Reverted),
            None => true,
        }
    }
    pub fn is_balance_synced_to_trades(&self, instrument_name: &str) -> bool {
        let trades = self.get_trades(instrument_name);
        let position = self.get_position(instrument_name);
        match (trades, position) {
            (Some(trades), Some(position)) => {
                let total_trade_amount: BigDecimal = trades
                    .values()
                    .map(|t| match t.direction {
                        Direction::Buy => t.trade_amount.clone(),
                        Direction::Sell => -t.trade_amount.clone(),
                    })
                    .sum();
                total_trade_amount == position.amount
            }
            _ => false,
        }
    }
    pub fn log_state(&self) {
        info!("Market state:");
        info!("Tickers:");
        for ticker in self.iter_tickers() {
            info!("{:?}", ticker);
        }
        info!("Orderbooks:");
        for orderbook in self.iter_orderbooks() {
            info!("{:?}", orderbook);
        }
        info!("Positions:");
        for position in self.iter_positions() {
            info!("{:?}", position);
        }
        info!("Orders:");
        for orders in self.iter_orders() {
            for order in orders.values() {
                info!("{:?}", order);
            }
        }
        info!("Trades:");
        for trades in self.trades.values() {
            for trade in trades.values() {
                info!("{:?}", trade);
            }
        }
    }
}

pub fn new_market_state() -> MarketState {
    Arc::new(RwLock::new(MarketData::new()))
}
