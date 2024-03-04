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
  "title": "public/deregister_session_key",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicDeregisterSessionKeyJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicDeregisterSessionKey(pub PublicDeregisterSessionKeyJsonrpcSchema);
impl std::ops::Deref for PublicDeregisterSessionKey {
    type Target = PublicDeregisterSessionKeyJsonrpcSchema;
    fn deref(&self) -> &PublicDeregisterSessionKeyJsonrpcSchema {
        &self.0
    }
}
impl From<PublicDeregisterSessionKey> for PublicDeregisterSessionKeyJsonrpcSchema {
    fn from(value: PublicDeregisterSessionKey) -> Self {
        value.0
    }
}
impl From<&PublicDeregisterSessionKey> for PublicDeregisterSessionKey {
    fn from(value: &PublicDeregisterSessionKey) -> Self {
        value.clone()
    }
}
impl From<PublicDeregisterSessionKeyJsonrpcSchema> for PublicDeregisterSessionKey {
    fn from(value: PublicDeregisterSessionKeyJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicDeregisterSessionKeyJsonrpcSchema
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
      "$ref": "#/definitions/PublicDeregisterSessionKeyRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicDeregisterSessionKeyResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicDeregisterSessionKeyJsonrpcSchema {
    pub request: PublicDeregisterSessionKeyRequestSchema,
    pub response: PublicDeregisterSessionKeyResponseSchema,
}
impl From<&PublicDeregisterSessionKeyJsonrpcSchema>
for PublicDeregisterSessionKeyJsonrpcSchema {
    fn from(value: &PublicDeregisterSessionKeyJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicDeregisterSessionKeyParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "public_session_key",
    "signed_raw_tx",
    "wallet"
  ],
  "properties": {
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

pub struct PublicDeregisterSessionKeyParamsSchema {
    ///Session key in the form of an Ethereum EOA
    pub public_session_key: String,
    ///A signed RLP encoded ETH transaction in form of a hex string (same as `w3.eth.account.sign_transaction(unsigned_tx, private_key).rawTransaction.hex()`)
    pub signed_raw_tx: String,
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PublicDeregisterSessionKeyParamsSchema>
for PublicDeregisterSessionKeyParamsSchema {
    fn from(value: &PublicDeregisterSessionKeyParamsSchema) -> Self {
        value.clone()
    }
}
///PublicDeregisterSessionKeyRequestSchema
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
      "const": "public/deregister_session_key"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicDeregisterSessionKeyParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicDeregisterSessionKeyRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicDeregisterSessionKeyRequestSchemaId>,
    pub method: String,
    pub params: PublicDeregisterSessionKeyParamsSchema,
}
impl From<&PublicDeregisterSessionKeyRequestSchema>
for PublicDeregisterSessionKeyRequestSchema {
    fn from(value: &PublicDeregisterSessionKeyRequestSchema) -> Self {
        value.clone()
    }
}
///PublicDeregisterSessionKeyRequestSchemaId
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
pub enum PublicDeregisterSessionKeyRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicDeregisterSessionKeyRequestSchemaId>
for PublicDeregisterSessionKeyRequestSchemaId {
    fn from(value: &PublicDeregisterSessionKeyRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicDeregisterSessionKeyRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicDeregisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicDeregisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicDeregisterSessionKeyRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicDeregisterSessionKeyRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicDeregisterSessionKeyRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicDeregisterSessionKeyResponseSchema
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
      "$ref": "#/definitions/PublicDeregisterSessionKeyResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicDeregisterSessionKeyResponseSchema {
    pub id: PublicDeregisterSessionKeyResponseSchemaId,
    ///
    pub result: PublicDeregisterSessionKeyResultSchema,
}
impl From<&PublicDeregisterSessionKeyResponseSchema>
for PublicDeregisterSessionKeyResponseSchema {
    fn from(value: &PublicDeregisterSessionKeyResponseSchema) -> Self {
        value.clone()
    }
}
///PublicDeregisterSessionKeyResponseSchemaId
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
pub enum PublicDeregisterSessionKeyResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicDeregisterSessionKeyResponseSchemaId>
for PublicDeregisterSessionKeyResponseSchemaId {
    fn from(value: &PublicDeregisterSessionKeyResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicDeregisterSessionKeyResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicDeregisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicDeregisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicDeregisterSessionKeyResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicDeregisterSessionKeyResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicDeregisterSessionKeyResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicDeregisterSessionKeyResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "public_session_key",
    "transaction_id"
  ],
  "properties": {
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

pub struct PublicDeregisterSessionKeyResultSchema {
    ///Session key in the form of an Ethereum EOA
    pub public_session_key: String,
    ///ID to lookup status of transaction
    pub transaction_id: uuid::Uuid,
}
impl From<&PublicDeregisterSessionKeyResultSchema>
for PublicDeregisterSessionKeyResultSchema {
    fn from(value: &PublicDeregisterSessionKeyResultSchema) -> Self {
        value.clone()
    }
}
