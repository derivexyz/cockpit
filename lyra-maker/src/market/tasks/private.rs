use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

use lyra_client::json_rpc::{Notification, Response, WsClient, WsClientExt};
use orderbook_types::generated::private_get_subaccount::{PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema};
use orderbook_types::generated::channel_subaccount_id_balances::BalanceUpdateSchema;
use orderbook_types::types::orders::{OrderResponse, OrderNotificationData};
use chrono::Utc;
use log::{info, warn, error, debug};

use crate::market::core::{MarketState, Balance};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum SubaccountSubscriberData {
    BalancesMsg(Notification<Vec<BalanceUpdateSchema>>),
    OrdersMsg(Notification<OrderNotificationData>),
}

pub async fn start_subaccount(state: MarketState, subaccount_id: i64) -> Result<()> {
    let channels: Vec<String> = vec![
        format!("{}.balances", subaccount_id),
        format!("{}.orders", subaccount_id),
    ];
    let client = WsClient::new_client().await?;
    let login = client.login().await?;
    info!("Login: {:?}", login);
    let subacc = client.send_rpc::<_, PrivateGetSubaccountResponseSchema>("private/get_subaccount", PrivateGetSubaccountParamsSchema { subaccount_id }).await;
    info!("Subaccount state refreshed");

    if let Response::Success(subacc) = subacc? {
        let now = Utc::now().timestamp_millis();
        let mut writer = state.write().await;
        for position in subacc.result.positions {
            writer.insert_position(Balance {
                instrument_name: position.instrument_name,
                amount: position.amount,
                timestamp: now
            });
        }
        for collateral in subacc.result.collaterals {
            writer.insert_position(Balance {
                instrument_name: collateral.asset_name,
                amount: collateral.amount,
                timestamp: now
            });
        }
        for order in subacc.result.open_orders {
            // TODO horribly inefficient to do casting this way but only done once
            let v = serde_json::to_value(&order).unwrap();
            let order = serde_json::from_value(v).unwrap();
            writer.insert_order(order);
        }
    } else {
        return Err(Error::msg("Failed to get subaccount"));
    }

    // TODO can regularly re-check the balances and orders via RPC as a validation
    client.subscribe(channels, |d: SubaccountSubscriberData| async {
        match d {
            SubaccountSubscriberData::BalancesMsg(notification) => {
                let mut writer = state.write().await;
                for balance in notification.params.data {
                    writer.insert_position(Balance {
                        instrument_name: balance.name.clone(),
                        amount: balance.new_balance.clone(),
                        timestamp: Utc::now().timestamp_millis()
                    });
                }
            }
            SubaccountSubscriberData::OrdersMsg(notification) => {
                let mut writer = state.write().await;
                for order in notification.params.data {
                    writer.insert_order(order);
                }
            }
        }
        Ok(())
    }).await?;
    Ok(())
}
