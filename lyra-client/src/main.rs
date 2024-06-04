pub mod auth;
pub mod aws;
mod cli;
pub mod json_rpc;
pub mod orders;
pub mod setup;
pub mod utils;

use crate::cli::CliRpc;
use clap::Parser;
use log::{error, info};
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use lyra_client::setup::setup_env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    setup_env().await;
    let _ = CliRpc::execute().await;
    Ok(())
}
