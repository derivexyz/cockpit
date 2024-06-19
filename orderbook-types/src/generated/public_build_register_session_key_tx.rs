#![allow(unused_variables)]
#![allow(unused_imports)]
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;
///Build a signable transaction params dictionary.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/build_register_session_key_tx",
  "description": "Build a signable transaction params dictionary.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicBuildRegisterSessionKeyTxJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicBuildRegisterSessionKeyTx(pub PublicBuildRegisterSessionKeyTxJsonrpcSchema);
impl std::ops::Deref for PublicBuildRegisterSessionKeyTx {
    type Target = PublicBuildRegisterSessionKeyTxJsonrpcSchema;
    fn deref(&self) -> &PublicBuildRegisterSessionKeyTxJsonrpcSchema {
        &self.0
    }
}
impl From<PublicBuildRegisterSessionKeyTx> for PublicBuildRegisterSessionKeyTxJsonrpcSchema {
    fn from(value: PublicBuildRegisterSessionKeyTx) -> Self {
        value.0
    }
}
impl From<&PublicBuildRegisterSessionKeyTx> for PublicBuildRegisterSessionKeyTx {
    fn from(value: &PublicBuildRegisterSessionKeyTx) -> Self {
        value.clone()
    }
}
impl From<PublicBuildRegisterSessionKeyTxJsonrpcSchema> for PublicBuildRegisterSessionKeyTx {
    fn from(value: PublicBuildRegisterSessionKeyTxJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicBuildRegisterSessionKeyTxJsonrpcSchema
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
      "$ref": "#/definitions/PublicBuildRegisterSessionKeyTxRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicBuildRegisterSessionKeyTxResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicBuildRegisterSessionKeyTxJsonrpcSchema {
    pub request: PublicBuildRegisterSessionKeyTxRequestSchema,
    pub response: PublicBuildRegisterSessionKeyTxResponseSchema,
}
impl From<&PublicBuildRegisterSessionKeyTxJsonrpcSchema>
    for PublicBuildRegisterSessionKeyTxJsonrpcSchema
{
    fn from(value: &PublicBuildRegisterSessionKeyTxJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicBuildRegisterSessionKeyTxParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "expiry_sec",
    "gas",
    "nonce",
    "public_session_key",
    "wallet"
  ],
  "properties": {
    "expiry_sec": {
      "title": "expiry_sec",
      "description": "Expiry of the session key",
      "type": "integer"
    },
    "gas": {
      "title": "gas",
      "description": "Gas allowance for transaction. If none, will use estimateGas * 150%",
      "type": [
        "integer",
        "null"
      ]
    },
    "nonce": {
      "title": "nonce",
      "description": "Wallet's transaction count, If none, will use eth.getTransactionCount()",
      "type": [
        "integer",
        "null"
      ]
    },
    "public_session_key": {
      "title": "public_session_key",
      "description": "Session key in the form of an Ethereum EOA",
      "type": "string"
    },
    "wallet": {
      "title": "wallet",
      "description": "Ethereum wallet address of account",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicBuildRegisterSessionKeyTxParamsSchema {
    ///Expiry of the session key
    pub expiry_sec: i64,
    ///Gas allowance for transaction. If none, will use estimateGas * 150%
    pub gas: Option<i64>,
    ///Wallet's transaction count, If none, will use eth.getTransactionCount()
    pub nonce: Option<i64>,
    ///Session key in the form of an Ethereum EOA
    pub public_session_key: String,
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PublicBuildRegisterSessionKeyTxParamsSchema>
    for PublicBuildRegisterSessionKeyTxParamsSchema
{
    fn from(value: &PublicBuildRegisterSessionKeyTxParamsSchema) -> Self {
        value.clone()
    }
}
///PublicBuildRegisterSessionKeyTxRequestSchema
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
      "const": "public/build_register_session_key_tx"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicBuildRegisterSessionKeyTxParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicBuildRegisterSessionKeyTxRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicBuildRegisterSessionKeyTxRequestSchemaId>,
    pub method: String,
    pub params: PublicBuildRegisterSessionKeyTxParamsSchema,
}
impl From<&PublicBuildRegisterSessionKeyTxRequestSchema>
    for PublicBuildRegisterSessionKeyTxRequestSchema
{
    fn from(value: &PublicBuildRegisterSessionKeyTxRequestSchema) -> Self {
        value.clone()
    }
}
///PublicBuildRegisterSessionKeyTxRequestSchemaId
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
pub enum PublicBuildRegisterSessionKeyTxRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicBuildRegisterSessionKeyTxRequestSchemaId>
    for PublicBuildRegisterSessionKeyTxRequestSchemaId
{
    fn from(value: &PublicBuildRegisterSessionKeyTxRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicBuildRegisterSessionKeyTxRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicBuildRegisterSessionKeyTxRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicBuildRegisterSessionKeyTxRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicBuildRegisterSessionKeyTxRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicBuildRegisterSessionKeyTxRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicBuildRegisterSessionKeyTxRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicBuildRegisterSessionKeyTxResponseSchema
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
      "$ref": "#/definitions/PublicBuildRegisterSessionKeyTxResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicBuildRegisterSessionKeyTxResponseSchema {
    pub id: PublicBuildRegisterSessionKeyTxResponseSchemaId,
    ///
    pub result: PublicBuildRegisterSessionKeyTxResultSchema,
}
impl From<&PublicBuildRegisterSessionKeyTxResponseSchema>
    for PublicBuildRegisterSessionKeyTxResponseSchema
{
    fn from(value: &PublicBuildRegisterSessionKeyTxResponseSchema) -> Self {
        value.clone()
    }
}
///PublicBuildRegisterSessionKeyTxResponseSchemaId
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
pub enum PublicBuildRegisterSessionKeyTxResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicBuildRegisterSessionKeyTxResponseSchemaId>
    for PublicBuildRegisterSessionKeyTxResponseSchemaId
{
    fn from(value: &PublicBuildRegisterSessionKeyTxResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicBuildRegisterSessionKeyTxResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicBuildRegisterSessionKeyTxResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicBuildRegisterSessionKeyTxResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicBuildRegisterSessionKeyTxResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicBuildRegisterSessionKeyTxResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicBuildRegisterSessionKeyTxResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicBuildRegisterSessionKeyTxResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "tx_params"
  ],
  "properties": {
    "tx_params": {
      "title": "tx_params",
      "description": "Transaction params in dictionary form, same as `TxParams` in `actions.py`",
      "type": "object",
      "additionalProperties": {}
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicBuildRegisterSessionKeyTxResultSchema {
    ///Transaction params in dictionary form, same as `TxParams` in `actions.py`
    pub tx_params: serde_json::Map<String, serde_json::Value>,
}
impl From<&PublicBuildRegisterSessionKeyTxResultSchema>
    for PublicBuildRegisterSessionKeyTxResultSchema
{
    fn from(value: &PublicBuildRegisterSessionKeyTxResultSchema) -> Self {
        value.clone()
    }
}
