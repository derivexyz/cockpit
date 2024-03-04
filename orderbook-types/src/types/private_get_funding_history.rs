#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///FundingPaymentSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "funding",
    "instrument_name",
    "timestamp"
  ],
  "properties": {
    "funding": {
      "title": "funding",
      "description": "Dollar funding paid (if negative) or received (if positive) by the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the funding payment (in ms since UNIX epoch)",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct FundingPaymentSchema {
    ///Dollar funding paid (if negative) or received (if positive) by the subaccount
    pub funding: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Timestamp of the funding payment (in ms since UNIX epoch)
    pub timestamp: i64,
}
impl From<&FundingPaymentSchema> for FundingPaymentSchema {
    fn from(value: &FundingPaymentSchema) -> Self {
        value.clone()
    }
}
/**Get subaccount funding history.

DB: read replica*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_funding_history",
  "description": "Get subaccount funding history.\n\nDB: read replica",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetFundingHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetFundingHistory(pub PrivateGetFundingHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetFundingHistory {
    type Target = PrivateGetFundingHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetFundingHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetFundingHistory> for PrivateGetFundingHistoryJsonrpcSchema {
    fn from(value: PrivateGetFundingHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetFundingHistory> for PrivateGetFundingHistory {
    fn from(value: &PrivateGetFundingHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetFundingHistoryJsonrpcSchema> for PrivateGetFundingHistory {
    fn from(value: PrivateGetFundingHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetFundingHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetFundingHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetFundingHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetFundingHistoryJsonrpcSchema {
    pub request: PrivateGetFundingHistoryRequestSchema,
    pub response: PrivateGetFundingHistoryResponseSchema,
}
impl From<&PrivateGetFundingHistoryJsonrpcSchema>
for PrivateGetFundingHistoryJsonrpcSchema {
    fn from(value: &PrivateGetFundingHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetFundingHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id"
  ],
  "properties": {
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "End timestamp of the event history (default current time)",
      "default": 9223372036854775807,
      "type": "integer"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name (returns history for all perpetuals if not provided)",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Start timestamp of the event history (default 0)",
      "default": 0,
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetFundingHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Instrument name (returns history for all perpetuals if not provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetFundingHistoryParamsSchema>
for PrivateGetFundingHistoryParamsSchema {
    fn from(value: &PrivateGetFundingHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetFundingHistoryRequestSchema
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
      "const": "private/get_funding_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetFundingHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetFundingHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetFundingHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetFundingHistoryParamsSchema,
}
impl From<&PrivateGetFundingHistoryRequestSchema>
for PrivateGetFundingHistoryRequestSchema {
    fn from(value: &PrivateGetFundingHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetFundingHistoryRequestSchemaId
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
pub enum PrivateGetFundingHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetFundingHistoryRequestSchemaId>
for PrivateGetFundingHistoryRequestSchemaId {
    fn from(value: &PrivateGetFundingHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetFundingHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetFundingHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetFundingHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetFundingHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetFundingHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetFundingHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetFundingHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetFundingHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetFundingHistoryResponseSchema {
    pub id: PrivateGetFundingHistoryResponseSchemaId,
    ///
    pub result: PrivateGetFundingHistoryResultSchema,
}
impl From<&PrivateGetFundingHistoryResponseSchema>
for PrivateGetFundingHistoryResponseSchema {
    fn from(value: &PrivateGetFundingHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetFundingHistoryResponseSchemaId
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
pub enum PrivateGetFundingHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetFundingHistoryResponseSchemaId>
for PrivateGetFundingHistoryResponseSchemaId {
    fn from(value: &PrivateGetFundingHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetFundingHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetFundingHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetFundingHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetFundingHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetFundingHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetFundingHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetFundingHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "events"
  ],
  "properties": {
    "events": {
      "title": "events",
      "description": "List of funding payments",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/FundingPaymentSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetFundingHistoryResultSchema {
    ///List of funding payments
    pub events: Vec<FundingPaymentSchema>,
}
impl From<&PrivateGetFundingHistoryResultSchema>
for PrivateGetFundingHistoryResultSchema {
    fn from(value: &PrivateGetFundingHistoryResultSchema) -> Self {
        value.clone()
    }
}
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
