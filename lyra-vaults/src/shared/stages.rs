use crate::market::new_market_state;
use crate::shared::auction::{LimitOrderAuctionExecutor, OrderStrategy};
use crate::shared::rfq::{RFQAuctionExecutor, RFQStrategy};
use anyhow::Error;
use log::error;
use lyra_client::json_rpc::{WsClient, WsClientExt};
use std::fmt::Debug;
use tokio::select;

pub trait ExecutorStage
where
    Self: Debug,
{
    async fn run(&self) -> anyhow::Result<()>;
    async fn reconnect(&mut self) -> anyhow::Result<()>;
    async fn reconnect_with_backoff(&mut self) -> anyhow::Result<()> {
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
    async fn run_with_reconnect(&mut self) -> anyhow::Result<()> {
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

impl<S: OrderStrategy + Debug> ExecutorStage for LimitOrderAuctionExecutor<S> {
    async fn run(&self) -> anyhow::Result<()> {
        let market_task = self.run_market();
        let auction_task = self.run_auction();
        let ping_task = self.auction.client.ping_interval(15);
        let res = select! {
            _ = market_task => {Err(Error::msg("Market task exited early"))},
            _ = ping_task => {Err(Error::msg("Ping task exited early"))},
            auction_res = auction_task => { auction_res },
        };
        res
    }
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.auction.market = new_market_state();
        self.auction.client = WsClient::new_client().await?;
        self.auction.client.login().await?;
        self.auction.client.enable_cancel_on_disconnect().await?;
        Ok(())
    }
}

impl<S: RFQStrategy + Debug> ExecutorStage for RFQAuctionExecutor<S> {
    async fn run(&self) -> anyhow::Result<()> {
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
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        self.auction.market = new_market_state();
        self.auction.client = WsClient::new_client().await?;
        self.auction.client.login().await?;
        self.auction.client.enable_cancel_on_disconnect().await?;
        Ok(())
    }
}
