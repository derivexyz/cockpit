use crate::helpers::{fetch_ticker, get_option_expiry, sync_subaccount};
use crate::lrtc::params::{LRTCParams, OptionAuctionParams};
use crate::lrtc::selector::maybe_select_from_positions;
use crate::market::new_market_state;
use crate::shared::auction::{LimitOrderAuctionExecutor, OrderStrategy};
use crate::shared::params::SpotAuctionParams;
use crate::web3::{
    get_tsa_contract, process_deposits_forever, process_deposits_once, process_withdrawals,
    ProviderWithSigner, TSA,
};
use anyhow::{Error, Result};
use bigdecimal::Zero;
use log::{error, info, warn};
use lyra_client::auth::get_auth_headers;
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::types::tickers::TickerResponse;
use serde_json::json;
use std::fmt::Debug;
use tokio::select;

/// - This stage will process withdrawals for the spot asset.
/// - Will keep reconnecting in case of any errors raised during process_withdrawals.
/// - Will stop when total pending withdrawals are 0.
/// - TODO Note that there are also some edge cases when some stray USDC was not fully sold off and
/// withdrawals are so large that the vault has not enough LRT balance to cover it.
#[derive(Debug)]
pub struct LRTCSpotOnly {
    pub tsa: TSA<ProviderWithSigner>,
}

impl LRTCSpotOnly {
    pub async fn new() -> Result<Self> {
        info!("Starting LRTCSpotOnly Stage");
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(Self { tsa })
    }
}

impl LRTCStage for LRTCSpotOnly {
    async fn run(&self) -> Result<()> {
        let asset_name = std::env::var("SPOT_NAME").unwrap();
        process_deposits_once(&self.tsa, asset_name.clone()).await?;
        process_withdrawals(&self.tsa, asset_name.clone()).await?;
        process_deposits_once(&self.tsa, asset_name.clone()).await?;
        Ok(())
    }
    async fn reconnect(&mut self) -> Result<()> {
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        self.tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRTCAwaitSettlement {
    pub subaccount_id: i64,
    pub tsa: TSA<ProviderWithSigner>,
    pub option_name: String,
    pub option_expiry: i64,
    pub delay_min: i64,
}

impl LRTCAwaitSettlement {
    pub async fn new(params: LRTCParams, option_name: String) -> Result<Self> {
        let subaccount_id = std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap();
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        let option_expiry = get_option_expiry(&option_name).await?;
        Ok(Self {
            subaccount_id,
            tsa,
            option_name,
            option_expiry,
            delay_min: params.spot_auction_delay_min,
        })
    }
    pub async fn is_settled(&self) -> Result<bool> {
        // todo some of these might be cleaner to use REST for
        let market = new_market_state();
        sync_subaccount(market.clone(), self.subaccount_id, vec![]).await?;
        let option_name = maybe_select_from_positions(&market).await?;
        return Ok(option_name.is_none());
    }

    fn sec_to_auction(&self) -> i64 {
        let sec_to_expiry = self.option_expiry - chrono::Utc::now().timestamp();
        let sec_to_auction = sec_to_expiry + self.delay_min * 60;
        sec_to_auction
    }

    async fn wait_for_auction(&self) -> Result<()> {
        let heartbeat_sec = 600;
        let mut sleep_sec = self.sec_to_auction().min(heartbeat_sec);
        while sleep_sec > 0 {
            info!("AwaitSettlement heartbeat, {} seconds till auction", self.sec_to_auction());
            tokio::time::sleep(tokio::time::Duration::from_secs(sleep_sec as u64)).await;
            sleep_sec = self.sec_to_auction().min(heartbeat_sec);
        }
        loop {
            if self.is_settled().await? {
                return Ok(());
            }
            warn!("Option not yet settled past expiry, waiting");
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
    }
}

impl LRTCStage for LRTCAwaitSettlement {
    async fn run(&self) -> Result<()> {
        let wait_task = self.wait_for_auction();
        let asset_name = std::env::var("SPOT_NAME").unwrap();
        let deposit_task = process_deposits_forever(&self.tsa, asset_name);
        tokio::select! {
            w = wait_task => w,
            d = deposit_task => {
                error!("Deposit task unexpected early exit with {:#?}", d);
                Err(Error::msg("Deposit task unexpected early exit"))
            }
        }
    }
    async fn reconnect(&mut self) -> Result<()> {
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        self.tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum LRTCExecutorStage {
    SpotOnly(LRTCSpotOnly),
    OptionAuction(LimitOrderAuctionExecutor<OptionAuctionParams>),
    AwaitSettlement(LRTCAwaitSettlement),
    SpotAuction(LimitOrderAuctionExecutor<SpotAuctionParams>),
}

pub trait LRTCStage
where
    Self: Debug,
{
    async fn run(&self) -> Result<()>;
    async fn reconnect(&mut self) -> Result<()>;
    async fn reconnect_with_backoff(&mut self) -> Result<()> {
        let mut backoff = 4;
        let max_backoff = 64;
        loop {
            let res = self.reconnect().await;
            if res.is_ok() {
                return Ok(());
            }
            error!("{:#?} reconnect failed with {:#?}, reconnecting in {}", self, res, backoff);
            tokio::time::sleep(tokio::time::Duration::from_secs(backoff)).await;
            backoff = (backoff * 2).min(max_backoff);
        }
    }
    async fn run_with_reconnect(&mut self) -> Result<()> {
        loop {
            let res = self.run().await;
            if res.is_ok() {
                return Ok(());
            }
            error!("{:#?} run failed with {:#?}", self, res);
            self.reconnect_with_backoff().await?;
        }
    }
}

impl<S: OrderStrategy + Debug> LRTCStage for LimitOrderAuctionExecutor<S> {
    async fn run(&self) -> Result<()> {
        let remain_sec = self.auction.remain_sec();
        if remain_sec <= 0 {
            return Ok(());
        }

        let market_task = self.run_market();
        let auction_task = self.run_auction();
        let ping_task = self.auction.client.ping_interval(15);
        let res = select! {
            _ = market_task => {Err(Error::msg("Market task exited early"))},
            _ = ping_task => {Err(Error::msg("Ping task exited early"))},
            _ = tokio::time::sleep(tokio::time::Duration::from_secs(remain_sec as u64)) => {Ok(())},
            auction_res = auction_task => { auction_res },
        };
        res
    }
    async fn reconnect(&mut self) -> Result<()> {
        self.auction.market = new_market_state();
        self.auction.client = WsClient::new_client().await?;
        self.auction.client.login().await?;
        self.auction.client.enable_cancel_on_disconnect().await?;
        Ok(())
    }
}
