#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Erc20TransferSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset",
    "counterparty_subaccount_id",
    "is_outgoing",
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
    "counterparty_subaccount_id": {
      "title": "counterparty_subaccount_id",
      "description": "Recipient or sender subaccount_id of transfer",
      "type": "integer"
    },
    "is_outgoing": {
      "title": "is_outgoing",
      "description": "True if the transfer was initiated by the subaccount, False otherwise",
      "type": "boolean"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the transfer (in ms since UNIX epoch)",
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

pub struct Erc20TransferSchema {
    ///Amount withdrawn by the subaccount
    pub amount: bigdecimal::BigDecimal,
    ///Asset withdrawn
    pub asset: String,
    ///Recipient or sender subaccount_id of transfer
    pub counterparty_subaccount_id: i64,
    ///True if the transfer was initiated by the subaccount, False otherwise
    pub is_outgoing: bool,
    ///Timestamp of the transfer (in ms since UNIX epoch)
    pub timestamp: i64,
    ///Hash of the transaction that withdrew the funds
    pub tx_hash: String,
}
impl From<&Erc20TransferSchema> for Erc20TransferSchema {
    fn from(value: &Erc20TransferSchema) -> Self {
        value.clone()
    }
}
/**Get subaccount erc20 transfer history.

Position transfers (e.g. options or perps) are treated as trades. Use `private/get_trade_history` for position transfer history.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_erc20_transfer_history",
  "description": "Get subaccount erc20 transfer history.\n\nPosition transfers (e.g. options or perps) are treated as trades. Use `private/get_trade_history` for position transfer history.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetErc20TransferHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetErc20TransferHistory(
    pub PrivateGetErc20TransferHistoryJsonrpcSchema,
);
impl std::ops::Deref for PrivateGetErc20TransferHistory {
    type Target = PrivateGetErc20TransferHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetErc20TransferHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetErc20TransferHistory>
for PrivateGetErc20TransferHistoryJsonrpcSchema {
    fn from(value: PrivateGetErc20TransferHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetErc20TransferHistory> for PrivateGetErc20TransferHistory {
    fn from(value: &PrivateGetErc20TransferHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetErc20TransferHistoryJsonrpcSchema>
for PrivateGetErc20TransferHistory {
    fn from(value: PrivateGetErc20TransferHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetErc20TransferHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetErc20TransferHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetErc20TransferHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetErc20TransferHistoryJsonrpcSchema {
    pub request: PrivateGetErc20TransferHistoryRequestSchema,
    pub response: PrivateGetErc20TransferHistoryResponseSchema,
}
impl From<&PrivateGetErc20TransferHistoryJsonrpcSchema>
for PrivateGetErc20TransferHistoryJsonrpcSchema {
    fn from(value: &PrivateGetErc20TransferHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetErc20TransferHistoryParamsSchema
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

pub struct PrivateGetErc20TransferHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetErc20TransferHistoryParamsSchema>
for PrivateGetErc20TransferHistoryParamsSchema {
    fn from(value: &PrivateGetErc20TransferHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetErc20TransferHistoryRequestSchema
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
      "const": "private/get_erc20_transfer_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetErc20TransferHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetErc20TransferHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetErc20TransferHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetErc20TransferHistoryParamsSchema,
}
impl From<&PrivateGetErc20TransferHistoryRequestSchema>
for PrivateGetErc20TransferHistoryRequestSchema {
    fn from(value: &PrivateGetErc20TransferHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetErc20TransferHistoryRequestSchemaId
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
pub enum PrivateGetErc20TransferHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetErc20TransferHistoryRequestSchemaId>
for PrivateGetErc20TransferHistoryRequestSchemaId {
    fn from(value: &PrivateGetErc20TransferHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetErc20TransferHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetErc20TransferHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetErc20TransferHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetErc20TransferHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetErc20TransferHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetErc20TransferHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetErc20TransferHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetErc20TransferHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetErc20TransferHistoryResponseSchema {
    pub id: PrivateGetErc20TransferHistoryResponseSchemaId,
    ///
    pub result: PrivateGetErc20TransferHistoryResultSchema,
}
impl From<&PrivateGetErc20TransferHistoryResponseSchema>
for PrivateGetErc20TransferHistoryResponseSchema {
    fn from(value: &PrivateGetErc20TransferHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetErc20TransferHistoryResponseSchemaId
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
pub enum PrivateGetErc20TransferHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetErc20TransferHistoryResponseSchemaId>
for PrivateGetErc20TransferHistoryResponseSchemaId {
    fn from(value: &PrivateGetErc20TransferHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetErc20TransferHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetErc20TransferHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetErc20TransferHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetErc20TransferHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetErc20TransferHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetErc20TransferHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetErc20TransferHistoryResultSchema
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
      "description": "List of erc20 transfers",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/ERC20TransferSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetErc20TransferHistoryResultSchema {
    ///List of erc20 transfers
    pub events: Vec<Erc20TransferSchema>,
}
impl From<&PrivateGetErc20TransferHistoryResultSchema>
for PrivateGetErc20TransferHistoryResultSchema {
    fn from(value: &PrivateGetErc20TransferHistoryResultSchema) -> Self {
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
