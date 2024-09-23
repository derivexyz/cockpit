#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::types::liquidations::enums::MarginType;
use crate::types::liquidations::AuctionState;
use crate::types::orders::{OrderResponse, TradeResponse};
use crate::types::RPCId;
use bigdecimal;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid;

///AuctionsWatchNotificationParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel",
    "data"
  ],
  "properties": {
    "channel": {
      "title": "channel",
      "description": "Subscribed channel name",
      "type": "string"
    },
    "data": {
      "title": "data",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/AuctionsWatchResultSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AuctionsWatchNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<AuctionsWatchResultSchema>,
}
impl From<&AuctionsWatchNotificationParamsSchema> for AuctionsWatchNotificationParamsSchema {
    fn from(value: &AuctionsWatchNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///AuctionsWatchNotificationSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "method",
    "params"
  ],
  "properties": {
    "method": {
      "title": "method",
      "type": "string"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/AuctionsWatchNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AuctionsWatchNotificationSchema {
    pub method: String,
    pub params: AuctionsWatchNotificationParamsSchema,
}
impl From<&AuctionsWatchNotificationSchema> for AuctionsWatchNotificationSchema {
    fn from(value: &AuctionsWatchNotificationSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionDetailsSchema {
    ///Currency of subaccount
    pub currency: Option<String>,
    ///Estimated bid price
    pub estimated_bid_price: BigDecimal,
    ///Estimated discount pnl
    pub estimated_discount_pnl: BigDecimal,
    ///Estimated mark-to-market
    pub estimated_mtm: BigDecimal,
    ///Estimated percent bid
    pub estimated_percent_bid: BigDecimal,
    ///Last seen trade id
    pub last_seen_trade_id: i64,
    ///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
    pub margin_type: MarginType,
    ///Minimum cash transfer
    pub min_cash_transfer: BigDecimal,
    ///Minimum price limit
    pub min_price_limit: BigDecimal,
    ///Subaccount balances
    pub subaccount_balances: std::collections::HashMap<String, BigDecimal>,
}

impl AuctionDetailsSchema {
    pub fn buffer(&self) -> BigDecimal {
        let buffer = &self.min_cash_transfer * BigDecimal::from_str("0.05").unwrap();
        buffer.max(BigDecimal::from_str("10").unwrap())
    }
    pub fn cash_transfer_with_buffer(&self) -> BigDecimal {
        let buffer = self.buffer();
        let cash_transfer = &self.min_cash_transfer + &buffer;
        cash_transfer
    }
    pub fn price_limit_with_buffer(&self) -> BigDecimal {
        let buffer = self.buffer();
        let price_limit = &self.estimated_bid_price + &buffer;
        price_limit
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionsWatchResultSchema {
    pub details: Option<AuctionDetailsSchema>,
    pub state: AuctionState,
    pub subaccount_id: i64,
    pub timestamp: i64,
}

impl From<&AuctionsWatchResultSchema> for AuctionsWatchResultSchema {
    fn from(value: &AuctionsWatchResultSchema) -> Self {
        value.clone()
    }
}

pub type AuctionsWatchData = Vec<AuctionsWatchResultSchema>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendLiquidateResult {
    pub estimated_percent_bid: BigDecimal,
    pub estimated_bid_price: BigDecimal,
    pub estimated_discount_pnl: BigDecimal,
    pub transaction_id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendLiquidateResponse {
    pub id: RPCId,
    pub result: SendLiquidateResult,
}
