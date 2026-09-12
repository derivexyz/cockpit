pub mod actions;
pub mod auth;
pub mod aws;
mod cli;
pub mod json_rpc;
pub mod setup;
pub mod utils;

use crate::cli::CliRpc;
use clap::Parser;
use log::{error, info};
use derive_rs::json_rpc::{http_rpc, WsClient, WsClientExt};
use derive_rs::setup::{ensure_owner, ensure_session_key, setup_env};

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    setup_env().await;
    ensure_session_key().await;
    ensure_owner().await;
    if let Err(e) = CliRpc::execute().await {
        eprintln!("CLI failed: {e:?}");
        error!("CLI failed: {e:?}");
        return Err(e);
    }
    Ok(())
}
