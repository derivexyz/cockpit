use crate::lrtc::auction::LimitOrderAuctionExecutor;
use crate::lrtc::params::{OptionAuctionParams, SpotAuctionParams};
use log::{error, warn};
use std::fmt::Debug;

#[derive(Debug)]
pub struct LRTCSpotOnly {}

impl LRTCStage for LRTCSpotOnly {
    async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRTCAwaitSettlement {}

impl LRTCStage for LRTCAwaitSettlement {
    async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct LRTCSpotAuction {}

impl LRTCStage for LRTCSpotAuction {
    async fn run(&self) -> anyhow::Result<()> {
        Ok(())
    }
    async fn reconnect(&mut self) -> anyhow::Result<()> {
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
    async fn run(&self) -> anyhow::Result<()>;
    async fn reconnect(&mut self) -> anyhow::Result<()>;
    async fn reconnect_with_backoff(&mut self) -> anyhow::Result<()> {
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
