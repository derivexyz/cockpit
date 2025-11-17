use crate::helpers::{
    fetch_instrument, sleep_till, subscribe_subaccount, subscribe_tickers, sync_subaccount,
    TickerInterval,
};
use crate::market::{new_market_state, MarketState};
use crate::shared::stages::ExecutorStage;
use crate::web3::actions::{get_tsa_contract, sign_order, ProviderWithSigner, TSA};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive, Zero};
use core::fmt;
use ethers::prelude::Middleware;
use log::{info, warn};
use lyra_client::actions::{Direction, OrderArgs, OrderType, TimeInForce};
use lyra_client::json_rpc::{WsClient, WsClientExt};
use lyra_utils::black76::OptionContract;
use orderbook_types::types::tickers::OptionType;
use serde_json::Value;
use std::fmt::Debug;
use std::str::FromStr;
use tokio::select;

pub trait OrderStrategy {
    async fn get_desired_price(&self, auction: &LimitOrderAuction) -> Result<BigDecimal>;
    /// Returns the amount to trade and the direction to trade in
    /// Auction stops IF AND ONLY IF the amount returned is zero
    async fn get_desired_amount(
        &self,
        auction: &LimitOrderAuction,
        price: &BigDecimal,
    ) -> Result<(Direction, BigDecimal)>;
}

/// State struct for a limit order auction.
pub struct LimitOrderAuction {
    // State
    pub subaccount_id: i64,
    pub market: MarketState,
    pub client: WsClient,
    pub tsa: TSA<ProviderWithSigner>,
    pub start_timestamp_sec: i64,

    // Params
    pub instrument_name: String,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,
}

impl LimitOrderAuction {
    pub async fn new(
        instrument_name: String,
        start_sec: i64,
        auction_sec: i64,
        price_change_tolerance: BigDecimal,
    ) -> Result<Self> {
        info!("LimitOrderAuction selected option: {}", instrument_name);
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let subaccount_id = std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap();
        sleep_till(start_sec).await;

        let start_timestamp_sec = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let client = WsClient::new_client().await?;
        client.login().await?;
        client.enable_cancel_on_disconnect().await?;
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(LimitOrderAuction {
            subaccount_id,
            market,
            client,
            tsa,
            start_timestamp_sec,
            instrument_name,
            auction_sec,
            price_change_tolerance,
        })
    }
    pub fn remain_sec(&self) -> i64 {
        self.auction_sec - (chrono::Utc::now().timestamp() - self.start_timestamp_sec)
    }
}

impl Debug for LimitOrderAuction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LimitOrderAuction")
            .field("instrument_name", &self.instrument_name)
            .field("market", &"MarketState")
            .field("client", &"WsClient")
            .field("start_timestamp_sec", &self.start_timestamp_sec)
            .field("auction_sec", &self.auction_sec)
            .field("price_change_tolerance", &self.price_change_tolerance)
            .finish()
    }
}

#[derive(Debug)]
pub struct LimitOrderAuctionExecutor<S: OrderStrategy + Debug> {
    pub auction: LimitOrderAuction,
    pub strategy: S,
}

impl<S: OrderStrategy + Debug> LimitOrderAuctionExecutor<S> {
    pub async fn run_market(&self) -> Result<()> {
        let market = &self.auction.market;
        let sync_instruments = vec![self.auction.instrument_name.clone()];
        let instrument = fetch_instrument(&self.auction.instrument_name).await?;
        sync_subaccount(market.clone(), self.auction.subaccount_id, sync_instruments).await?;
        let subacc_sub = subscribe_subaccount(market.clone(), self.auction.subaccount_id);
        let ticker_sub =
            subscribe_tickers(market.clone(), vec![instrument], TickerInterval::_100Ms);

        let res = select! {
            _ = ticker_sub => {Err(Error::msg("Market subscription exited early"))},
            _ = subacc_sub => {Err(Error::msg("Subaccount subscription exited early"))},
        };

        warn!("LimitOrderAuction run_market finished with {:?}", res);
        res
    }

    async fn wait_for_ticker(&self) {
        let market = &self.auction.market;
        loop {
            let reader = market.read().await;
            if reader.get_ticker(&self.auction.instrument_name).is_some() {
                break;
            }
            drop(reader);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Executes an option auction. Assumes market is already running and has correct state.
    pub async fn run_auction(&self) -> Result<()> {
        self.wait_for_ticker().await;
        loop {
            let desired_price = self.strategy.get_desired_price(&self.auction).await?;
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
            warn!("LimitOrderAuction run_auction not synced, waiting for 1 sec");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }

    async fn is_synced(&self) -> bool {
        let market = &self.auction.market;
        let reader = market.read().await;
        reader.all_trades_confirmed(&self.auction.instrument_name)
    }

    async fn get_open_order_price(&self) -> Result<Option<BigDecimal>> {
        let market = &self.auction.market;
        let reader = market.read().await;
        let orders = reader.get_orders(&self.auction.instrument_name);
        match orders {
            None => Ok(None),
            Some(orders) => {
                let prices = orders.values().map(|o| o.limit_price.clone()).collect::<Vec<_>>();
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
        // Note: API migration,
        // this used to call private/cancel_all but it no longer returns # of cancelled orders
        let res = self
            .auction
            .client
            .cancel_by_instrument(self.auction.subaccount_id, self.auction.instrument_name.clone())
            .await?
            .into_result()?;
        if res.result.cancelled_orders == 0 {
            warn!("LimitOrderAuction cancel_all failed to cancel any orders, likely mid fill");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
        Ok(())
    }

    async fn needs_update(&self, desired_price: &BigDecimal) -> Result<bool> {
        let open_price = self.get_open_order_price().await?;
        match open_price {
            None => Ok(true),
            Some(open_price) => {
                Ok((&open_price - desired_price).abs() > self.auction.price_change_tolerance)
            }
        }
    }

    async fn update_order(&self, desired_price: &BigDecimal) -> Result<BigDecimal> {
        self.cancel_all().await?;
        self.sync().await;
        let (direction, amount) =
            self.strategy.get_desired_amount(&self.auction, desired_price).await?;
        info!("LimitOrderAuction desired price: {}", desired_price);
        info!("LimitOrderAuction {} desired amount: {}", direction.to_string(), amount);
        if amount.is_zero() {
            return Ok(amount);
        }

        let order_args = OrderArgs {
            amount: amount.clone(),
            limit_price: desired_price.clone(),
            direction,
            time_in_force: TimeInForce::Gtc,
            order_type: OrderType::Limit,
            mmp: false,
            label: "".to_string(),
        };

        info!("LimitOrderAuction run_auction sending order: {:?}", order_args);
        let market = &self.auction.market;
        let reader = market.read().await;
        let ticker = reader
            .get_ticker(&self.auction.instrument_name)
            .ok_or(Error::msg("Ticker not found"))?
            .clone();
        drop(reader);

        let provider = self.auction.tsa.client();
        let signer = provider.inner().signer();
        let action_data = sign_order(&self.auction.tsa, &ticker, &order_args).await?;
        let order_params = action_data.to_order_params(&signer, &ticker, order_args)?;
        let res = self.auction.client.send_rpc::<_, Value>("private/order", order_params).await?;
        res.into_result()?;
        Ok(amount)
    }
}
