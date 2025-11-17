use crate::helpers::{
    fetch_instrument, fetch_instruments, sleep_till, subscribe_subaccount, subscribe_tickers,
    sync_subaccount, TickerInterval,
};
use crate::market::{new_market_state, MarketState};
use crate::web3::{get_tsa_contract, sign_execute_quote, sign_order, ProviderWithSigner, TSA};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, Zero};
use core::fmt;
use ethers::prelude::Middleware;
use log::{error, info, warn};
use lyra_client::actions::rfq::{LegUnpriced, QuoteResultPublic};
use lyra_client::json_rpc::{Response, WsClient, WsClientExt};
use orderbook_types::types::rfqs::{
    Direction, GetRFQsResponse, OrderStatus, PollQuotesResponse, PollQuotesResult,
    RFQResponsePrivate, RFQResultPrivate,
};

use orderbook_types::types::tickers::InstrumentTicker;
use serde_json::{json, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::select;
use tokio::sync::Mutex;

/// An RFQ auction is executed by taking a large desired total size and executing it over
/// multiple RFQs in smaller lots. Each lot has size of at most `lot_size`. Any remainder
/// gets RFQ'd as a final partial lot.
/// For each individual RFQ lot, the goals are:
/// - make sure all market makers have a chance to participate
/// - make sure each lot does not take too long (esp. if MMs do manual work to price via UI)
/// - make sure the execution cost is reasonable

const POLL_INTERVAL_MS: u64 = 500;
const BASE_FREEZE_SEC: i64 = 4;

pub trait RFQStrategy {
    /// Implement specific pricing logic for specific spreads (e.g. for long calls spreads, etc.)
    async fn get_desired_unit_cost(
        &self,
        auction: &RFQAuction,
        start_sec: i64,
    ) -> Result<BigDecimal>;
    /// Returns the TOTAL number of spreads to request the RFQs for (e.g. 6969 call spreads)
    /// before splitting into sub-lots. Will get re-computed with more accurate unit prices
    /// as the fill info comes in for the first few lots.
    async fn get_desired_lot_size(
        &self,
        auction: &RFQAuction,
        unit_cost: &BigDecimal,
    ) -> Result<BigDecimal>;
}

/// State struct for a single RFQ lot
#[derive(Debug)]
pub struct RFQLot {
    pub rfq: RFQResultPrivate,
    pub size: BigDecimal,
    quotes: Vec<QuoteResultPublic>,
    timeouts: HashMap<String, (i64, u32)>,
}

impl RFQLot {
    pub fn new(rfq: RFQResultPrivate, size: BigDecimal) -> Self {
        Self { rfq, size, quotes: vec![], timeouts: HashMap::new() }
    }
    /// Sorts quotes with the largest total cost first. Also filters out quotes to be sell only.
    pub fn sort_quotes(mut quotes: Vec<QuoteResultPublic>) -> Vec<QuoteResultPublic> {
        quotes.retain(|quote| quote.direction == Direction::Sell);
        quotes.sort_by(|a, b| b.total_cost().partial_cmp(&a.total_cost()).unwrap());
        quotes
    }
    pub fn update_quotes(&mut self, quotes: Vec<QuoteResultPublic>) {
        self.quotes = Self::sort_quotes(quotes);
    }
    pub fn update_rfq(&mut self, rfq: RFQResultPrivate) {
        self.rfq = rfq;
    }
    /// Note: assumes quotes are sorted from large cost to small cost (from maker perspective)
    pub fn best_quote(&self) -> Option<&QuoteResultPublic> {
        /// filter out quotes that are frozen
        let now = chrono::Utc::now().timestamp();
        self.quotes
            .iter()
            .filter(|quote| {
                let (unfreeze, _) = self.timeouts.get(&quote.wallet).unwrap_or(&(0, 0));
                now > *unfreeze
            })
            .next()
    }
    pub fn status(&self) -> OrderStatus {
        self.rfq.status
    }
    pub fn start_sec(&self) -> i64 {
        let ms_time = self.rfq.creation_timestamp;
        ms_time / 1000
    }
    pub fn timeout(&mut self, wallet: &String) {
        let now = chrono::Utc::now().timestamp();
        let (unfreeze, count) = self.timeouts.entry(wallet.clone()).or_insert((0, 0));
        *unfreeze = now + BASE_FREEZE_SEC * 2_i64.pow(*count);
        *count += 1;
        warn!("RFQLot timeout wallet: {} for: {} count: {}", wallet, *unfreeze - now, count);
    }
}

/// State struct for an RFQ auction
pub struct RFQAuction {
    // State
    pub subaccount_id: i64,
    pub market: MarketState,
    pub client: WsClient,
    pub tsa: TSA<ProviderWithSigner>,
    pub start_timestamp_sec: i64,
    pub lots: Arc<Mutex<Vec<RFQLot>>>,

    // Params
    pub unit_legs: Vec<LegUnpriced>,
    pub lot_init_sleep_sec: u64,
    pub auction_sec: i64,
}

impl RFQAuction {
    pub async fn new(
        unit_legs: Vec<LegUnpriced>,
        start_sec: i64,
        lot_init_sleep_sec: u64,
        auction_sec: i64,
    ) -> Result<Self> {
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let subaccount_id = std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap();
        sleep_till(start_sec).await;
        let start_timestamp_sec = chrono::Utc::now().timestamp();

        let market = new_market_state();
        let client = WsClient::new_client().await?;
        client.login().await?;
        client.enable_cancel_on_disconnect().await?;
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(Self {
            subaccount_id,
            market,
            client,
            tsa,
            start_timestamp_sec,
            lots: Arc::new(Mutex::new(vec![])),
            unit_legs,
            auction_sec,
            lot_init_sleep_sec,
        })
    }
    pub fn remain_sec(&self) -> i64 {
        self.auction_sec - (chrono::Utc::now().timestamp() - self.start_timestamp_sec)
    }
    pub fn instrument_names(&self) -> Vec<String> {
        self.unit_legs.iter().map(|leg| leg.instrument_name.clone()).collect()
    }
    pub fn scaled_legs(&self, size: BigDecimal) -> Vec<LegUnpriced> {
        self.unit_legs
            .iter()
            .map(|leg| LegUnpriced {
                instrument_name: leg.instrument_name.clone(),
                amount: &size * &leg.amount,
                direction: leg.direction.clone(),
            })
            .collect()
    }
    pub async fn get_mark_unit_cost(&self) -> Result<BigDecimal> {
        let reader = self.market.read().await;
        let tickers = reader.get_tickers();
        get_legs_mark_unit_cost(&self.unit_legs, tickers)
    }
    pub async fn update_current_rfq(&self, rfq: RFQResultPrivate) {
        let mut lots = self.lots.lock().await;
        lots.last_mut().unwrap().update_rfq(rfq);
    }
    pub async fn update_current_quotes(&self, quotes: Vec<QuoteResultPublic>) {
        let mut lots = self.lots.lock().await;
        lots.last_mut().unwrap().update_quotes(quotes);
    }
}

#[derive(Debug)]
pub struct RFQAuctionExecutor<S: RFQStrategy + Debug> {
    pub auction: RFQAuction,
    pub strategy: S,
}

impl<S: RFQStrategy + Debug> RFQAuctionExecutor<S> {
    pub async fn run_market(&self) -> Result<()> {
        let market = &self.auction.market;
        let instrument_names = self.auction.instrument_names();
        let instruments = fetch_instruments(&instrument_names).await?;
        sync_subaccount(market.clone(), self.auction.subaccount_id, instrument_names.clone())
            .await?;

        let subacc_sub = subscribe_subaccount(market.clone(), self.auction.subaccount_id);
        let ticker_sub = subscribe_tickers(market.clone(), instruments, TickerInterval::_100Ms);

        let res = select! {
            _ = ticker_sub => {Err(Error::msg("Market subscription exited early"))},
            _ = subacc_sub => {Err(Error::msg("Subaccount subscription exited early"))},
        };

        warn!("RFQAuctionExecutor run_market finished with {:?}", res);
        res
    }
    async fn wait_for_tickers(&self) {
        let market = &self.auction.market;
        loop {
            let reader = market.read().await;
            let tickers_exist: Vec<bool> = self
                .auction
                .instrument_names()
                .iter()
                .map(|name| reader.get_ticker(name).is_some())
                .collect();
            if tickers_exist.iter().all(|x| *x) {
                break;
            }
            drop(reader);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    pub async fn create_new_lot(&self, size: BigDecimal) -> Result<()> {
        let rfq = self
            .auction
            .client
            .send_rpc::<Value, RFQResponsePrivate>(
                "private/send_rfq",
                json!({
                    "subaccount_id": self.auction.subaccount_id,
                    "legs": self.auction.scaled_legs(size.clone()),
                }),
            )
            .await?
            .into_result()?;
        let lot = RFQLot::new(rfq.result, size);
        info!("RFQAuctionExecutor new lot created");
        let mut lots = self.auction.lots.lock().await;
        lots.push(lot);
        Ok(())
    }
    async fn update_current_rfq(&self, current_lot: &mut RFQLot) -> Result<()> {
        let rfqs_resp = self
            .auction
            .client
            .send_rpc::<Value, GetRFQsResponse>(
                "private/get_rfqs",
                json!({
                    "subaccount_id": self.auction.subaccount_id,
                    "rfq_id": current_lot.rfq.rfq_id,
                }),
            )
            .await?
            .into_result()?;

        current_lot.update_rfq(rfqs_resp.result.rfqs[0].clone());
        Ok(())
    }

    async fn update_current_quotes(&self, current_lot: &mut RFQLot) -> Result<()> {
        let quotes_resp = self
            .auction
            .client
            .send_rpc::<Value, PollQuotesResponse>(
                "private/poll_quotes",
                json!({
                    "subaccount_id": self.auction.subaccount_id,
                    "rfq_id": current_lot.rfq.rfq_id,
                    "status": "open"
                }),
            )
            .await?
            .into_result()?;

        current_lot.update_quotes(quotes_resp.result.quotes);
        Ok(())
    }

    async fn update_current_lot(&self) -> Result<OrderStatus> {
        let mut lots = self.auction.lots.lock().await;
        let current_lot = lots.last_mut().unwrap();
        self.update_current_rfq(current_lot).await?;
        self.update_current_quotes(current_lot).await?;
        Ok(current_lot.status())
    }

    /// Attempts to execute the current lot. Will panic if called when no lot is created.
    /// Returns error if intermediate RPC call fails.
    /// Returns Ok(None) if no quotes are found for the current lot within the desired cost.
    /// Returns Ok(None) if private/execute_quote (e.g. quote got cancelled last second)
    /// Returns Ok(Some(Value)) if the lot is executed successfully.
    async fn maybe_execute_lot(&self) -> Result<Option<Value>> {
        let mut lots = self.auction.lots.lock().await;
        let mut current_lot = lots.last_mut().unwrap();
        let unit_cost =
            self.strategy.get_desired_unit_cost(&self.auction, current_lot.start_sec()).await?;
        let desired_cost = &unit_cost * &current_lot.size;
        info!("RFQAuctionExecutor unit and desired costs: {}, {}", unit_cost, desired_cost);
        let best_quote = current_lot.best_quote();
        if best_quote.is_none() {
            info!("RFQAuctionExecutor no quotes found for current lot");
            return Ok(None);
        }
        let best_quote = best_quote.unwrap();
        let best_cost = -best_quote.total_cost();
        info!("RFQAuctionExecutor best quote cost: {} vs. desired {}", best_cost, desired_cost);
        if best_cost > desired_cost {
            info!("RFQ best quote cost too high. Cost: {}", best_cost);
            Ok(None)
        } else {
            let provider = self.auction.tsa.client();
            let signer = provider.inner().signer();
            let reader = self.auction.market.read().await;
            let tickers = reader.get_tickers();
            let action_data = sign_execute_quote(&self.auction.tsa, &tickers, &best_quote).await?;
            let execute_params = action_data.to_execute_params(&signer, tickers, best_quote)?;
            let send_resp = self
                .auction
                .client
                .send_rpc::<_, Value>("private/execute_quote", execute_params)
                .await?;
            return match send_resp {
                Response::Success(v) => Ok(Some(v)),
                Response::Error(e) => {
                    error!("RFQAuctionExecutor send_execute failed with {:#}", e);
                    match e.error.code {
                        11104 | 8501 | 8500 => {
                            let wallet = best_quote.wallet.clone();
                            current_lot.timeout(&wallet);
                            Ok(None)
                        }
                        _ => Err(Error::new(e)),
                    }
                }
            };
        }
    }

    pub async fn run_auction(&self) -> Result<()> {
        self.wait_for_tickers().await;
        let mut unit_cost = self.auction.get_mark_unit_cost().await?;

        loop {
            self.sync().await;
            let lot_size = self.strategy.get_desired_lot_size(&self.auction, &unit_cost).await?;
            if lot_size <= BigDecimal::zero() {
                break;
            }

            self.cancel_all().await?;
            self.create_new_lot(lot_size.clone()).await?;
            let init_sleep = tokio::time::Duration::from_secs(self.auction.lot_init_sleep_sec);
            tokio::time::sleep(init_sleep).await;

            loop {
                let status = self.update_current_lot().await?;
                info!("RFQAuctionExecutor current lot status: {:?}", status);
                if status != OrderStatus::Open {
                    warn!("RFQAuctionExecutor current lot status no longer open: {:?}", status);
                    break;
                }
                if let Some(_) = self.maybe_execute_lot().await? {
                    warn!("RFQAuctionExecutor current lot executed successfully");
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
            }

            let lots = self.auction.lots.lock().await;
            if let Some(quote) = lots.last().unwrap().best_quote() {
                let start = lots.last().unwrap().start_sec();
                let best_price = self.auction.get_mark_unit_cost().await?;
                let worst_price = self.strategy.get_desired_unit_cost(&self.auction, start).await?;
                let quote_price = -quote.total_cost() / &lot_size;
                unit_cost = best_price.max(quote_price).min(worst_price);
                info!("RFQAuctionExecutor new est_unit_cost: {}", unit_cost);
            }
        }

        Ok(())
    }
    async fn sync(&self) {
        loop {
            if self.is_synced().await {
                break;
            }
            warn!("RFQAuctionExecutor run_auction not synced, waiting for 1 sec");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    async fn is_synced(&self) -> bool {
        let market = &self.auction.market;
        let reader = market.read().await;
        self.auction.instrument_names().iter().all(|name| reader.all_trades_confirmed(name))
    }
    async fn cancel_all(&self) -> Result<()> {
        self.auction
            .client
            .send_rpc::<Value, Value>(
                "private/cancel_batch_rfqs",
                json!({"subaccount_id": self.auction.subaccount_id}),
            )
            .await?
            .into_result()?;
        Ok(())
    }
}

impl Debug for RFQAuction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RFQAuction")
            .field("unit_legs", &self.unit_legs)
            .field("lots", &self.lots)
            .field("market", &"MarketState")
            .field("client", &"WsClient")
            .field("start_timestamp_sec", &self.start_timestamp_sec)
            .field("duration_sec", &self.auction_sec)
            .finish()
    }
}

pub fn get_legs_mark_unit_cost(
    legs: &Vec<LegUnpriced>,
    tickers: &HashMap<String, InstrumentTicker>,
) -> Result<BigDecimal> {
    legs.iter()
        .map(|leg| {
            let ticker = tickers.get(&leg.instrument_name).ok_or(Error::msg("Ticker not found"))?;
            let price = ticker.mark_price.clone();
            Ok(price * &leg.amount * &leg.direction.sign())
        })
        .sum::<Result<BigDecimal>>()
}
