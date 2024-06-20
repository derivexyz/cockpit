extern crate core;

mod lrtc;
mod market;
mod shared;
mod web3;

use crate::lrtc::executor::LRTCExecutor;
use crate::lrtc::stages::LRTCStage;
use crate::web3::{actions, events};
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use log::{debug, error, info, warn};
use lrtc::params::{LRTCParams, OptionAuctionParams, SpotAuctionParams};
use lyra_client::setup::setup_env;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{join, select, try_join};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    info!("Setting up env for LRTC executor");
    setup_env().await;
    // read json name from cmd input
    info!("Reading params from json file");
    let args: Vec<String> = std::env::args().collect();
    let json_name = args.get(1).ok_or(Error::msg("No json name provided"))?;
    let params = tokio::fs::read_to_string(format!("./params/{json_name}.json")).await?;
    let params: LRTCParams = serde_json::from_str(&params)?;

    let vault_name = params.vault_name.clone();
    std::env::set_var("SUBACCOUNT_ID", params.subaccount_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());
    std::env::set_var("SPOT_NAME", params.spot_name.clone());
    std::env::set_var("CASH_NAME", params.cash_name.clone());

    let tsa_address: String = std::env::var(format!("{vault_name}_TSA_ADDRESS")).unwrap();
    std::env::set_var("OWNER_PUBLIC_KEY", tsa_address);

    info!("Starting LRTC executor");
    let mut executor = LRTCExecutor::new(params).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }

    Ok(())
}
