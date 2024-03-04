#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get subaccount withdrawal history.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_withdrawal_history",
  "description": "Get subaccount withdrawal history.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetWithdrawalHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetWithdrawalHistory(pub PrivateGetWithdrawalHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetWithdrawalHistory {
    type Target = PrivateGetWithdrawalHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetWithdrawalHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetWithdrawalHistory> for PrivateGetWithdrawalHistoryJsonrpcSchema {
    fn from(value: PrivateGetWithdrawalHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetWithdrawalHistory> for PrivateGetWithdrawalHistory {
    fn from(value: &PrivateGetWithdrawalHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetWithdrawalHistoryJsonrpcSchema> for PrivateGetWithdrawalHistory {
    fn from(value: PrivateGetWithdrawalHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetWithdrawalHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetWithdrawalHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetWithdrawalHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetWithdrawalHistoryJsonrpcSchema {
    pub request: PrivateGetWithdrawalHistoryRequestSchema,
    pub response: PrivateGetWithdrawalHistoryResponseSchema,
}
impl From<&PrivateGetWithdrawalHistoryJsonrpcSchema>
for PrivateGetWithdrawalHistoryJsonrpcSchema {
    fn from(value: &PrivateGetWithdrawalHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetWithdrawalHistoryParamsSchema
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

pub struct PrivateGetWithdrawalHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetWithdrawalHistoryParamsSchema>
for PrivateGetWithdrawalHistoryParamsSchema {
    fn from(value: &PrivateGetWithdrawalHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetWithdrawalHistoryRequestSchema
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
      "const": "private/get_withdrawal_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetWithdrawalHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetWithdrawalHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetWithdrawalHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetWithdrawalHistoryParamsSchema,
}
impl From<&PrivateGetWithdrawalHistoryRequestSchema>
for PrivateGetWithdrawalHistoryRequestSchema {
    fn from(value: &PrivateGetWithdrawalHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetWithdrawalHistoryRequestSchemaId
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
pub enum PrivateGetWithdrawalHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetWithdrawalHistoryRequestSchemaId>
for PrivateGetWithdrawalHistoryRequestSchemaId {
    fn from(value: &PrivateGetWithdrawalHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetWithdrawalHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetWithdrawalHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetWithdrawalHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetWithdrawalHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetWithdrawalHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetWithdrawalHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetWithdrawalHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetWithdrawalHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetWithdrawalHistoryResponseSchema {
    pub id: PrivateGetWithdrawalHistoryResponseSchemaId,
    ///
    pub result: PrivateGetWithdrawalHistoryResultSchema,
}
impl From<&PrivateGetWithdrawalHistoryResponseSchema>
for PrivateGetWithdrawalHistoryResponseSchema {
    fn from(value: &PrivateGetWithdrawalHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetWithdrawalHistoryResponseSchemaId
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
pub enum PrivateGetWithdrawalHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetWithdrawalHistoryResponseSchemaId>
for PrivateGetWithdrawalHistoryResponseSchemaId {
    fn from(value: &PrivateGetWithdrawalHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetWithdrawalHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetWithdrawalHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetWithdrawalHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetWithdrawalHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetWithdrawalHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetWithdrawalHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetWithdrawalHistoryResultSchema
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
      "description": "List of withdrawals",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/WithdrawalSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetWithdrawalHistoryResultSchema {
    ///List of withdrawals
    pub events: Vec<WithdrawalSchema>,
}
impl From<&PrivateGetWithdrawalHistoryResultSchema>
for PrivateGetWithdrawalHistoryResultSchema {
    fn from(value: &PrivateGetWithdrawalHistoryResultSchema) -> Self {
        value.clone()
    }
}
///WithdrawalSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset",
    "error_log",
    "timestamp",
    "tx_hash"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount withdrawn by the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "asset": {
      "title": "asset",
      "description": "Asset withdrawn",
      "type": "string"
    },
    "error_log": {
      "title": "error_log",
      "description": "If failed, error log for reason",
      "default": null,
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": {}
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the withdrawal (in ms since UNIX epoch)",
      "type": "integer"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Hash of the transaction that withdrew the funds",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct WithdrawalSchema {
    ///Amount withdrawn by the subaccount
    pub amount: bigdecimal::BigDecimal,
    ///Asset withdrawn
    pub asset: String,
    ///If failed, error log for reason
    pub error_log: Option<serde_json::Map<String, serde_json::Value>>,
    ///Timestamp of the withdrawal (in ms since UNIX epoch)
    pub timestamp: i64,
    ///Hash of the transaction that withdrew the funds
    pub tx_hash: String,
}
impl From<&WithdrawalSchema> for WithdrawalSchema {
    fn from(value: &WithdrawalSchema) -> Self {
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
