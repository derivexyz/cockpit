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

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    setup_env().await;
    console_subscriber::init();
    let params = tokio::fs::read_to_string("./lyra-vaults/lrtc.json").await?;
    let params: LRTCParams = serde_json::from_str(&params)?;
    std::env::set_var("SUBACCOUNT_ID", params.subaccount_id.to_string());

    let mut executor = LRTCExecutor::new(params).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}
