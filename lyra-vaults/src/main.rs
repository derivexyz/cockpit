extern crate core;

mod lrtc;
mod market;
mod shared;

use crate::lrtc::executor::LRTCExecutor;
use crate::lrtc::stages::LRTCStage;
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use log::{debug, error, info, warn};
use lrtc::params::{LRTCParams, OptionAuctionParams, SpotAuctionParams};
use lyra_client::setup::setup_env;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{join, select, try_join};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    setup_env().await;
    console_subscriber::init();
    let params = LRTCParams {
        subaccount_id: std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap(),
        currency: "ETH".to_string(),
        spot_name: "ETH".to_string(), // todo spot instrument might have ETH-SPOT name...
        cash_name: "USDC".to_string(),
        expiry: lrtc::params::TargetExpiry::Weekly,
        target_delta: BigDecimal::from_str("0.1").unwrap(),
        max_delta: BigDecimal::from_str("0.4").unwrap(),

        option_auction_params: OptionAuctionParams {
            max_iv_spread: 0.2,
            init_iv_spread: -0.05,
            iv_spread_per_min: 0.02,
            auction_sec: 60 * 60,
            price_change_tolerance: BigDecimal::from_str("0.2").unwrap(),

            spot_name: "ETH".to_string(),
        },

        spot_auction_params: SpotAuctionParams {
            max_spot_spread: 0.01,
            init_spot_spread: -0.001,
            spot_spread_per_min: 0.001,
            auction_sec: 60 * 15,
            price_change_tolerance: BigDecimal::from_str("1").unwrap(),
            cash_name: "USDC".to_string(),
            max_cash: BigDecimal::from_str("500").unwrap(),
        },
    };
    let mut executor = LRTCExecutor::new(params).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }

    Ok(())
}
