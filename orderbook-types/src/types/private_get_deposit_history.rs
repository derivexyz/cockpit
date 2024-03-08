#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///DepositSchema
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
    "transaction_id",
    "tx_hash",
    "tx_status"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount deposited by the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "asset": {
      "title": "asset",
      "description": "Asset deposited",
      "type": "string"
    },
    "error_log": {
      "title": "error_log",
      "description": "If failed, error log for reason",
      "type": [
        "object",
        "null"
      ],
      "additionalProperties": {}
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the deposit (in ms since UNIX epoch)",
      "type": "integer"
    },
    "transaction_id": {
      "title": "transaction_id",
      "description": "Transaction ID",
      "type": "string",
      "format": "uuid"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Hash of the transaction that deposited the funds",
      "type": "string"
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Status of the transaction that deposited the funds",
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

pub struct DepositSchema {
    ///Amount deposited by the subaccount
    pub amount: bigdecimal::BigDecimal,
    ///Asset deposited
    pub asset: String,
    ///If failed, error log for reason
    pub error_log: Option<serde_json::Map<String, serde_json::Value>>,
    ///Timestamp of the deposit (in ms since UNIX epoch)
    pub timestamp: i64,
    ///Transaction ID
    pub transaction_id: uuid::Uuid,
    ///Hash of the transaction that deposited the funds
    pub tx_hash: String,
    ///Status of the transaction that deposited the funds
    pub tx_status: TxStatus,
}
impl From<&DepositSchema> for DepositSchema {
    fn from(value: &DepositSchema) -> Self {
        value.clone()
    }
}
///Get subaccount deposit history.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_deposit_history",
  "description": "Get subaccount deposit history.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetDepositHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetDepositHistory(pub PrivateGetDepositHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetDepositHistory {
    type Target = PrivateGetDepositHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetDepositHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetDepositHistory> for PrivateGetDepositHistoryJsonrpcSchema {
    fn from(value: PrivateGetDepositHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetDepositHistory> for PrivateGetDepositHistory {
    fn from(value: &PrivateGetDepositHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetDepositHistoryJsonrpcSchema> for PrivateGetDepositHistory {
    fn from(value: PrivateGetDepositHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetDepositHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetDepositHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetDepositHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetDepositHistoryJsonrpcSchema {
    pub request: PrivateGetDepositHistoryRequestSchema,
    pub response: PrivateGetDepositHistoryResponseSchema,
}
impl From<&PrivateGetDepositHistoryJsonrpcSchema>
for PrivateGetDepositHistoryJsonrpcSchema {
    fn from(value: &PrivateGetDepositHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetDepositHistoryParamsSchema
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

pub struct PrivateGetDepositHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetDepositHistoryParamsSchema>
for PrivateGetDepositHistoryParamsSchema {
    fn from(value: &PrivateGetDepositHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetDepositHistoryRequestSchema
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
      "const": "private/get_deposit_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetDepositHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetDepositHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetDepositHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetDepositHistoryParamsSchema,
}
impl From<&PrivateGetDepositHistoryRequestSchema>
for PrivateGetDepositHistoryRequestSchema {
    fn from(value: &PrivateGetDepositHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetDepositHistoryRequestSchemaId
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
pub enum PrivateGetDepositHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetDepositHistoryRequestSchemaId>
for PrivateGetDepositHistoryRequestSchemaId {
    fn from(value: &PrivateGetDepositHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetDepositHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetDepositHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetDepositHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetDepositHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetDepositHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetDepositHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetDepositHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetDepositHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetDepositHistoryResponseSchema {
    pub id: PrivateGetDepositHistoryResponseSchemaId,
    ///
    pub result: PrivateGetDepositHistoryResultSchema,
}
impl From<&PrivateGetDepositHistoryResponseSchema>
for PrivateGetDepositHistoryResponseSchema {
    fn from(value: &PrivateGetDepositHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetDepositHistoryResponseSchemaId
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
pub enum PrivateGetDepositHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetDepositHistoryResponseSchemaId>
for PrivateGetDepositHistoryResponseSchemaId {
    fn from(value: &PrivateGetDepositHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetDepositHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetDepositHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetDepositHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetDepositHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetDepositHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetDepositHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetDepositHistoryResultSchema
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
      "description": "List of deposit payments",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/DepositSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetDepositHistoryResultSchema {
    ///List of deposit payments
    pub events: Vec<DepositSchema>,
}
impl From<&PrivateGetDepositHistoryResultSchema>
for PrivateGetDepositHistoryResultSchema {
    fn from(value: &PrivateGetDepositHistoryResultSchema) -> Self {
        value.clone()
    }
}
///Status of the transaction that deposited the funds
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Status of the transaction that deposited the funds",
  "type": "string",
  "enum": [
    "requested",
    "pending",
    "settled",
    "reverted",
    "ignored"
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
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
    #[serde(rename = "ignored")]
    Ignored,
}
impl From<&TxStatus> for TxStatus {
    fn from(value: &TxStatus) -> Self {
        value.clone()
    }
}
impl ToString for TxStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Requested => "requested".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
            Self::Ignored => "ignored".to_string(),
        }
    }
}
impl std::str::FromStr for TxStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "requested" => Ok(Self::Requested),
            "pending" => Ok(Self::Pending),
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            "ignored" => Ok(Self::Ignored),
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
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
