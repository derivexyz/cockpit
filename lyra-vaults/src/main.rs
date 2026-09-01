extern crate core;

mod clickhouse;
mod helpers;
mod longpp;
mod lrtc;
mod market;
mod shared;
mod signals;
mod web3;

use crate::clickhouse::ClickhouseClient;
use crate::longpp::executor::LongPPExecutor;
use crate::longpp::params::LongPPParams;
use crate::longpp::selector::select_new_spread;
use crate::lrtc::executor::LRTCExecutor;
use crate::signals::mock_vault::MockCCVaultParams;
use crate::signals::strategies::swather::{SwatherParams, SwatherVault};
use crate::signals::strategies::weathervane::{WeathervaneParams, WeathervaneVault};
use crate::signals::vault::SignalVaultExecutor;
use crate::web3::scripts::test_initiate_deposit;
use crate::web3::yields::get_price_at_timestamp;
use crate::web3::{actions, events, get_subaccount_id, maybe_tsa_address};
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use log::{debug, error, info, warn};
use lrtc::params::{LRTCParams, OptionAuctionParams};
use lyra_client::setup::{ensure_owner, ensure_session_key, setup_env};
use orderbook_types::types::rfqs::{Direction, LegUnpriced};
use serde::{Deserialize, Serialize};
use shared::params::SpotAuctionParams;
use shared::stages::ExecutorStage;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{join, select, try_join};
use web3::scripts;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum VaultParams {
    LRTC(LRTCParams),
    LongPP(LongPPParams),
    MockCC(MockCCVaultParams),
    Swather(SwatherVaultParams),
    Weathervane(WeathervaneVaultParams),
    // Add more vaults here
}

/// Sets up the owner of the vault's subaccount:
/// - vaults backed by a TSA contract are owned (and signed for) by that contract
/// - vaults without a TSA are owned by the session key's owner wallet, and their actions are
///   signed by the session key itself via the regular API flow
async fn setup_owner(vault_name: &str) {
    match maybe_tsa_address(vault_name) {
        Some(tsa_address) => {
            info!("Vault {} owner is the TSA at {}", vault_name, tsa_address);
            std::env::set_var("OWNER_PUBLIC_KEY", tsa_address);
        }
        None => {
            info!("No TSA address for {}, using the session key owner", vault_name);
            if std::env::var("OWNER_KEY_NAME").is_err() {
                std::env::set_var("OWNER_KEY_NAME", vault_name.to_lowercase());
            }
            ensure_owner().await;
        }
    }
}

async fn run_lrtc(params: LRTCParams) -> Result<()> {
    let vault_name = params.vault_name.clone();
    std::env::set_var("ENV", params.env.clone());
    std::env::set_var("SESSION_KEY_NAME", vault_name.to_lowercase());
    println!("Setting up {} env for LRTC executor", params.env.clone());
    setup_env().await;
    ensure_session_key().await;
    setup_owner(&vault_name).await;
    info!("LRTC executor params: {:?}", params);

    let subacc_id = get_subaccount_id(&vault_name).await?;
    info!("Vault Subaccount ID: {}", subacc_id);
    std::env::set_var("SUBACCOUNT_ID", subacc_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());
    std::env::set_var("SPOT_NAME", params.option_auction_params.spot_name.clone());
    std::env::set_var("CASH_NAME", params.spot_auction_params.cash_name.clone());

    info!("Starting LRTC executor");
    let mut executor = LRTCExecutor::new(params).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}

async fn run_long_pp(params: LongPPParams) -> Result<()> {
    let vault_name = params.vault_name.clone();
    std::env::set_var("ENV", params.env.clone());
    std::env::set_var("SESSION_KEY_NAME", vault_name.to_lowercase());
    println!("Setting up {} env for Long PP executor", params.env.clone());
    setup_env().await;
    ensure_session_key().await;
    setup_owner(&vault_name).await;
    info!("Long PP executor params: {:?}", params);

    let subacc_id = get_subaccount_id(&vault_name).await?;
    info!("Vault Subaccount ID: {}", subacc_id);
    std::env::set_var("SUBACCOUNT_ID", subacc_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());
    std::env::set_var("SPOT_NAME", params.option_auction_params.collat_name.clone());
    std::env::set_var("CASH_NAME", params.spot_auction_params.cash_name.clone());

    info!("Starting LongPP executor");
    let mut executor = LongPPExecutor::new(params).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}

/// A signal-driven vault with no TSA: the subaccount is owned by the session key's owner wallet
/// and the subaccount id comes from the params rather than from a contract.
#[derive(Debug, Clone, Deserialize)]
struct SwatherVaultParams {
    env: String,        // Environment name (e.g. staging, prod)
    vault_name: String, // used for logging and as the default key name
    subaccount_id: i64,
    /// Name of the session key and its owner in the AWS parameter store, defaulting to the
    /// lowercased vault name.
    key_name: Option<String>,
    /// Decision cadence in seconds. The signal is calibrated on hourly bars, so 3600.
    decision_interval_sec: Option<u64>,

    strategy_params: SwatherParams,
}

impl SwatherVaultParams {
    fn key_name(&self) -> String {
        self.key_name.clone().unwrap_or(self.vault_name.to_lowercase())
    }
    fn decision_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.decision_interval_sec.unwrap_or(3600))
    }
}

/// A signal-driven vault with no TSA, same shape as [SwatherVaultParams].
#[derive(Debug, Clone, Deserialize)]
struct WeathervaneVaultParams {
    env: String,
    vault_name: String,
    subaccount_id: i64,
    key_name: Option<String>,
    decision_interval_sec: Option<u64>,

    strategy_params: WeathervaneParams,
}

impl WeathervaneVaultParams {
    fn key_name(&self) -> String {
        self.key_name.clone().unwrap_or(self.vault_name.to_lowercase())
    }
    fn decision_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.decision_interval_sec.unwrap_or(3600))
    }
}

/// Runs the Weathervane vault: weekly 5-delta puts in an uptrend, calls in a downtrend.
async fn run_weathervane(params: WeathervaneVaultParams) -> Result<()> {
    let vault_name = params.vault_name.clone();
    let key_name = params.key_name();
    std::env::set_var("ENV", params.env.clone());
    std::env::set_var("SESSION_KEY_NAME", key_name.clone());
    std::env::set_var("OWNER_KEY_NAME", key_name);
    println!("Setting up {} env for the Weathervane executor", params.env.clone());
    setup_env().await;
    ensure_session_key().await;
    setup_owner(&vault_name).await;
    ClickhouseClient::ensure_keys().await;
    info!("Weathervane executor params: {:?}", params);

    info!("Vault Subaccount ID: {}", params.subaccount_id);
    std::env::set_var("SUBACCOUNT_ID", params.subaccount_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());

    let vault = WeathervaneVault {
        params: params.strategy_params.clone(),
        clickhouse: std::sync::Arc::new(ClickhouseClient::from_env()?),
    };
    info!("Starting Weathervane executor");
    let executor =
        SignalVaultExecutor::new(vault, params.subaccount_id, params.decision_interval()).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}

/// Runs the Swather vault: weekly short 10-delta calls gated on the live ClickHouse signal.
async fn run_swather(params: SwatherVaultParams) -> Result<()> {
    let vault_name = params.vault_name.clone();
    let key_name = params.key_name();
    std::env::set_var("ENV", params.env.clone());
    std::env::set_var("SESSION_KEY_NAME", key_name.clone());
    std::env::set_var("OWNER_KEY_NAME", key_name);
    println!("Setting up {} env for the Swather executor", params.env.clone());
    setup_env().await;
    ensure_session_key().await;
    setup_owner(&vault_name).await;
    ClickhouseClient::ensure_keys().await;
    info!("Swather executor params: {:?}", params);

    info!("Vault Subaccount ID: {}", params.subaccount_id);
    std::env::set_var("SUBACCOUNT_ID", params.subaccount_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());

    let vault = SwatherVault {
        params: params.strategy_params.clone(),
        clickhouse: std::sync::Arc::new(ClickhouseClient::from_env()?),
    };
    info!("Starting Swather executor");
    let executor =
        SignalVaultExecutor::new(vault, params.subaccount_id, params.decision_interval()).await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}

/// Runs the mock signal-driven covered call vault. Unlike the LRTC / LongPP vaults, this one is
/// not backed by a TSA: the subaccount is owned by the session key's owner wallet, there are no
/// deposits / withdrawals to process, and the subaccount id comes from the params file.
async fn run_mock_cc(params: MockCCVaultParams) -> Result<()> {
    let vault_name = params.vault_name.clone();
    let key_name = params.key_name();
    std::env::set_var("ENV", params.env.clone());
    std::env::set_var("SESSION_KEY_NAME", key_name.clone());
    std::env::set_var("OWNER_KEY_NAME", key_name);
    println!("Setting up {} env for the mock CC executor", params.env.clone());
    setup_env().await;
    ensure_session_key().await;
    setup_owner(&vault_name).await;
    info!("Mock CC executor params: {:?}", params);

    info!("Vault Subaccount ID: {}", params.subaccount_id);
    std::env::set_var("SUBACCOUNT_ID", params.subaccount_id.to_string());
    std::env::set_var("VAULT_NAME", vault_name.clone());

    info!("Starting mock CC executor");
    let executor = SignalVaultExecutor::new(
        params.strategy_params.clone(),
        params.subaccount_id,
        params.decision_interval(),
    )
    .await?;
    let task_handle = tokio::spawn(async move { executor.run().await });
    let res = task_handle.await?;
    if let Err(e) = res {
        error!("Executor failed: {:?}", e);
    }
    Ok(())
}

async fn run_mock_pp(params: LongPPParams) -> Result<()> {
    std::env::set_var("ENV", params.env.clone());
    setup_env().await;
    ensure_session_key().await;

    let subacc_id = 6581;
    info!("Vault Subaccount ID: {}", subacc_id);
    std::env::set_var("SUBACCOUNT_ID", subacc_id.to_string());
    std::env::set_var("VAULT_NAME", "RSWETH");
    let legs = select_new_spread(&params).await?;
    info!("Selected legs: {:?}", legs);
    let p = params.option_auction_params;
    let exec_p = p.clone();
    let now = chrono::Utc::now().timestamp();
    let auction =
        shared::rfq::RFQAuction::new(legs, now, p.lot_init_sleep_sec, p.auction_sec).await?;
    let mut exec = shared::rfq::RFQAuctionExecutor { auction, strategy: exec_p };
    exec.run_with_reconnect().await?;
    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    println!("Reading params from json file");
    // read json name from cmd input
    let args: Vec<String> = std::env::args().collect();
    let json_name = args.get(1).ok_or(Error::msg("No json name provided"))?;
    let params = tokio::fs::read_to_string(format!("./params/{json_name}.json")).await?;
    let params: VaultParams = serde_json::from_str(&params)?;
    match params {
        VaultParams::LRTC(params) => run_lrtc(params).await?,
        VaultParams::LongPP(params) => run_long_pp(params).await?,
        VaultParams::MockCC(params) => run_mock_cc(params).await?,
        VaultParams::Swather(params) => run_swather(params).await?,
        VaultParams::Weathervane(params) => run_weathervane(params).await?,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn read_params(json_name: &str) -> VaultParams {
        let path = format!("{}/../params/{}.json", env!("CARGO_MANIFEST_DIR"), json_name);
        let params = std::fs::read_to_string(path).unwrap();
        serde_json::from_str(&params).unwrap()
    }

    /// The params enum is untagged, so each file must still route to its own vault.
    #[test]
    fn params_files_route_to_their_vaults() {
        assert!(matches!(read_params("weeth_lrtc_prod"), VaultParams::LRTC(_)));
        assert!(matches!(read_params("weeth_pp_prod"), VaultParams::LongPP(_)));
        assert!(matches!(read_params("mock_cc_staging"), VaultParams::MockCC(_)));
        assert!(matches!(read_params("swather_staging"), VaultParams::Swather(_)));
        assert!(matches!(read_params("weathervane_staging"), VaultParams::Weathervane(_)));
        assert!(matches!(read_params("weathervane_prod"), VaultParams::Weathervane(_)));
    }

    /// The gate constants and the tenor band are what make this Swather rather than some other
    /// covered call, so pin them to the spec's parameter table (§8).
    #[test]
    fn swather_params_match_the_spec() {
        let VaultParams::Swather(params) = read_params("swather_staging") else {
            panic!("swather_staging did not parse as a Swather vault");
        };
        let p = &params.strategy_params;
        assert_eq!(params.key_name(), "mm-acc");
        assert_eq!(params.decision_interval(), std::time::Duration::from_secs(3600));
        // the signal must be read on the same asset the book trades, else the gate is unrelated
        assert_eq!(p.gate.option_currency, p.option_currency);
        assert_eq!(p.leg.target_delta, BigDecimal::from_str("0.1").unwrap());
        // sized off the held collateral, hard-capped while the book is being proved out
        assert_eq!(p.leg.spot_currency, "LBTC");
        assert_eq!(p.leg.notional_ratio, BigDecimal::from_str("1.0").unwrap());
        assert_eq!(p.leg.max_contracts, BigDecimal::from_str("0.5").unwrap());
        assert_eq!(p.leg.max_delta_dev, BigDecimal::from_str("0.1").unwrap());
        assert_eq!(p.leg.max_mark_dev, BigDecimal::from_str("0.25").unwrap());
        assert_eq!(p.gate.iv_rank_floor, 40.0);
        assert_eq!(p.gate.mom_max_pct, 5.0);
        assert_eq!(p.gate.minhold_hours, 72);
        assert_eq!(p.min_hold_days, 3.0);
        assert_eq!(p.no_close_below_dte, 1.0);
        // the 4-10 DTE band of §1
        assert_eq!(p.leg.tenor_band_sec().unwrap(), (4 * 86400, 10 * 86400));
        // an auction must fit inside a decision, so a stale one cannot block the next signal
        assert!(p.auction_cfg.auction_sec < params.decision_interval().as_secs() as i64);
    }

    #[test]
    fn mock_cc_params_are_eth_weekly_ten_delta() {
        let VaultParams::MockCC(params) = read_params("mock_cc_staging") else {
            panic!("mock_cc_staging did not parse as a mock CC vault");
        };
        let strategy = &params.strategy_params;
        assert_eq!(params.vault_name, "MOCK_CC");
        // both /session_keys/staging/mm-acc and /owners/staging/mm-acc
        assert_eq!(params.key_name(), "mm-acc");
        assert_eq!(params.decision_interval(), std::time::Duration::from_secs(3600));
        assert_eq!(strategy.option_currency, "ETH");
        assert_eq!(strategy.target_delta, BigDecimal::from_str("0.1").unwrap());
        assert_eq!(strategy.target_notional, BigDecimal::from_str("10000").unwrap());
        // the window must span a full week so that the latest expiry in it is the next weekly
        assert!(strategy.expiry_sec().unwrap() > 7 * 24 * 60 * 60);
        assert!(strategy.min_expiry_sec().unwrap() > 0);
    }
}
