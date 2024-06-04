mod algos;
mod market;

use algos::GammaDDHAlgo;
use algos::MakerAlgo;
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use env_logger::Env;
use ethers::prelude::{LocalWallet, Signer};
use ethers::types::spoof::state;
use futures::future::join_all;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use lyra_client::auth::{load_signer, sign_auth_header, sign_auth_msg};
use lyra_client::json_rpc::{http_rpc, Response, WsClient, WsClientExt};
use lyra_client::orders::{get_order_signature, OrderArgs};
use lyra_client::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, TimeInForce,
};
use market::tasks::private::start_subaccount;
use market::tasks::public::{start_market, start_option_tickers};
use reqwest::{header::HeaderMap, header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::mpsc;
use tokio::{join, select, try_join};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use ethers::utils::hex;
use market::core::{new_market_state, MarketState};
use orderbook_types::generated::channel_ticker_instrument_name_interval::InstrumentTickerSchema;
use tokio_tungstenite::tungstenite::client;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use orderbook_types::generated::private_get_collaterals::{
    PrivateGetCollateralsParamsSchema, PrivateGetCollateralsResponseSchema,
};
use orderbook_types::generated::private_get_orders::{
    PrivateGetOrdersParamsSchema, PrivateGetOrdersResponseSchema,
};
use orderbook_types::generated::private_get_positions::{
    PrivateGetPositionsParamsSchema, PrivateGetPositionsResponseSchema,
};
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::generated::private_order_debug::{
    PrivateOrderDebugParamsSchema, PrivateOrderDebugResultSchema,
};
use orderbook_types::generated::public_get_ticker::{
    PublicGetTickerParamsSchema, PublicGetTickerResultSchema,
};
use orderbook_types::generated::public_login::{
    PublicLoginParamsSchema, PublicLoginResponseSchema,
};

use crate::market::tasks::public::MarketSubscriberData;
use lyra_client::setup::setup_env;

pub async fn setup_ip_whitelist() -> Result<()> {
    let client = WsClient::new_client().await?;
    client.login().await?.into_result()?;
    let res = client
        .send_rpc::<Value, Value>(
            "private/edit_session_key",
            json!({
                "public_session_key": client.get_signer().await,
                "wallet": client.get_owner().await,
                "ip_whitelist": []
            }),
        )
        .await?
        .into_result()?;
    info!("Edit session key: {:?}", res);
    Ok(())
}

async fn run_ddh() -> Result<()> {
    // read subaccount id from os args provided to the run script
    let subaccount_id: i64 =
        std::env::args().nth(1).unwrap_or(std::env::var("SUBACCOUNT_ID")?).parse()?;
    // set SUBACCOUNT_ID env variable
    std::env::set_var("SUBACCOUNT_ID", subaccount_id.to_string());

    let currency = "ETH".to_string();
    let perps = vec![format!("{currency}-PERP")];
    let token = CancellationToken::new();
    let tracker = TaskTracker::new();
    let market_ptr = new_market_state();

    let perp_token = token.clone();
    let perp_market_ptr = market_ptr.clone();
    let perp_task = tracker.spawn(async move {
        select! {
        res = start_market(perp_market_ptr, perps) => res,
        _ = perp_token.cancelled() => Ok(())}
    });

    let option_token = token.clone();
    let option_market_ptr = market_ptr.clone();
    let option_task = tracker.spawn(async move {
        select! {
        res = start_option_tickers(option_market_ptr, currency.clone()) => res,
        _ = option_token.cancelled() => Ok(())}
    });

    let subacc_token = token.clone();
    let subacc_market_ptr = market_ptr.clone();
    let subacc_task = tracker.spawn(async move {
        select! {
        res = start_subaccount(subacc_market_ptr, subaccount_id) => res,
        _ = subacc_token.cancelled() => Ok(())}
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let algo = GammaDDHAlgo {
        subaccount_id,
        perp_name: "ETH-PERP".to_string(),
        max_abs_delta: BigDecimal::from_str("0.3")?,
        max_abs_spread: BigDecimal::from_str("40")?,
        action_wait_ms: 3000,
        price_tol: BigDecimal::from_str("1.5")?,
        amount_tol: BigDecimal::from_str("0.02")?,
        mid_price_tol: BigDecimal::from_str("20")?,
    };

    let algo_token = token.clone();
    let algo_task = tokio::spawn(async move {
        let hedger_task = algo.start_hedger(market_ptr.clone());
        // let maker_task = algo.start_maker(market_ptr.clone());
        select! {
            // _ = maker_task => (),
            res = hedger_task => res,
            _ = algo_token.cancelled() => Ok(())
        }
    });

    tracker.close();
    let exit_result = select! {
        res = perp_task => res,
        res = option_task => res,
        res = subacc_task => res,
        res = algo_task => res,
    };
    token.cancel();
    tracker.wait().await;

    match exit_result {
        Err(e) => Err(Error::new(e)),
        Ok(res) => res,
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {
    setup_env().await;
    loop {
        let res = run_ddh().await;
        error!("Unexpected exit in run_ddh(): {:?}", res);
        warn!("Restarting in 10s");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
