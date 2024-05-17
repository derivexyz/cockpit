use crate::market::{new_market_state, Balance, MarketState};
use lyra_client::auth::get_auth_headers;
use lyra_client::json_rpc::{http_rpc, Notification, Response, WsClient, WsClientExt};

use orderbook_types::generated::channel_subaccount_id_balances::BalanceUpdateSchema;
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::types::tickers::result::{
    InstrumentTicker, InstrumentsResponse, OptionType, TickerNotificationData,
};

use anyhow::{Error, Result};
use chrono::Utc;
use log::{debug, error, info, warn};
use orderbook_types::types::orders::{
    GetTradesParams, GetTradesResponse, OrderNotificationData, OrderResponse,
    TradeNotificationData, TxStatus,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::select;

type TickerMsg = Notification<TickerNotificationData>;

#[derive(Copy, Clone)]
pub enum TickerInterval {
    _1000Ms = 1000,
    _100Ms = 100,
}

pub async fn subscribe_tickers(
    market: MarketState,
    instrument_names: Vec<String>,
    interval: TickerInterval,
) -> Result<()> {
    let channels: Vec<String> = instrument_names
        .iter()
        .map(|instrument_name| format!("ticker.{}.{}", instrument_name, interval as u32))
        .collect();
    let client = WsClient::new_client().await?;
    info!("Subscribing to tickers: {:?}", channels);
    client
        .subscribe(channels, |msg: TickerMsg| async {
            market.write().await.insert_ticker(msg.params.data.instrument_ticker);
            Ok(())
        })
        .await?;
    Ok(())
}

pub async fn sync_subaccount(
    state: MarketState,
    subaccount_id: i64,
    instrument_names: Vec<String>,
) -> Result<()> {
    let headers = get_auth_headers().await;
    let subacc = http_rpc::<_, PrivateGetSubaccountResponseSchema>(
        "private/get_subaccount",
        PrivateGetSubaccountParamsSchema { subaccount_id },
        Some(headers.clone()),
    )
    .await;
    info!("Subaccount state fetched");
    let mut writer = state.write().await;
    if let Response::Success(subacc) = subacc? {
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
    } else {
        return Err(Error::msg("Failed to get subaccount"));
    }
    info!("Subaccount state refreshed");
    for instrument_name in instrument_names {
        let trades = http_rpc::<_, GetTradesResponse>(
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
        if let Response::Success(trades) = trades {
            for trade in trades.result.trades {
                writer.insert_trade(trade);
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
