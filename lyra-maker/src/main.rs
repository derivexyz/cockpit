mod market;
mod algos;

use log::{info, warn, error, debug};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::sync::{Arc, Mutex};
use env_logger::Env;
use bigdecimal::BigDecimal;
use lyra_client::orders::{OrderTicker, OrderArgs, get_order_signature};
use lyra_client::orders::{Direction, LiquidityRole, OrderStatus, OrderType, OrderParams, OrderResponse, TimeInForce};
use anyhow::{Result, Error};
use lyra_client::auth::{load_signer, sign_auth_header, sign_auth_msg};
use futures_util::{SinkExt, StreamExt};
use lyra_client::json_rpc::{http_rpc, Response, WsClient, WsClientExt};
use market::tasks::public::start_market;
use market::tasks::private::start_subaccount;
use algos::MakerAlgo;
use reqwest::{Client, header::HeaderMap, header::HeaderValue};
use serde_json::json;
use std::str::FromStr;
use std::time::Instant;
use ethers::prelude::{LocalWallet, Signer};
use ethers::types::spoof::state;
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::join;
use futures::future::join_all;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use ethers::utils::hex;
use tokio_tungstenite::tungstenite::client;
use market::core::{MarketState, new_market_state};
use orderbook_types::generated::channel_ticker_instrument_name_interval::InstrumentTickerSchema;

use orderbook_types::generated::public_get_ticker::{
    PublicGetTickerParamsSchema, PublicGetTickerResultSchema,
};
use orderbook_types::generated::private_order_debug::{PrivateOrderDebugParamsSchema, PrivateOrderDebugResultSchema};
use orderbook_types::generated::public_login::{PublicLoginParamsSchema, PublicLoginResponseSchema};
use orderbook_types::generated::private_get_subaccount::{PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema};
use orderbook_types::generated::private_get_positions::{PrivateGetPositionsParamsSchema, PrivateGetPositionsResponseSchema};
use orderbook_types::generated::private_get_orders::{PrivateGetOrdersParamsSchema, PrivateGetOrdersResponseSchema};
use orderbook_types::generated::private_get_collaterals::{PrivateGetCollateralsParamsSchema, PrivateGetCollateralsResponseSchema};

use crate::market::tasks::public::MarketSubscriberData;

async fn printer_task(state: MarketState) -> Result<()> {
    loop {
        let data = state.read().await;
        for ob in data.iter_orderbooks() {
            info!("Orderbook: {}", ob.instrument_name);
            info!("Full orderbook: {}", serde_json::to_string_pretty(&data.get_orderbook(&ob.instrument_name).unwrap())?);
            info!("Orderbook w/o my orders: {}", serde_json::to_string_pretty(&data.get_orderbook_exclude_my_orders(&ob.instrument_name).unwrap())?);
        }
        for ticker in data.iter_tickers() {
            info!("Ticker: {}", ticker.instrument_name);
        }
        for position in data.iter_positions() {
            info!("Position: {}, {}", position.instrument_name, position.amount);
        }
        for orders in data.iter_orders() {
            for (id, order) in orders.iter() {
                info!("Order: {}, {}, {}, {}", order.instrument_name, id, order.amount, order.filled_amount);
            }
        }
        drop(data);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<()> {

    println!("Starting maker bot from {}", std::env::current_dir().unwrap().display());
    let args: Vec<String> = std::env::args().collect();
    let env_name = args.get(1);
    if let Some(env_name) = env_name {
        println!("Reading .env.{}", env_name);
        dotenv::from_filename(format!(".env.{env_name}")).expect("Failed to load .env file");
    } else {
        dotenv::from_filename(".env.local").expect("Failed to load .env file");
    }
    env_logger::builder().format_timestamp_millis().init();

    let subaccount_id: i64 = 6581;
    let state_ptr = new_market_state();
    let eth_task = tokio::spawn(start_market(
        state_ptr.clone(),
        vec!["ETH-PERP".to_string()],
    ));
    let subacc_task = tokio::spawn(start_subaccount(state_ptr.clone(), subaccount_id));
    // tokio::spawn(printer_task(state_ptr.clone()));
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    let client = WsClient::new_client().await?;
    client.login().await?;

    let algo = MakerAlgo {
        subaccount_id,
        perp_name: "ETH-PERP".to_string(),
        spot_name: "ETH-PERP".to_string(),
        hedge_spread: BigDecimal::from_str("0.00025")?,
        size: BigDecimal::from_str("0.5")?,
        min_index_spread: BigDecimal::from_str("0.00025")?,
        max_index_spread: BigDecimal::from_str("0.02")?,
        target_delta: BigDecimal::from_str("0")?,
        twap_ms: 10000,
    };
    let algo_task = tokio::spawn(async move {
        let _ = algo.start_maker(state_ptr.clone()).await;
    });

    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    Ok(())
}
