#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "spot_feed.{currency}",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/SpotFeedCurrencyPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SpotFeedCurrency(pub SpotFeedCurrencyPubSubSchema);
impl std::ops::Deref for SpotFeedCurrency {
    type Target = SpotFeedCurrencyPubSubSchema;
    fn deref(&self) -> &SpotFeedCurrencyPubSubSchema {
        &self.0
    }
}
impl From<SpotFeedCurrency> for SpotFeedCurrencyPubSubSchema {
    fn from(value: SpotFeedCurrency) -> Self {
        value.0
    }
}
impl From<&SpotFeedCurrency> for SpotFeedCurrency {
    fn from(value: &SpotFeedCurrency) -> Self {
        value.clone()
    }
}
impl From<SpotFeedCurrencyPubSubSchema> for SpotFeedCurrency {
    fn from(value: SpotFeedCurrencyPubSubSchema) -> Self {
        Self(value)
    }
}
///SpotFeedCurrencyChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedCurrencyChannelSchema {
    ///Currency
    pub currency: String,
}
impl From<&SpotFeedCurrencyChannelSchema> for SpotFeedCurrencyChannelSchema {
    fn from(value: &SpotFeedCurrencyChannelSchema) -> Self {
        value.clone()
    }
}
///SpotFeedCurrencyNotificationParamsSchema
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
      "$ref": "#/definitions/SpotFeedCurrencyPublisherDataSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedCurrencyNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: SpotFeedCurrencyPublisherDataSchema,
}
impl From<&SpotFeedCurrencyNotificationParamsSchema>
for SpotFeedCurrencyNotificationParamsSchema {
    fn from(value: &SpotFeedCurrencyNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///SpotFeedCurrencyNotificationSchema
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
      "$ref": "#/definitions/SpotFeedCurrencyNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedCurrencyNotificationSchema {
    pub method: String,
    pub params: SpotFeedCurrencyNotificationParamsSchema,
}
impl From<&SpotFeedCurrencyNotificationSchema> for SpotFeedCurrencyNotificationSchema {
    fn from(value: &SpotFeedCurrencyNotificationSchema) -> Self {
        value.clone()
    }
}
///SpotFeedCurrencyPubSubSchema
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
      "$ref": "#/definitions/SpotFeedCurrencyChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/SpotFeedCurrencyNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedCurrencyPubSubSchema {
    pub channel_params: SpotFeedCurrencyChannelSchema,
    pub notification: SpotFeedCurrencyNotificationSchema,
}
impl From<&SpotFeedCurrencyPubSubSchema> for SpotFeedCurrencyPubSubSchema {
    fn from(value: &SpotFeedCurrencyPubSubSchema) -> Self {
        value.clone()
    }
}
///SpotFeedCurrencyPublisherDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "feeds",
    "timestamp"
  ],
  "properties": {
    "feeds": {
      "title": "feeds",
      "description": "Spot feed data",
      "type": "object",
      "additionalProperties": {
        "type": "object",
        "$ref": "#/definitions/SpotFeedSnapshotSchema",
        "field_many": true
      }
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the spot feed snapshot",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedCurrencyPublisherDataSchema {
    ///Spot feed data
    pub feeds: std::collections::HashMap<String, SpotFeedSnapshotSchema>,
    ///Timestamp of the spot feed snapshot
    pub timestamp: i64,
}
impl From<&SpotFeedCurrencyPublisherDataSchema> for SpotFeedCurrencyPublisherDataSchema {
    fn from(value: &SpotFeedCurrencyPublisherDataSchema) -> Self {
        value.clone()
    }
}
///SpotFeedSnapshotSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "confidence",
    "confidence_prev_daily",
    "price",
    "price_prev_daily",
    "timestamp_prev_daily"
  ],
  "properties": {
    "confidence": {
      "title": "confidence",
      "description": "Confidence levels of the price",
      "type": "string",
      "format": "decimal"
    },
    "confidence_prev_daily": {
      "title": "confidence_prev_daily",
      "description": "Confidence levels of the price 24 hours ago",
      "type": "string",
      "format": "decimal"
    },
    "price": {
      "title": "price",
      "description": "Price of the currency",
      "type": "string",
      "format": "decimal"
    },
    "price_prev_daily": {
      "title": "price_prev_daily",
      "description": "Price of the currency 24 hours ago",
      "type": "string",
      "format": "decimal"
    },
    "timestamp_prev_daily": {
      "title": "timestamp_prev_daily",
      "description": "Unix timestamp of the price 24 hours ago",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedSnapshotSchema {
    ///Confidence levels of the price
    pub confidence: bigdecimal::BigDecimal,
    ///Confidence levels of the price 24 hours ago
    pub confidence_prev_daily: bigdecimal::BigDecimal,
    ///Price of the currency
    pub price: bigdecimal::BigDecimal,
    ///Price of the currency 24 hours ago
    pub price_prev_daily: bigdecimal::BigDecimal,
    ///Unix timestamp of the price 24 hours ago
    pub timestamp_prev_daily: i64,
}
impl From<&SpotFeedSnapshotSchema> for SpotFeedSnapshotSchema {
    fn from(value: &SpotFeedSnapshotSchema) -> Self {
        value.clone()
    }
}
