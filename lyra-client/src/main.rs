pub mod auth;
pub mod aws;
mod cli;
pub mod json_rpc;
pub mod orders;
pub mod setup;
pub mod utils;

use crate::cli::CliRpc;
use clap::Parser;
use log::info;
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use lyra_client::orders::OrderArgs;
use lyra_client::setup::setup_env;
use orderbook_types::types::tickers::TickerResponse;
use serde_json::{json, Value};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    setup_env().await;
    let resp = CliRpc::call().await;
    match resp {
        Ok(r) => info!("{}", serde_json::to_string_pretty(&r)?),
        Err(e) => info!("Error: {:?}", e),
    };
    Ok(())
}
