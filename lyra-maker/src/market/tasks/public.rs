use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::market::core::{MarketState, OrderbookData};
use log::{debug, error, info, warn};
use lyra_client::auth::{load_signer, sign_auth_header};
use lyra_client::json_rpc::{http_rpc, Notification, Response, WsClient, WsClientExt};
use orderbook_types::generated::public_get_instruments::{
    InstrumentType, PublicGetInstrumentsParamsSchema, PublicGetInstrumentsResponseSchema,
};
use orderbook_types::types::tickers::result::{InstrumentTicker, TickerNotificationData};
use serde_json::{json, Value};
use tokio::select;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MarketSubscriberData {
    OrderbookMsg(Notification<OrderbookData>),
    TickerMsg(Notification<TickerNotificationData>),
}

pub async fn fetch_live_options(currency: String) -> Result<Vec<String>> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let headers = sign_auth_header(&load_signer().await).await;
    let pos_resp = http_rpc::<Value, Value>(
        "private/get_positions",
        json!({"subaccount_id": subaccount_id}),
        Some(headers),
    )
    .await?
    .into_result()?;
    // TODO filter out zero positions due to open orders margin....
    let positions = pos_resp["result"]["positions"].as_array().unwrap();
    let positions_set = positions
        .iter()
        .map(|p| p["instrument_name"].as_str().unwrap().to_string())
        .collect::<std::collections::HashSet<String>>();
    let mut instruments: Vec<String> = http_rpc::<_, PublicGetInstrumentsResponseSchema>(
        "public/get_instruments",
        PublicGetInstrumentsParamsSchema {
            currency,
            instrument_type: InstrumentType::Option,
            expired: false,
        },
        None,
    )
    .await?
    .into_result()?
    .result
    .into_iter()
    .filter(|r| r.is_active && positions_set.contains(&r.instrument_name))
    .map(|r| r.instrument_name)
    .collect();
    instruments.sort();
    info!("Fetched instruments: {:?}", instruments);
    Ok(instruments)
}

async fn fetch_until_different(currency: String, instruments: Vec<String>) -> Result<Vec<String>> {
    loop {
        let mut current_instruments = fetch_live_options(currency.clone()).await?;
        if current_instruments != instruments {
            return Ok(current_instruments);
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
}

pub async fn start_option_tickers(state: MarketState, currency: String) -> Result<()> {
    info!("Starting option tickers task");
    let mut instruments = fetch_live_options(currency.clone()).await?;
    loop {
        if instruments.is_empty() {
            warn!("No instruments found, retrying in 30s");
            tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
            instruments = fetch_live_options(currency.clone()).await?;
            continue;
        }
        let new_instruments = select! {
            instruments = fetch_until_different(currency.clone(), instruments.clone()) => {
                info!("Instruments changed, re-subscribing");
                instruments
            }
            _ = subscribe_tickers(state.clone(), instruments.clone()) => {
                Err(Error::msg("Ticker subscription returned early"))
            }
        };
        instruments = new_instruments?;
    }
}

pub async fn subscribe_tickers(state: MarketState, instrument_names: Vec<String>) -> Result<()> {
    let channels: Vec<String> = instrument_names
        .iter()
        .map(|instrument_name| format!("ticker.{}.100", instrument_name))
        .collect();
    let client = WsClient::new_client().await?;
    client
        .subscribe(channels, |d: MarketSubscriberData| async {
            match d {
                MarketSubscriberData::TickerMsg(msg) => {
                    state.write().await.insert_ticker(msg.params.data.instrument_ticker);
                }
                _ => {}
            }
            Ok(())
        })
        .await?;
    Ok(())
}

pub async fn start_market(state: MarketState, instrument_names: Vec<String>) -> Result<()> {
    info!("Starting market task");
    let channels: Vec<String> = instrument_names
        .iter()
        .flat_map(|instrument_name| {
            vec![
                format!("orderbook.{}.1.10", instrument_name),
                format!("ticker.{}.100", instrument_name),
            ]
        })
        .collect();

    let client = WsClient::new_client().await?;
    client
        .subscribe(channels, |d: MarketSubscriberData| async {
            match d {
                MarketSubscriberData::OrderbookMsg(msg) => {
                    state.write().await.insert_orderbook(msg.params.data);
                }
                MarketSubscriberData::TickerMsg(msg) => {
                    state.write().await.insert_ticker(msg.params.data.instrument_ticker);
                }
            }
            Ok(())
        })
        .await?;
    Ok(())
}
