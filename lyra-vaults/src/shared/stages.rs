use crate::helpers::{get_option_expiry, sync_subaccount};
use crate::lrtc::params::LRTCParams;
use crate::lrtc::selector::maybe_select_from_positions;
use crate::market::new_market_state;
use crate::shared::auction::{LimitOrderAuctionExecutor, OrderStrategy};
use crate::shared::rfq::{RFQAuctionExecutor, RFQStrategy};
use crate::web3::{
    get_tsa_contract, process_deposits_forever, process_deposits_once, process_withdrawals,
    ProviderWithSigner, TSA,
};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, Zero};
use log::{error, info, warn};
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

/// - This stage will process withdrawals for the spot asset.
/// - Will keep reconnecting in case of any errors raised during process_withdrawals.
/// - Will stop when total pending withdrawals are 0.
/// - TODO Note that there are also some edge cases when some stray USDC was not fully sold off and
/// withdrawals are so large that the vault has not enough LRT balance to cover it.
#[derive(Debug)]
pub struct TSACollateralOnly {
    pub tsa: TSA<ProviderWithSigner>,
}

impl TSACollateralOnly {
    pub async fn new() -> Result<Self> {
        info!("Starting TSASpotOnly Stage");
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(Self { tsa })
    }
}

impl ExecutorStage for TSACollateralOnly {
    async fn run(&self) -> Result<()> {
        // todo might wanna rename the env to COLLATERAL_NAME for clarity
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

/// - This stage will wait for the options to be settled.
/// - Assumes all options have the same expiry.
#[derive(Debug)]
pub struct TSAWaitForSettlement {
    pub subaccount_id: i64,
    pub tsa: TSA<ProviderWithSigner>,
    pub option_names: Vec<String>,
    pub option_expiry: i64,
    pub delay_min: i64,
}

impl TSAWaitForSettlement {
    pub async fn new(delay_min: i64, option_names: Vec<String>) -> Result<Self> {
        let subaccount_id = std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap();
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        let tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        let option_expiry = get_option_expiry(&option_names[0]).await?;
        Ok(Self { subaccount_id, tsa, option_names, option_expiry, delay_min })
    }
    pub async fn is_settled(&self) -> Result<bool> {
        // todo some of these might be cleaner to just use get_subaccount over REST...
        let market = new_market_state();
        sync_subaccount(market.clone(), self.subaccount_id, vec![]).await?;
        let reader = market.read().await;
        let option_positions = reader
            .iter_positions()
            .filter(|&p| self.option_names.contains(&p.instrument_name))
            .collect::<Vec<_>>();
        let zero = BigDecimal::zero();
        let all_settled = option_positions.into_iter().all(|p| p.amount == zero);
        return Ok(all_settled);
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

impl ExecutorStage for TSAWaitForSettlement {
    async fn run(&self) -> Result<()> {
        let wait_task = self.wait_for_auction();
        let asset_name = std::env::var("SPOT_NAME").unwrap();
        let deposit_task = process_deposits_forever(&self.tsa, asset_name);
        select! {
            w = wait_task => w,
            d = deposit_task => {
                error!("Deposit task unexpected early exit with {:#?}", d);
                Err(Error::msg("Deposit task unexpected early exit"))
            }
        }
    }
    async fn reconnect(&mut self) -> anyhow::Result<()> {
        let vault_name = std::env::var("VAULT_NAME").unwrap();
        self.tsa = get_tsa_contract(&vault_name, "SESSION").await?;
        Ok(())
    }
}
