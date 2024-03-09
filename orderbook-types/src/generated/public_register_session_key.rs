#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Register or update expiry of an existing session key.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/register_session_key",
  "description": "Register or update expiry of an existing session key.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicRegisterSessionKeyJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicRegisterSessionKey(pub PublicRegisterSessionKeyJsonrpcSchema);
impl std::ops::Deref for PublicRegisterSessionKey {
    type Target = PublicRegisterSessionKeyJsonrpcSchema;
    fn deref(&self) -> &PublicRegisterSessionKeyJsonrpcSchema {
        &self.0
    }
}
impl From<PublicRegisterSessionKey> for PublicRegisterSessionKeyJsonrpcSchema {
    fn from(value: PublicRegisterSessionKey) -> Self {
        value.0
    }
}
impl From<&PublicRegisterSessionKey> for PublicRegisterSessionKey {
    fn from(value: &PublicRegisterSessionKey) -> Self {
        value.clone()
    }
}
impl From<PublicRegisterSessionKeyJsonrpcSchema> for PublicRegisterSessionKey {
    fn from(value: PublicRegisterSessionKeyJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicRegisterSessionKeyJsonrpcSchema
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
      "$ref": "#/definitions/PublicRegisterSessionKeyRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicRegisterSessionKeyResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicRegisterSessionKeyJsonrpcSchema {
    pub request: PublicRegisterSessionKeyRequestSchema,
    pub response: PublicRegisterSessionKeyResponseSchema,
}
impl From<&PublicRegisterSessionKeyJsonrpcSchema>
for PublicRegisterSessionKeyJsonrpcSchema {
    fn from(value: &PublicRegisterSessionKeyJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicRegisterSessionKeyParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "expiry_sec",
    "label",
    "public_session_key",
    "signed_raw_tx",
    "wallet"
  ],
  "properties": {
    "expiry_sec": {
      "title": "expiry_sec",
      "description": "Expiry of the session key",
      "type": "integer"
    },
    "label": {
      "title": "label",
      "description": "Ethereum wallet address",
      "type": "string"
    },
    "public_session_key": {
      "title": "public_session_key",
      "description": "Session key in the form of an Ethereum EOA",
      "type": "string"
    },
    "signed_raw_tx": {
      "title": "signed_raw_tx",
      "description": "A signed RLP encoded ETH transaction in form of a hex string (same as `w3.eth.account.sign_transaction(unsigned_tx, private_key).rawTransaction.hex()`)",
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

pub struct PublicRegisterSessionKeyParamsSchema {
    ///Expiry of the session key
    pub expiry_sec: i64,
    ///Ethereum wallet address
    pub label: String,
    ///Session key in the form of an Ethereum EOA
    pub public_session_key: String,
    ///A signed RLP encoded ETH transaction in form of a hex string (same as `w3.eth.account.sign_transaction(unsigned_tx, private_key).rawTransaction.hex()`)
    pub signed_raw_tx: String,
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PublicRegisterSessionKeyParamsSchema>
for PublicRegisterSessionKeyParamsSchema {
    fn from(value: &PublicRegisterSessionKeyParamsSchema) -> Self {
        value.clone()
    }
}
///PublicRegisterSessionKeyRequestSchema
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
      "const": "public/register_session_key"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicRegisterSessionKeyParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicRegisterSessionKeyRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicRegisterSessionKeyRequestSchemaId>,
    pub method: String,
    pub params: PublicRegisterSessionKeyParamsSchema,
}
impl From<&PublicRegisterSessionKeyRequestSchema>
for PublicRegisterSessionKeyRequestSchema {
    fn from(value: &PublicRegisterSessionKeyRequestSchema) -> Self {
        value.clone()
    }
}
///PublicRegisterSessionKeyRequestSchemaId
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
pub enum PublicRegisterSessionKeyRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicRegisterSessionKeyRequestSchemaId>
for PublicRegisterSessionKeyRequestSchemaId {
    fn from(value: &PublicRegisterSessionKeyRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicRegisterSessionKeyRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicRegisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicRegisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicRegisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicRegisterSessionKeyRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicRegisterSessionKeyRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicRegisterSessionKeyResponseSchema
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
      "$ref": "#/definitions/PublicRegisterSessionKeyResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicRegisterSessionKeyResponseSchema {
    pub id: PublicRegisterSessionKeyResponseSchemaId,
    ///
    pub result: PublicRegisterSessionKeyResultSchema,
}
impl From<&PublicRegisterSessionKeyResponseSchema>
for PublicRegisterSessionKeyResponseSchema {
    fn from(value: &PublicRegisterSessionKeyResponseSchema) -> Self {
        value.clone()
    }
}
///PublicRegisterSessionKeyResponseSchemaId
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
pub enum PublicRegisterSessionKeyResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicRegisterSessionKeyResponseSchemaId>
for PublicRegisterSessionKeyResponseSchemaId {
    fn from(value: &PublicRegisterSessionKeyResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicRegisterSessionKeyResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicRegisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicRegisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicRegisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicRegisterSessionKeyResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicRegisterSessionKeyResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicRegisterSessionKeyResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label",
    "public_session_key",
    "transaction_id"
  ],
  "properties": {
    "label": {
      "title": "label",
      "description": "User-defined session key label",
      "type": "string"
    },
    "public_session_key": {
      "title": "public_session_key",
      "description": "Session key in the form of an Ethereum EOA",
      "type": "string"
    },
    "transaction_id": {
      "title": "transaction_id",
      "description": "ID to lookup status of transaction",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicRegisterSessionKeyResultSchema {
    ///User-defined session key label
    pub label: String,
    ///Session key in the form of an Ethereum EOA
    pub public_session_key: String,
    ///ID to lookup status of transaction
    pub transaction_id: uuid::Uuid,
}
impl From<&PublicRegisterSessionKeyResultSchema>
for PublicRegisterSessionKeyResultSchema {
    fn from(value: &PublicRegisterSessionKeyResultSchema) -> Self {
        value.clone()
    }
}
