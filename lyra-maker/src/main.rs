mod algos;
mod aws;
mod market;
mod setup;

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
use lyra_client::orders::{get_order_signature, OrderArgs, OrderTicker};
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
use crate::setup::setup_env;

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
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let currency = "ETH".to_string();
    let market_ptr = new_market_state();
    let perp_task =
        tokio::spawn(start_market(market_ptr.clone(), vec![format!("{currency}-PERP")]));
    let option_task = tokio::spawn(start_option_tickers(market_ptr.clone(), currency.clone()));
    let subacc_task = tokio::spawn(start_subaccount(market_ptr.clone(), subaccount_id));
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let algo = GammaDDHAlgo {
        subaccount_id,
        perp_name: "ETH-PERP".to_string(),
        max_abs_delta: BigDecimal::from_str("0.15")?,
        action_wait_ms: 2000,
        price_tol: BigDecimal::from_str("1.5")?,
        amount_tol: BigDecimal::from_str("0.02")?,
        mid_price_tol: BigDecimal::from_str("20")?,
    };

    let token = CancellationToken::new();
    let algo_cancel_token = token.clone();

    let algo_task = tokio::spawn(async move {
        let hedger_task = algo.start_hedger(market_ptr.clone());
        // let hedger_task = algo.start_maker(market_ptr.clone());
        select! {
            // _ = maker_task => (),
            res = hedger_task => res,
            _ = algo_cancel_token.cancelled() => Ok(())
        }
    });

    let exit_result = select! {
        res = perp_task => res,
        res = option_task => res,
        res = subacc_task => res,
        res = algo_task => res,
    };
    token.cancel();

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
        error!("Error in run_ddh(): {:?}", res);
        warn!("Restarting in 10s");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
