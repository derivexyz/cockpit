#![allow(unused_variables)]
#![allow(unused_imports)]
use bigdecimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid;
///Used for getting a transaction by its transaction id
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_transaction",
  "description": "Used for getting a transaction by its transaction id",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetTransactionJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetTransaction(pub PublicGetTransactionJsonrpcSchema);
impl std::ops::Deref for PublicGetTransaction {
    type Target = PublicGetTransactionJsonrpcSchema;
    fn deref(&self) -> &PublicGetTransactionJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetTransaction> for PublicGetTransactionJsonrpcSchema {
    fn from(value: PublicGetTransaction) -> Self {
        value.0
    }
}
impl From<&PublicGetTransaction> for PublicGetTransaction {
    fn from(value: &PublicGetTransaction) -> Self {
        value.clone()
    }
}
impl From<PublicGetTransactionJsonrpcSchema> for PublicGetTransaction {
    fn from(value: PublicGetTransactionJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetTransactionJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetTransactionRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTransactionResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTransactionJsonrpcSchema {
    pub request: PublicGetTransactionRequestSchema,
    pub response: PublicGetTransactionResponseSchema,
}
impl From<&PublicGetTransactionJsonrpcSchema> for PublicGetTransactionJsonrpcSchema {
    fn from(value: &PublicGetTransactionJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetTransactionParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "transaction_id"
  ],
  "properties": {
    "transaction_id": {
      "title": "transaction_id",
      "description": "transaction_id of the transaction to get",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTransactionParamsSchema {
    ///transaction_id of the transaction to get
    pub transaction_id: uuid::Uuid,
}
impl From<&PublicGetTransactionParamsSchema> for PublicGetTransactionParamsSchema {
    fn from(value: &PublicGetTransactionParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetTransactionRequestSchema
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
      "const": "public/get_transaction"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTransactionParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTransactionRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetTransactionRequestSchemaId>,
    pub method: String,
    pub params: PublicGetTransactionParamsSchema,
}
impl From<&PublicGetTransactionRequestSchema> for PublicGetTransactionRequestSchema {
    fn from(value: &PublicGetTransactionRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetTransactionRequestSchemaId
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
pub enum PublicGetTransactionRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTransactionRequestSchemaId> for PublicGetTransactionRequestSchemaId {
    fn from(value: &PublicGetTransactionRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTransactionRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTransactionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTransactionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTransactionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTransactionRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTransactionRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetTransactionResponseSchema
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
      "$ref": "#/definitions/PublicGetTransactionResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTransactionResponseSchema {
    pub id: PublicGetTransactionResponseSchemaId,
    ///
    pub result: PublicGetTransactionResultSchema,
}
impl From<&PublicGetTransactionResponseSchema> for PublicGetTransactionResponseSchema {
    fn from(value: &PublicGetTransactionResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetTransactionResponseSchemaId
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
pub enum PublicGetTransactionResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTransactionResponseSchemaId> for PublicGetTransactionResponseSchemaId {
    fn from(value: &PublicGetTransactionResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTransactionResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTransactionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTransactionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTransactionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTransactionResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTransactionResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetTransactionResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "data",
    "error_log",
    "status",
    "transaction_hash"
  ],
  "properties": {
    "data": {
      "title": "data",
      "description": "Data used to create transaction",
      "type": "string"
    },
    "error_log": {
      "title": "error_log",
      "description": "Error log if failed tx",
      "type": [
        "string",
        "null"
      ]
    },
    "status": {
      "title": "status",
      "description": "Status of the transaction",
      "type": "string",
      "enum": [
        "requested",
        "pending",
        "settled",
        "reverted",
        "ignored"
      ]
    },
    "transaction_hash": {
      "title": "transaction_hash",
      "description": "Transaction hash of a pending tx",
      "type": [
        "string",
        "null"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTransactionResultSchema {
    ///Data used to create transaction
    pub data: Value,
    ///Error log if failed tx
    pub error_log: Option<Value>,
    ///Status of the transaction
    pub status: Status,
    ///Transaction hash of a pending tx
    pub transaction_hash: Option<String>,
}
impl From<&PublicGetTransactionResultSchema> for PublicGetTransactionResultSchema {
    fn from(value: &PublicGetTransactionResultSchema) -> Self {
        value.clone()
    }
}
///Status of the transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "Status of the transaction",
  "type": "string",
  "enum": [
    "requested",
    "pending",
    "settled",
    "reverted",
    "ignored"
    "timed_out"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Status {
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
    #[serde(rename = "timed_out")]
    TimedOut,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match *self {
            Self::Requested => "requested".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
            Self::Ignored => "ignored".to_string(),
            Self::TimedOut => "timed_out".to_string(),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "requested" => Ok(Self::Requested),
            "pending" => Ok(Self::Pending),
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            "ignored" => Ok(Self::Ignored),
            "timed_out" => Ok(Self::TimedOut),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Status {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Status {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Status {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
