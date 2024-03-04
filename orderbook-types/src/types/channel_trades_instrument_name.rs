#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Direction of the taker order
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "direction",
  "description": "Direction of the taker order",
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
///TradePublicResponseSchema
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
    "mark_price",
    "quote_id",
    "timestamp",
    "trade_amount",
    "trade_id",
    "trade_price"
  ],
  "properties": {
    "direction": {
      "title": "direction",
      "description": "Direction of the taker order",
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
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradePublicResponseSchema {
    ///Direction of the taker order
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///Trade timestamp (in ms since Unix epoch)
    pub timestamp: i64,
    ///Amount filled in this trade
    pub trade_amount: bigdecimal::BigDecimal,
    ///Trade ID
    pub trade_id: String,
    ///Price at which the trade was filled
    pub trade_price: bigdecimal::BigDecimal,
}
impl From<&TradePublicResponseSchema> for TradePublicResponseSchema {
    fn from(value: &TradePublicResponseSchema) -> Self {
        value.clone()
    }
}
///Subscribe to trades (order executions) for a given instrument name.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "trades.{instrument_name}",
  "description": "Subscribe to trades (order executions) for a given instrument name.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/TradesInstrumentNamePubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradesInstrumentName(pub TradesInstrumentNamePubSubSchema);
impl std::ops::Deref for TradesInstrumentName {
    type Target = TradesInstrumentNamePubSubSchema;
    fn deref(&self) -> &TradesInstrumentNamePubSubSchema {
        &self.0
    }
}
impl From<TradesInstrumentName> for TradesInstrumentNamePubSubSchema {
    fn from(value: TradesInstrumentName) -> Self {
        value.0
    }
}
impl From<&TradesInstrumentName> for TradesInstrumentName {
    fn from(value: &TradesInstrumentName) -> Self {
        value.clone()
    }
}
impl From<TradesInstrumentNamePubSubSchema> for TradesInstrumentName {
    fn from(value: TradesInstrumentNamePubSubSchema) -> Self {
        Self(value)
    }
}
///TradesInstrumentNameChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_name"
  ],
  "properties": {
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentNameChannelSchema {
    ///Instrument name
    pub instrument_name: String,
}
impl From<&TradesInstrumentNameChannelSchema> for TradesInstrumentNameChannelSchema {
    fn from(value: &TradesInstrumentNameChannelSchema) -> Self {
        value.clone()
    }
}
///TradesInstrumentNameNotificationParamsSchema
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
        "$ref": "#/definitions/TradePublicResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentNameNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<TradePublicResponseSchema>,
}
impl From<&TradesInstrumentNameNotificationParamsSchema>
for TradesInstrumentNameNotificationParamsSchema {
    fn from(value: &TradesInstrumentNameNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///TradesInstrumentNameNotificationSchema
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
      "$ref": "#/definitions/TradesInstrumentNameNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentNameNotificationSchema {
    pub method: String,
    pub params: TradesInstrumentNameNotificationParamsSchema,
}
impl From<&TradesInstrumentNameNotificationSchema>
for TradesInstrumentNameNotificationSchema {
    fn from(value: &TradesInstrumentNameNotificationSchema) -> Self {
        value.clone()
    }
}
///TradesInstrumentNamePubSubSchema
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
      "$ref": "#/definitions/TradesInstrumentNameChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/TradesInstrumentNameNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradesInstrumentNamePubSubSchema {
    pub channel_params: TradesInstrumentNameChannelSchema,
    pub notification: TradesInstrumentNameNotificationSchema,
}
impl From<&TradesInstrumentNamePubSubSchema> for TradesInstrumentNamePubSubSchema {
    fn from(value: &TradesInstrumentNamePubSubSchema) -> Self {
        value.clone()
    }
}
