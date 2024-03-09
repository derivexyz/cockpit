#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Order direction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "direction",
  "description": "Order direction",
  "type": "string",
  "enum": [
    "buy",
    "sell"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum Direction {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl From<&Direction> for Direction {
    fn from(value: &Direction) -> Self {
        value.clone()
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Self::Buy => "buy".to_string(),
            Self::Sell => "sell".to_string(),
        }
    }
}
impl std::str::FromStr for Direction {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Direction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Direction {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Direction {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Role of the user in the trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "liquidity_role",
  "description": "Role of the user in the trade",
  "type": "string",
  "enum": [
    "maker",
    "taker"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum LiquidityRole {
    #[serde(rename = "maker")]
    Maker,
    #[serde(rename = "taker")]
    Taker,
}
impl From<&LiquidityRole> for LiquidityRole {
    fn from(value: &LiquidityRole) -> Self {
        value.clone()
    }
}
impl ToString for LiquidityRole {
    fn to_string(&self) -> String {
        match *self {
            Self::Maker => "maker".to_string(),
            Self::Taker => "taker".to_string(),
        }
    }
}
impl std::str::FromStr for LiquidityRole {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "maker" => Ok(Self::Maker),
            "taker" => Ok(Self::Taker),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Subscribe to user's trade settlement for a given subaccount ID.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "{subaccount_id}.trades.{tx_status}",
  "description": "Subscribe to user's trade settlement for a given subaccount ID.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/SubaccountIdTradesTxStatusPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubaccountIdTradesTxStatus(pub SubaccountIdTradesTxStatusPubSubSchema);
impl std::ops::Deref for SubaccountIdTradesTxStatus {
    type Target = SubaccountIdTradesTxStatusPubSubSchema;
    fn deref(&self) -> &SubaccountIdTradesTxStatusPubSubSchema {
        &self.0
    }
}
impl From<SubaccountIdTradesTxStatus> for SubaccountIdTradesTxStatusPubSubSchema {
    fn from(value: SubaccountIdTradesTxStatus) -> Self {
        value.0
    }
}
impl From<&SubaccountIdTradesTxStatus> for SubaccountIdTradesTxStatus {
    fn from(value: &SubaccountIdTradesTxStatus) -> Self {
        value.clone()
    }
}
impl From<SubaccountIdTradesTxStatusPubSubSchema> for SubaccountIdTradesTxStatus {
    fn from(value: SubaccountIdTradesTxStatusPubSubSchema) -> Self {
        Self(value)
    }
}
///SubaccountIdTradesTxStatusChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id",
    "tx_status"
  ],
  "properties": {
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Transaction status (`settled` or `reverted`)",
      "type": "string",
      "enum": [
        "settled",
        "reverted"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdTradesTxStatusChannelSchema {
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Transaction status (`settled` or `reverted`)
    pub tx_status: TxStatus,
}
impl From<&SubaccountIdTradesTxStatusChannelSchema>
for SubaccountIdTradesTxStatusChannelSchema {
    fn from(value: &SubaccountIdTradesTxStatusChannelSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdTradesTxStatusNotificationParamsSchema
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
        "$ref": "#/definitions/TradeResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdTradesTxStatusNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<TradeResponseSchema>,
}
impl From<&SubaccountIdTradesTxStatusNotificationParamsSchema>
for SubaccountIdTradesTxStatusNotificationParamsSchema {
    fn from(value: &SubaccountIdTradesTxStatusNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdTradesTxStatusNotificationSchema
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
      "$ref": "#/definitions/SubaccountIdTradesTxStatusNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdTradesTxStatusNotificationSchema {
    pub method: String,
    pub params: SubaccountIdTradesTxStatusNotificationParamsSchema,
}
impl From<&SubaccountIdTradesTxStatusNotificationSchema>
for SubaccountIdTradesTxStatusNotificationSchema {
    fn from(value: &SubaccountIdTradesTxStatusNotificationSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdTradesTxStatusPubSubSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel_params",
    "notification"
  ],
  "properties": {
    "channel_params": {
      "type": "object",
      "$ref": "#/definitions/SubaccountIdTradesTxStatusChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/SubaccountIdTradesTxStatusNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdTradesTxStatusPubSubSchema {
    pub channel_params: SubaccountIdTradesTxStatusChannelSchema,
    pub notification: SubaccountIdTradesTxStatusNotificationSchema,
}
impl From<&SubaccountIdTradesTxStatusPubSubSchema>
for SubaccountIdTradesTxStatusPubSubSchema {
    fn from(value: &SubaccountIdTradesTxStatusPubSubSchema) -> Self {
        value.clone()
    }
}
///TradeResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "direction",
    "index_price",
    "instrument_name",
    "is_transfer",
    "label",
    "liquidity_role",
    "mark_price",
    "order_id",
    "quote_id",
    "realized_pnl",
    "subaccount_id",
    "timestamp",
    "trade_amount",
    "trade_fee",
    "trade_id",
    "trade_price",
    "tx_hash",
    "tx_status"
  ],
  "properties": {
    "direction": {
      "title": "direction",
      "description": "Order direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "index_price": {
      "title": "index_price",
      "description": "Index price of the underlying at the time of the trade",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "is_transfer": {
      "title": "is_transfer",
      "description": "Whether the trade was generated through `private/transfer_position`",
      "type": "boolean"
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "type": "string"
    },
    "liquidity_role": {
      "title": "liquidity_role",
      "description": "Role of the user in the trade",
      "type": "string",
      "enum": [
        "maker",
        "taker"
      ]
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Mark price of the instrument at the time of the trade",
      "type": "string",
      "format": "decimal"
    },
    "order_id": {
      "title": "order_id",
      "description": "Order ID",
      "type": "string"
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID if the trade was executed via RFQ",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "realized_pnl": {
      "title": "realized_pnl",
      "description": "Realized PnL for this trade",
      "type": "string",
      "format": "decimal"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Trade timestamp (in ms since Unix epoch)",
      "type": "integer"
    },
    "trade_amount": {
      "title": "trade_amount",
      "description": "Amount filled in this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_fee": {
      "title": "trade_fee",
      "description": "Fee for this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_id": {
      "title": "trade_id",
      "description": "Trade ID",
      "type": "string"
    },
    "trade_price": {
      "title": "trade_price",
      "description": "Price at which the trade was filled",
      "type": "string",
      "format": "decimal"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Blockchain transaction hash",
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status",
      "type": "string",
      "enum": [
        "requested",
        "pending",
        "settled",
        "reverted",
        "ignored"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeResponseSchema {
    ///Order direction
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Whether the trade was generated through `private/transfer_position`
    pub is_transfer: bool,
    ///Optional user-defined label for the order
    pub label: String,
    ///Role of the user in the trade
    pub liquidity_role: LiquidityRole,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
    ///Order ID
    pub order_id: String,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///Realized PnL for this trade
    pub realized_pnl: bigdecimal::BigDecimal,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Trade timestamp (in ms since Unix epoch)
    pub timestamp: i64,
    ///Amount filled in this trade
    pub trade_amount: bigdecimal::BigDecimal,
    ///Fee for this trade
    pub trade_fee: bigdecimal::BigDecimal,
    ///Trade ID
    pub trade_id: String,
    ///Price at which the trade was filled
    pub trade_price: bigdecimal::BigDecimal,
    ///Blockchain transaction hash
    pub tx_hash: Option<String>,
    ///Blockchain transaction status
    pub tx_status: TxStatus,
}
impl From<&TradeResponseSchema> for TradeResponseSchema {
    fn from(value: &TradeResponseSchema) -> Self {
        value.clone()
    }
}
///Transaction status (`settled` or `reverted`)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Transaction status (`settled` or `reverted`)",
  "type": "string",
  "enum": [
    "settled",
    "reverted"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum TxStatus {
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
}
impl From<&TxStatus> for TxStatus {
    fn from(value: &TxStatus) -> Self {
        value.clone()
    }
}
impl ToString for TxStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
        }
    }
}
impl std::str::FromStr for TxStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
