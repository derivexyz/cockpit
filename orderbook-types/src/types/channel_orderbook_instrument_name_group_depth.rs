#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Number of price levels returned
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "depth",
  "description": "Number of price levels returned",
  "type": "string",
  "enum": [
    "1",
    "10",
    "20",
    "100"
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
pub enum Depth {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "20")]
    _20,
    #[serde(rename = "100")]
    _100,
}
impl From<&Depth> for Depth {
    fn from(value: &Depth) -> Self {
        value.clone()
    }
}
impl ToString for Depth {
    fn to_string(&self) -> String {
        match *self {
            Self::_1 => "1".to_string(),
            Self::_10 => "10".to_string(),
            Self::_20 => "20".to_string(),
            Self::_100 => "100".to_string(),
        }
    }
}
impl std::str::FromStr for Depth {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "1" => Ok(Self::_1),
            "10" => Ok(Self::_10),
            "20" => Ok(Self::_20),
            "100" => Ok(Self::_100),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Depth {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Depth {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Depth {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Price grouping (rounding)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "group",
  "description": "Price grouping (rounding)",
  "type": "string",
  "enum": [
    "1",
    "10",
    "100"
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
pub enum Group {
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "10")]
    _10,
    #[serde(rename = "100")]
    _100,
}
impl From<&Group> for Group {
    fn from(value: &Group) -> Self {
        value.clone()
    }
}
impl ToString for Group {
    fn to_string(&self) -> String {
        match *self {
            Self::_1 => "1".to_string(),
            Self::_10 => "10".to_string(),
            Self::_100 => "100".to_string(),
        }
    }
}
impl std::str::FromStr for Group {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "1" => Ok(Self::_1),
            "10" => Ok(Self::_10),
            "100" => Ok(Self::_100),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Group {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Group {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Group {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Periodically publishes bids and asks for an instrument.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "orderbook.{instrument_name}.{group}.{depth}",
  "description": "Periodically publishes bids and asks for an instrument.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/OrderbookInstrumentNameGroupDepthPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderbookInstrumentNameGroupDepth(
    pub OrderbookInstrumentNameGroupDepthPubSubSchema,
);
impl std::ops::Deref for OrderbookInstrumentNameGroupDepth {
    type Target = OrderbookInstrumentNameGroupDepthPubSubSchema;
    fn deref(&self) -> &OrderbookInstrumentNameGroupDepthPubSubSchema {
        &self.0
    }
}
impl From<OrderbookInstrumentNameGroupDepth>
for OrderbookInstrumentNameGroupDepthPubSubSchema {
    fn from(value: OrderbookInstrumentNameGroupDepth) -> Self {
        value.0
    }
}
impl From<&OrderbookInstrumentNameGroupDepth> for OrderbookInstrumentNameGroupDepth {
    fn from(value: &OrderbookInstrumentNameGroupDepth) -> Self {
        value.clone()
    }
}
impl From<OrderbookInstrumentNameGroupDepthPubSubSchema>
for OrderbookInstrumentNameGroupDepth {
    fn from(value: OrderbookInstrumentNameGroupDepthPubSubSchema) -> Self {
        Self(value)
    }
}
///OrderbookInstrumentNameGroupDepthChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "depth",
    "group",
    "instrument_name"
  ],
  "properties": {
    "depth": {
      "title": "depth",
      "description": "Number of price levels returned",
      "type": "string",
      "enum": [
        "1",
        "10",
        "20",
        "100"
      ]
    },
    "group": {
      "title": "group",
      "description": "Price grouping (rounding)",
      "type": "string",
      "enum": [
        "1",
        "10",
        "100"
      ]
    },
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

pub struct OrderbookInstrumentNameGroupDepthChannelSchema {
    ///Number of price levels returned
    pub depth: Depth,
    ///Price grouping (rounding)
    pub group: Group,
    ///Instrument name
    pub instrument_name: String,
}
impl From<&OrderbookInstrumentNameGroupDepthChannelSchema>
for OrderbookInstrumentNameGroupDepthChannelSchema {
    fn from(value: &OrderbookInstrumentNameGroupDepthChannelSchema) -> Self {
        value.clone()
    }
}
///OrderbookInstrumentNameGroupDepthNotificationParamsSchema
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
      "type": "object",
      "$ref": "#/definitions/OrderbookInstrumentNameGroupDepthPublisherDataSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OrderbookInstrumentNameGroupDepthNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: OrderbookInstrumentNameGroupDepthPublisherDataSchema,
}
impl From<&OrderbookInstrumentNameGroupDepthNotificationParamsSchema>
for OrderbookInstrumentNameGroupDepthNotificationParamsSchema {
    fn from(value: &OrderbookInstrumentNameGroupDepthNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///OrderbookInstrumentNameGroupDepthNotificationSchema
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
      "$ref": "#/definitions/OrderbookInstrumentNameGroupDepthNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OrderbookInstrumentNameGroupDepthNotificationSchema {
    pub method: String,
    pub params: OrderbookInstrumentNameGroupDepthNotificationParamsSchema,
}
impl From<&OrderbookInstrumentNameGroupDepthNotificationSchema>
for OrderbookInstrumentNameGroupDepthNotificationSchema {
    fn from(value: &OrderbookInstrumentNameGroupDepthNotificationSchema) -> Self {
        value.clone()
    }
}
///OrderbookInstrumentNameGroupDepthPubSubSchema
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
      "$ref": "#/definitions/OrderbookInstrumentNameGroupDepthChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/OrderbookInstrumentNameGroupDepthNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OrderbookInstrumentNameGroupDepthPubSubSchema {
    pub channel_params: OrderbookInstrumentNameGroupDepthChannelSchema,
    pub notification: OrderbookInstrumentNameGroupDepthNotificationSchema,
}
impl From<&OrderbookInstrumentNameGroupDepthPubSubSchema>
for OrderbookInstrumentNameGroupDepthPubSubSchema {
    fn from(value: &OrderbookInstrumentNameGroupDepthPubSubSchema) -> Self {
        value.clone()
    }
}
///OrderbookInstrumentNameGroupDepthPublisherDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "asks",
    "bids",
    "instrument_name",
    "publish_id",
    "timestamp"
  ],
  "properties": {
    "asks": {
      "title": "asks",
      "description": "List of asks as [price, amount] tuples optionally grouped into price buckets",
      "type": "array",
      "items": {
        "title": "asks",
        "type": "array",
        "items": {
          "title": "asks",
          "type": "string",
          "format": "decimal"
        }
      }
    },
    "bids": {
      "title": "bids",
      "description": "List of bids as [price, amount] tuples optionally grouped into price buckets",
      "type": "array",
      "items": {
        "title": "bids",
        "type": "array",
        "items": {
          "title": "bids",
          "type": "string",
          "format": "decimal"
        }
      }
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "publish_id": {
      "title": "publish_id",
      "description": "Publish ID, incremented for each publish",
      "type": "integer"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the orderbook snapshot",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OrderbookInstrumentNameGroupDepthPublisherDataSchema {
    ///List of asks as [price, amount] tuples optionally grouped into price buckets
    pub asks: Vec<Vec<bigdecimal::BigDecimal>>,
    ///List of bids as [price, amount] tuples optionally grouped into price buckets
    pub bids: Vec<Vec<bigdecimal::BigDecimal>>,
    ///Instrument name
    pub instrument_name: String,
    ///Publish ID, incremented for each publish
    pub publish_id: i64,
    ///Timestamp of the orderbook snapshot
    pub timestamp: i64,
}
impl From<&OrderbookInstrumentNameGroupDepthPublisherDataSchema>
for OrderbookInstrumentNameGroupDepthPublisherDataSchema {
    fn from(value: &OrderbookInstrumentNameGroupDepthPublisherDataSchema) -> Self {
        value.clone()
    }
}
