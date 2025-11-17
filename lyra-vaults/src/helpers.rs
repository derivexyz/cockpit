use crate::market::{new_market_state, Balance, MarketState};
use lyra_client::auth::get_auth_headers;
use lyra_client::json_rpc::{http_rpc, Notification, Response, WsClient, WsClientExt};
use std::str::FromStr;

use orderbook_types::generated::channel_subaccount_id_balances::BalanceUpdateSchema;
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::generated::public_get_spot_feed_history::{
    PublicGetSpotFeedHistoryParamsSchema, PublicGetSpotFeedHistoryResponseSchema,
    SpotFeedHistoryResponseSchema,
};
use orderbook_types::types::tickers::result::{
    InstrumentsResponse, OptionType, TickerNotificationData, TickerResponse,
};

use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, Zero};
use chrono::Utc;
use log::{debug, error, info, warn};
use orderbook_types::types::orders::{
    GetTradesParams, GetTradesResponse, OrderNotificationData, OrderResponse,
    TradeNotificationData, TxStatus,
};
use orderbook_types::types::tickers::{
    InstrumentData, InstrumentResponse, TickerSlimNotificationData,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::select;

type TickerSlimMsg = Notification<TickerSlimNotificationData>;

#[derive(Copy, Clone)]
pub enum TickerInterval {
    _1000Ms = 1000,
    _100Ms = 100,
}

pub async fn subscribe_tickers(
    market: MarketState,
    instruments: Vec<InstrumentData>,
    interval: TickerInterval,
) -> Result<()> {
    let instrument_names =
        instruments.iter().map(|inst| inst.instrument_name.clone()).collect::<Vec<String>>();
    let channels: Vec<String> = instrument_names
        .iter()
        .map(|instrument_name| format!("ticker_slim.{}.{}", instrument_name, interval as u32))
        .collect();
    let mut writer = market.write().await;
    writer.insert_instruments(instruments);
    drop(writer);
    let client = WsClient::new_client().await?;
    info!("Subscribing to tickers: {:?}", channels);
    client
        .subscribe(channels, |msg: TickerSlimMsg| async {
            let ticker = msg.params.data.instrument_ticker;
            let channel = msg.params.channel;
            let name = channel.split('.').nth(1).expect("Invalid channel format").to_string();
            market.write().await.insert_ticker_slim(ticker, name);
            Ok(())
        })
        .await?;
    Ok(())
}

pub async fn sync_subaccount(
    market: MarketState,
    subaccount_id: i64,
    instrument_names: Vec<String>,
) -> Result<()> {
    let headers = get_auth_headers().await;
    let subacc = http_rpc::<_, PrivateGetSubaccountResponseSchema>(
        "private/get_subaccount",
        PrivateGetSubaccountParamsSchema { subaccount_id },
        Some(headers.clone()),
    )
    .await?;
    info!("Subaccount state fetched");
    let mut writer = market.write().await;
    match subacc {
        Response::Error(e) => {
            error!("Failed to get subaccount with {:?}", e);
            return Err(Error::msg("Failed to get subaccount"));
        }
        Response::Success(subacc) => {
            let now = Utc::now().timestamp_millis();
            for position in subacc.result.positions {
                writer.insert_position(Balance {
                    instrument_name: position.instrument_name,
                    amount: position.amount,
                    timestamp: now,
                });
            }
            for collateral in subacc.result.collaterals {
                writer.insert_position(Balance {
                    instrument_name: collateral.asset_name,
                    amount: collateral.amount,
                    timestamp: now,
                });
            }
            for order in subacc.result.open_orders {
                // TODO horribly inefficient to do casting this way but don't want to rewrite schema
                let v = serde_json::to_value(&order).unwrap();
                let order = serde_json::from_value(v).unwrap();
                writer.insert_order(order);
            }
        }
    }

    info!("Subaccount state refreshed");
    for instrument_name in instrument_names {
        let trades = http_rpc::<_, Value>(
            "private/get_trade_history",
            GetTradesParams {
                subaccount_id,
                instrument_name: Some(instrument_name),
                order_id: None,
                quote_id: None,
                from_timestamp: 0,
                to_timestamp: Utc::now().timestamp_millis(),
                page: 1,
                page_size: 1000, // NOTE: assumes there's <= 1000 trades for each instrument
            },
            Some(headers.clone()),
        )
        .await?;
        info!("Trades");
        info!("{:?}", trades);
        if let Response::Success(trades) = trades {
            let trades = serde_json::from_value::<GetTradesResponse>(trades);
            if let Ok(trades) = trades {
                for trade in trades.result.trades {
                    writer.insert_trade(trade);
                }
            } else {
                error!("Failed to get trades with {:?}", trades);
                return Err(Error::msg("Failed to deserialize trades"));
            }
        } else {
            error!("Failed to get trades with {:?}", trades);
            return Err(Error::msg("Failed to get trades"));
        }
    }
    info!("Trades state refreshed");
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SubaccountSubscriberData {
    BalancesMsg(Notification<Vec<BalanceUpdateSchema>>),
    OrdersMsg(Notification<OrderNotificationData>),
    TradesMsg(Notification<TradeNotificationData>),
}

pub async fn subscribe_subaccount(state: MarketState, subaccount_id: i64) -> Result<()> {
    let channels: Vec<String> = vec![
        format!("{}.balances", subaccount_id),
        format!("{}.orders", subaccount_id),
        format!("{}.trades.settled", subaccount_id),
        format!("{}.trades.reverted", subaccount_id),
        format!("{}.trades", subaccount_id),
    ];

    let client = WsClient::new_client().await?;
    let login = client.login().await?.into_result()?;
    info!("Login: {:?}", login);
    info!("Subscribing to subaccount: {:?}", channels);
    client
        .subscribe(channels, |d: SubaccountSubscriberData| async {
            match d {
                SubaccountSubscriberData::BalancesMsg(msg) => {
                    let mut writer = state.write().await;
                    for balance in msg.params.data {
                        writer.insert_position(Balance {
                            instrument_name: balance.name.clone(),
                            amount: balance.new_balance.clone(),
                            timestamp: Utc::now().timestamp_millis(),
                        });
                    }
                }
                SubaccountSubscriberData::OrdersMsg(msg) => {
                    let mut writer = state.write().await;
                    for order in msg.params.data {
                        writer.insert_order(order);
                    }
                }
                SubaccountSubscriberData::TradesMsg(msg) => {
                    let mut writer = state.write().await;
                    for trade in msg.params.data {
                        writer.insert_trade(trade);
                    }
                }
            }
            Ok(())
        })
        .await?;
    Ok(())
}

/// Fetches the balance of a subaccount for a given asset.
pub async fn get_single_balance(subaccount_id: i64, asset_name: &str) -> Result<BigDecimal> {
    let headers = get_auth_headers().await;
    let subaccount = http_rpc::<_, PrivateGetSubaccountResponseSchema>(
        "private/get_subaccount",
        PrivateGetSubaccountParamsSchema { subaccount_id },
        Some(headers.clone()),
    )
    .await?
    .into_result()?;
    let balance = subaccount.result.collaterals.into_iter().find(|p| p.asset_name == asset_name);
    if let Some(balance) = balance {
        return Ok(balance.amount);
    }
    let balance = subaccount.result.positions.into_iter().find(|p| p.instrument_name == asset_name);
    if let Some(balance) = balance {
        return Ok(balance.amount);
    }
    Ok(BigDecimal::zero())
}

pub async fn get_option_expiry(instrument_name: &str) -> Result<i64> {
    let ticker = http_rpc::<_, InstrumentResponse>(
        "public/get_instrument",
        json!({ "instrument_name": instrument_name }),
        None,
    )
    .await?
    .into_result()?;
    Ok(ticker.result.option_details.unwrap().expiry)
}

pub async fn fetch_instrument(instrument_name: &str) -> Result<InstrumentData> {
    let instrument = http_rpc::<_, InstrumentResponse>(
        "public/get_instrument",
        json!({ "instrument_name": instrument_name }),
        None,
    )
    .await?
    .into_result()?;
    Ok(instrument.result)
}

pub async fn fetch_instruments(instrument_names: &Vec<String>) -> Result<Vec<InstrumentData>> {
    let mut out = vec![];
    for name in instrument_names {
        let instrument = fetch_instrument(name).await?;
        out.push(instrument);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
    Ok(out)
}

pub async fn get_expiry_options(
    currency: &str,
    max_expiry_sec: i64,
    min_expiry_sec: i64,
    is_call: bool,
) -> Result<Vec<InstrumentData>> {
    let now = Utc::now().timestamp();
    let options_res = http_rpc::<_, InstrumentsResponse>(
        "public/get_instruments",
        json!({"currency": currency, "instrument_type": "option", "expired": false}),
        None,
    )
    .await?
    .into_result()?
    .result;

    let options_iter = options_res.iter().filter(|&r| {
        if let Some(ref details) = r.option_details {
            r.is_active
                && (is_call == details.option_type.is_call())
                && details.expiry < now + max_expiry_sec
                && details.expiry > now + min_expiry_sec
        } else {
            false
        }
    });

    let err = Error::msg("No options found within the LRTC params");
    let expiry = options_iter.clone().map(|r| r.option_details.as_ref().unwrap().expiry).max();
    let expiry = match expiry {
        Some(expiry) => expiry,
        None => return Err(err),
    };

    let expiry_options: Vec<InstrumentData> = options_iter
        .filter(|r| r.option_details.as_ref().unwrap().expiry == expiry)
        .map(|r| r.clone())
        .collect();

    Ok(expiry_options)
}

pub async fn sleep_till(start_timestamp: i64) {
    let sleep_sec = start_timestamp - Utc::now().timestamp();
    if sleep_sec > 0 {
        info!("Sleeping for {} sec", sleep_sec);
        tokio::time::sleep(tokio::time::Duration::from_secs(sleep_sec as u64)).await;
    }
}
