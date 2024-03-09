#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin_type",
  "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
  "type": "string",
  "enum": [
    "PM",
    "SM"
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
pub enum MarginType {
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SM")]
    Sm,
}
impl From<&MarginType> for MarginType {
    fn from(value: &MarginType) -> Self {
        value.clone()
    }
}
impl ToString for MarginType {
    fn to_string(&self) -> String {
        match *self {
            Self::Pm => "PM".to_string(),
            Self::Sm => "SM".to_string(),
        }
    }
}
impl std::str::FromStr for MarginType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PM" => Ok(Self::Pm),
            "SM" => Ok(Self::Sm),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for MarginType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MarginType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MarginType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Subscribe to state of margin and MtM of all users.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin.watch",
  "description": "Subscribe to state of margin and MtM of all users.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/MarginWatchPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarginWatch(pub MarginWatchPubSubSchema);
impl std::ops::Deref for MarginWatch {
    type Target = MarginWatchPubSubSchema;
    fn deref(&self) -> &MarginWatchPubSubSchema {
        &self.0
    }
}
impl From<MarginWatch> for MarginWatchPubSubSchema {
    fn from(value: MarginWatch) -> Self {
        value.0
    }
}
impl From<&MarginWatch> for MarginWatch {
    fn from(value: &MarginWatch) -> Self {
        value.clone()
    }
}
impl From<MarginWatchPubSubSchema> for MarginWatch {
    fn from(value: MarginWatchPubSubSchema) -> Self {
        Self(value)
    }
}
///MarginWatchChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MarginWatchChannelSchema {}
impl From<&MarginWatchChannelSchema> for MarginWatchChannelSchema {
    fn from(value: &MarginWatchChannelSchema) -> Self {
        value.clone()
    }
}
///MarginWatchNotificationParamsSchema
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
        "$ref": "#/definitions/MarginWatchResultSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MarginWatchNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<MarginWatchResultSchema>,
}
impl From<&MarginWatchNotificationParamsSchema> for MarginWatchNotificationParamsSchema {
    fn from(value: &MarginWatchNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///MarginWatchNotificationSchema
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
      "$ref": "#/definitions/MarginWatchNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MarginWatchNotificationSchema {
    pub method: String,
    pub params: MarginWatchNotificationParamsSchema,
}
impl From<&MarginWatchNotificationSchema> for MarginWatchNotificationSchema {
    fn from(value: &MarginWatchNotificationSchema) -> Self {
        value.clone()
    }
}
///MarginWatchPubSubSchema
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
      "$ref": "#/definitions/MarginWatchChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/MarginWatchNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MarginWatchPubSubSchema {
    pub channel_params: MarginWatchChannelSchema,
    pub notification: MarginWatchNotificationSchema,
}
impl From<&MarginWatchPubSubSchema> for MarginWatchPubSubSchema {
    fn from(value: &MarginWatchPubSubSchema) -> Self {
        value.clone()
    }
}
///MarginWatchResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "maintenance_margin",
    "margin_type",
    "subaccount_id",
    "subaccount_value",
    "valuation_timestamp"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency of subaccount",
      "type": "string"
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.",
      "type": "string",
      "format": "decimal"
    },
    "margin_type": {
      "title": "margin_type",
      "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
      "type": "string",
      "enum": [
        "PM",
        "SM"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "subaccount_value": {
      "title": "subaccount_value",
      "description": "Total mark-to-market value of all positions and collaterals",
      "type": "string",
      "format": "decimal"
    },
    "valuation_timestamp": {
      "title": "valuation_timestamp",
      "description": "Timestamp (in seconds since epoch) of when margin and MtM were computed.",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct MarginWatchResultSchema {
    ///Currency of subaccount
    pub currency: String,
    ///Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
    pub margin_type: MarginType,
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Total mark-to-market value of all positions and collaterals
    pub subaccount_value: bigdecimal::BigDecimal,
    ///Timestamp (in seconds since epoch) of when margin and MtM were computed.
    pub valuation_timestamp: i64,
}
impl From<&MarginWatchResultSchema> for MarginWatchResultSchema {
    fn from(value: &MarginWatchResultSchema) -> Self {
        value.clone()
    }
}
