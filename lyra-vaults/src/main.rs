mod aws;
mod lrtc;
mod market;
mod setup;
mod shared;

use crate::setup::setup_env;
use anyhow::{Error, Result};
use log::{debug, error, info, warn};
use lrtc::params::LRTCParams;
use lrtc::selector::select_option;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{join, select, try_join};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    setup_env().await;
    let params = LRTCParams {
        subaccount_id: std::env::var("SUBACCOUNT_ID").unwrap().parse().unwrap(),
        currency: "ETH".to_string(),
        spot_name: "ETH-PERP".to_string(), // testing
        cash_name: "USDC".to_string(),
        expiry: lrtc::params::TargetExpiry::Weekly,
        delta: lrtc::params::TargetDelta::Ten,

        min_iv: 0.3,
        max_iv_spread: 0.2,
        init_iv_spread: -0.05,
        iv_spread_per_hour: 0.2,
        option_auction_sec: 60 * 60,
    };

    let option_executor = lrtc::option_auction::LRTCOptionExecutor::from_params(params).await?;
    let task_handle = tokio::spawn(async move { option_executor.run().await });
    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

    Ok(())
}
