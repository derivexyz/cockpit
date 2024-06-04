use crate::lrtc::auction::LimitOrderAuctionExecutor;
use crate::lrtc::params::{OptionAuctionParams, SpotAuctionParams};
use crate::lrtc::selector::maybe_select_from_positions;
use crate::market::new_market_state;
use crate::shared::{fetch_ticker, sync_subaccount};
use anyhow::Result;
use bigdecimal::Zero;
use log::{error, warn};
use lyra_client::auth::get_auth_headers;
use lyra_client::json_rpc::http_rpc;
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::types::tickers::TickerResponse;
use serde_json::json;
use std::fmt::Debug;

#[derive(Debug)]
pub struct LRTCSpotOnly {}

impl LRTCStage for LRTCSpotOnly {
    async fn run(&self) -> Result<()> {
        // todo process withdrawals
        Ok(())
    }
    async fn reconnect(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRTCAwaitSettlement {
    pub subaccount_id: i64,
    pub option_name: String,
    pub option_expiry: i64,
}

impl LRTCAwaitSettlement {
    pub async fn new(option_name: String) -> Result<Self> {
        let subaccount_id = std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap();
        let market = new_market_state();
        fetch_ticker(market.clone(), &option_name).await?;
        let reader = market.read().await;
        let option_expiry =
            reader.get_ticker(&option_name).unwrap().option_details.as_ref().unwrap().expiry;
        Ok(Self { subaccount_id, option_name, option_expiry })
    }
    pub async fn is_settled(&self) -> Result<bool> {
        // todo some of these might be cleaner to use REST for
        let market = new_market_state();
        sync_subaccount(market.clone(), self.subaccount_id, vec![]).await?;
        let option_name = maybe_select_from_positions(&market).await?;
        return Ok(option_name.is_none());
    }
}

impl LRTCStage for LRTCAwaitSettlement {
    async fn run(&self) -> Result<()> {
        let sec_to_expiry = self.option_expiry - chrono::Utc::now().timestamp();
        tokio::time::sleep(tokio::time::Duration::from_secs(sec_to_expiry as u64)).await;
        loop {
            if self.is_settled().await? {
                return Ok(());
            }
            warn!("Option not yet settled past expiry, waiting");
            tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        }
    }
    async fn reconnect(&mut self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRTCSpotAuction {}

impl LRTCStage for LRTCSpotAuction {
    async fn run(&self) -> Result<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> Result<()> {
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
        let mut backoff = 1;
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
