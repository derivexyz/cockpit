#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
/**Get spot feed history by currency

DB: read replica*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_spot_feed_history",
  "description": "Get spot feed history by currency\n\nDB: read replica",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetSpotFeedHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetSpotFeedHistory(pub PublicGetSpotFeedHistoryJsonrpcSchema);
impl std::ops::Deref for PublicGetSpotFeedHistory {
    type Target = PublicGetSpotFeedHistoryJsonrpcSchema;
    fn deref(&self) -> &PublicGetSpotFeedHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetSpotFeedHistory> for PublicGetSpotFeedHistoryJsonrpcSchema {
    fn from(value: PublicGetSpotFeedHistory) -> Self {
        value.0
    }
}
impl From<&PublicGetSpotFeedHistory> for PublicGetSpotFeedHistory {
    fn from(value: &PublicGetSpotFeedHistory) -> Self {
        value.clone()
    }
}
impl From<PublicGetSpotFeedHistoryJsonrpcSchema> for PublicGetSpotFeedHistory {
    fn from(value: PublicGetSpotFeedHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetSpotFeedHistoryJsonrpcSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "request",
    "response"
  ],
  "properties": {
    "request": {
      "type": "object",
      "$ref": "#/definitions/PublicGetSpotFeedHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetSpotFeedHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetSpotFeedHistoryJsonrpcSchema {
    pub request: PublicGetSpotFeedHistoryRequestSchema,
    pub response: PublicGetSpotFeedHistoryResponseSchema,
}
impl From<&PublicGetSpotFeedHistoryJsonrpcSchema>
for PublicGetSpotFeedHistoryJsonrpcSchema {
    fn from(value: &PublicGetSpotFeedHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetSpotFeedHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "end_timestamp",
    "period",
    "start_timestamp"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    },
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "End timestamp",
      "type": "integer"
    },
    "period": {
      "title": "period",
      "description": "Period",
      "type": "integer"
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Start timestamp",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetSpotFeedHistoryParamsSchema {
    ///Currency
    pub currency: String,
    ///End timestamp
    pub end_timestamp: i64,
    ///Period
    pub period: i64,
    ///Start timestamp
    pub start_timestamp: i64,
}
impl From<&PublicGetSpotFeedHistoryParamsSchema>
for PublicGetSpotFeedHistoryParamsSchema {
    fn from(value: &PublicGetSpotFeedHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetSpotFeedHistoryRequestSchema
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
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "method": {
      "title": "method",
      "type": "string",
      "const": "public/get_spot_feed_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetSpotFeedHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetSpotFeedHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetSpotFeedHistoryRequestSchemaId>,
    pub method: String,
    pub params: PublicGetSpotFeedHistoryParamsSchema,
}
impl From<&PublicGetSpotFeedHistoryRequestSchema>
for PublicGetSpotFeedHistoryRequestSchema {
    fn from(value: &PublicGetSpotFeedHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetSpotFeedHistoryRequestSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PublicGetSpotFeedHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetSpotFeedHistoryRequestSchemaId>
for PublicGetSpotFeedHistoryRequestSchemaId {
    fn from(value: &PublicGetSpotFeedHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetSpotFeedHistoryRequestSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PublicGetSpotFeedHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetSpotFeedHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetSpotFeedHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetSpotFeedHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetSpotFeedHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetSpotFeedHistoryResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "id",
    "result"
  ],
  "properties": {
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "result": {
      "description": "",
      "type": "object",
      "$ref": "#/definitions/PublicGetSpotFeedHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetSpotFeedHistoryResponseSchema {
    pub id: PublicGetSpotFeedHistoryResponseSchemaId,
    ///
    pub result: PublicGetSpotFeedHistoryResultSchema,
}
impl From<&PublicGetSpotFeedHistoryResponseSchema>
for PublicGetSpotFeedHistoryResponseSchema {
    fn from(value: &PublicGetSpotFeedHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetSpotFeedHistoryResponseSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PublicGetSpotFeedHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetSpotFeedHistoryResponseSchemaId>
for PublicGetSpotFeedHistoryResponseSchemaId {
    fn from(value: &PublicGetSpotFeedHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetSpotFeedHistoryResponseSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PublicGetSpotFeedHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetSpotFeedHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetSpotFeedHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetSpotFeedHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetSpotFeedHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetSpotFeedHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "spot_feed_history"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    },
    "spot_feed_history": {
      "title": "spot_feed_history",
      "description": "Spot feed history",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/SpotFeedHistoryResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetSpotFeedHistoryResultSchema {
    ///Currency
    pub currency: String,
    ///Spot feed history
    pub spot_feed_history: Vec<SpotFeedHistoryResponseSchema>,
}
impl From<&PublicGetSpotFeedHistoryResultSchema>
for PublicGetSpotFeedHistoryResultSchema {
    fn from(value: &PublicGetSpotFeedHistoryResultSchema) -> Self {
        value.clone()
    }
}
///SpotFeedHistoryResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "price",
    "timestamp",
    "timestamp_bucket"
  ],
  "properties": {
    "price": {
      "title": "price",
      "description": "Spot price",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of when the spot price was recorded into the database",
      "type": "integer"
    },
    "timestamp_bucket": {
      "title": "timestamp_bucket",
      "description": "Timestamp bucket; this value is regularly spaced out with `period` seconds between data points, missing values are forward-filled from earlier data where possible, if no earlier data is available, values are back-filled from the first observed data point",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SpotFeedHistoryResponseSchema {
    ///Spot price
    pub price: bigdecimal::BigDecimal,
    ///Timestamp of when the spot price was recored into the database
    pub timestamp: i64,
    ///Timestamp bucket; this value is regularly spaced out with `period` seconds between data points, missing values are forward-filled from earlier data where possible, if no earlier data is available, values are back-filled from the first observed data point
    pub timestamp_bucket: i64,
}
impl From<&SpotFeedHistoryResponseSchema> for SpotFeedHistoryResponseSchema {
    fn from(value: &SpotFeedHistoryResponseSchema) -> Self {
        value.clone()
    }
}
