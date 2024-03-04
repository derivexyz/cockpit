#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///InterestPaymentSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "interest",
    "timestamp"
  ],
  "properties": {
    "interest": {
      "title": "interest",
      "description": "Dollar interest paid (if negative) or received (if positive) by the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the interest payment (in ms since UNIX epoch)",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct InterestPaymentSchema {
    ///Dollar interest paid (if negative) or received (if positive) by the subaccount
    pub interest: bigdecimal::BigDecimal,
    ///Timestamp of the interest payment (in ms since UNIX epoch)
    pub timestamp: i64,
}
impl From<&InterestPaymentSchema> for InterestPaymentSchema {
    fn from(value: &InterestPaymentSchema) -> Self {
        value.clone()
    }
}
///Get subaccount interest payment history.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_interest_history",
  "description": "Get subaccount interest payment history.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetInterestHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetInterestHistory(pub PrivateGetInterestHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetInterestHistory {
    type Target = PrivateGetInterestHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetInterestHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetInterestHistory> for PrivateGetInterestHistoryJsonrpcSchema {
    fn from(value: PrivateGetInterestHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetInterestHistory> for PrivateGetInterestHistory {
    fn from(value: &PrivateGetInterestHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetInterestHistoryJsonrpcSchema> for PrivateGetInterestHistory {
    fn from(value: PrivateGetInterestHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetInterestHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetInterestHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetInterestHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetInterestHistoryJsonrpcSchema {
    pub request: PrivateGetInterestHistoryRequestSchema,
    pub response: PrivateGetInterestHistoryResponseSchema,
}
impl From<&PrivateGetInterestHistoryJsonrpcSchema>
for PrivateGetInterestHistoryJsonrpcSchema {
    fn from(value: &PrivateGetInterestHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetInterestHistoryParamsSchema
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

pub struct PrivateGetInterestHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetInterestHistoryParamsSchema>
for PrivateGetInterestHistoryParamsSchema {
    fn from(value: &PrivateGetInterestHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetInterestHistoryRequestSchema
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
      "const": "private/get_interest_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetInterestHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetInterestHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetInterestHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetInterestHistoryParamsSchema,
}
impl From<&PrivateGetInterestHistoryRequestSchema>
for PrivateGetInterestHistoryRequestSchema {
    fn from(value: &PrivateGetInterestHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetInterestHistoryRequestSchemaId
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
pub enum PrivateGetInterestHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetInterestHistoryRequestSchemaId>
for PrivateGetInterestHistoryRequestSchemaId {
    fn from(value: &PrivateGetInterestHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetInterestHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetInterestHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetInterestHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetInterestHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetInterestHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetInterestHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetInterestHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetInterestHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetInterestHistoryResponseSchema {
    pub id: PrivateGetInterestHistoryResponseSchemaId,
    ///
    pub result: PrivateGetInterestHistoryResultSchema,
}
impl From<&PrivateGetInterestHistoryResponseSchema>
for PrivateGetInterestHistoryResponseSchema {
    fn from(value: &PrivateGetInterestHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetInterestHistoryResponseSchemaId
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
pub enum PrivateGetInterestHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetInterestHistoryResponseSchemaId>
for PrivateGetInterestHistoryResponseSchemaId {
    fn from(value: &PrivateGetInterestHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetInterestHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetInterestHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetInterestHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetInterestHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetInterestHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetInterestHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetInterestHistoryResultSchema
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
      "description": "List of interest payments",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/InterestPaymentSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetInterestHistoryResultSchema {
    ///List of interest payments
    pub events: Vec<InterestPaymentSchema>,
}
impl From<&PrivateGetInterestHistoryResultSchema>
for PrivateGetInterestHistoryResultSchema {
    fn from(value: &PrivateGetInterestHistoryResultSchema) -> Self {
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
