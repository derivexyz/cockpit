use serde::{Deserialize, Serialize};
use anyhow::{Result, Error};

use lyra_client::json_rpc::{Notification, Response, WsClient, WsClientExt};
use orderbook_types::types::channel_ticker_instrument_name_interval::{
    InstrumentTickerSchema, TickerInstrumentNameIntervalPublisherDataSchema,
};
use crate::market::core::{MarketState, OrderbookData};
use log::{info, warn, error, debug};


#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum MarketSubscriberData {
    OrderbookMsg(Notification<OrderbookData>),
    TickerMsg(Notification<TickerInstrumentNameIntervalPublisherDataSchema>),
}


pub async fn start_market(state: MarketState, instrument_names: Vec<String>) -> Result<()> {
    let channels: Vec<String> = instrument_names
        .iter()
        .flat_map(|instrument_name| {
            vec![
                format!("orderbook.{}.1.10", instrument_name),
                format!("ticker.{}.1000", instrument_name),
            ]
        })
        .collect();

    let client = WsClient::new_client().await?;
    client.subscribe(channels, |d: MarketSubscriberData| async {
        match d {
            MarketSubscriberData::OrderbookMsg(notification) => {
                state.write().await.insert_orderbook(notification.params.data);
            }
            MarketSubscriberData::TickerMsg(notification) => {
                state.write().await.insert_ticker(notification.params.data.instrument_ticker);
            }
        }
        Ok(())
    }).await?;
    Ok(())
}
