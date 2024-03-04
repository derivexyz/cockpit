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
///Instrument type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "instrument_type",
  "description": "Instrument type",
  "type": "string",
  "enum": [
    "erc20",
    "option",
    "perp"
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
pub enum InstrumentType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}
impl From<&InstrumentType> for InstrumentType {
    fn from(value: &InstrumentType) -> Self {
        value.clone()
    }
}
impl ToString for InstrumentType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}
impl std::str::FromStr for InstrumentType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "erc20" => Ok(Self::Erc20),
            "option" => Ok(Self::Option),
            "perp" => Ok(Self::Perp),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InstrumentType {
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
///TradeSettledPublicResponseSchema
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
    "liquidity_role",
    "mark_price",
    "quote_id",
    "realized_pnl",
    "subaccount_id",
    "timestamp",
    "trade_amount",
    "trade_fee",
    "trade_id",
    "trade_price",
    "tx_hash",
    "tx_status",
    "wallet"
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
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID if the trade was executed via RFQ",
      "default": null,
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
      "type": "string"
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status",
      "type": "string",
      "enum": [
        "settled",
        "reverted"
      ]
    },
    "wallet": {
      "title": "wallet",
      "description": "Wallet address (owner) of the subaccount",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeSettledPublicResponseSchema {
    ///Order direction
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Role of the user in the trade
    pub liquidity_role: LiquidityRole,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
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
    pub tx_hash: String,
    ///Blockchain transaction status
    pub tx_status: TxStatus,
    ///Wallet address (owner) of the subaccount
    pub wallet: String,
}
impl From<&TradeSettledPublicResponseSchema> for TradeSettledPublicResponseSchema {
    fn from(value: &TradeSettledPublicResponseSchema) -> Self {
        value.clone()
    }
}
///Subscribe to the status on on-chain trade settlement events for a given instrument type and currency.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "trades.{instrument_type}.{currency}.{tx_status}",
  "description": "Subscribe to the status on on-chain trade settlement events for a given instrument type and currency.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/TradesInstrumentTypeCurrencyTxStatusPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradesInstrumentTypeCurrencyTxStatus(
    pub TradesInstrumentTypeCurrencyTxStatusPubSubSchema,
);
impl std::ops::Deref for TradesInstrumentTypeCurrencyTxStatus {
    type Target = TradesInstrumentTypeCurrencyTxStatusPubSubSchema;
    fn deref(&self) -> &TradesInstrumentTypeCurrencyTxStatusPubSubSchema {
        &self.0
    }
}
impl From<TradesInstrumentTypeCurrencyTxStatus>
for TradesInstrumentTypeCurrencyTxStatusPubSubSchema {
    fn from(value: TradesInstrumentTypeCurrencyTxStatus) -> Self {
        value.0
    }
}
impl From<&TradesInstrumentTypeCurrencyTxStatus>
for TradesInstrumentTypeCurrencyTxStatus {
    fn from(value: &TradesInstrumentTypeCurrencyTxStatus) -> Self {
        value.clone()
    }
}
impl From<TradesInstrumentTypeCurrencyTxStatusPubSubSchema>
for TradesInstrumentTypeCurrencyTxStatus {
    fn from(value: TradesInstrumentTypeCurrencyTxStatusPubSubSchema) -> Self {
        Self(value)
    }
}
///TradesInstrumentTypeCurrencyTxStatusChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "instrument_type",
    "tx_status"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    },
    "instrument_type": {
      "title": "instrument_type",
      "description": "Instrument type",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Transaction status",
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

pub struct TradesInstrumentTypeCurrencyTxStatusChannelSchema {
    ///Currency
    pub currency: String,
    ///Instrument type
    pub instrument_type: InstrumentType,
    ///Transaction status
    pub tx_status: TxStatus,
}
impl From<&TradesInstrumentTypeCurrencyTxStatusChannelSchema>
for TradesInstrumentTypeCurrencyTxStatusChannelSchema {
    fn from(value: &TradesInstrumentTypeCurrencyTxStatusChannelSchema) -> Self {
        value.clone()
    }
}
///TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema
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
        "$ref": "#/definitions/TradeSettledPublicResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<TradeSettledPublicResponseSchema>,
}
impl From<&TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema>
for TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema {
    fn from(
        value: &TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema,
    ) -> Self {
        value.clone()
    }
}
///TradesInstrumentTypeCurrencyTxStatusNotificationSchema
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
      "$ref": "#/definitions/TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentTypeCurrencyTxStatusNotificationSchema {
    pub method: String,
    pub params: TradesInstrumentTypeCurrencyTxStatusNotificationParamsSchema,
}
impl From<&TradesInstrumentTypeCurrencyTxStatusNotificationSchema>
for TradesInstrumentTypeCurrencyTxStatusNotificationSchema {
    fn from(value: &TradesInstrumentTypeCurrencyTxStatusNotificationSchema) -> Self {
        value.clone()
    }
}
///TradesInstrumentTypeCurrencyTxStatusPubSubSchema
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
      "$ref": "#/definitions/TradesInstrumentTypeCurrencyTxStatusChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/TradesInstrumentTypeCurrencyTxStatusNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentTypeCurrencyTxStatusPubSubSchema {
    pub channel_params: TradesInstrumentTypeCurrencyTxStatusChannelSchema,
    pub notification: TradesInstrumentTypeCurrencyTxStatusNotificationSchema,
}
impl From<&TradesInstrumentTypeCurrencyTxStatusPubSubSchema>
for TradesInstrumentTypeCurrencyTxStatusPubSubSchema {
    fn from(value: &TradesInstrumentTypeCurrencyTxStatusPubSubSchema) -> Self {
        value.clone()
    }
}
///Blockchain transaction status
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Blockchain transaction status",
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
