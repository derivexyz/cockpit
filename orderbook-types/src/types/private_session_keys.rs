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
  "title": "private/session_keys",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateSessionKeysJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateSessionKeys(pub PrivateSessionKeysJsonrpcSchema);
impl std::ops::Deref for PrivateSessionKeys {
    type Target = PrivateSessionKeysJsonrpcSchema;
    fn deref(&self) -> &PrivateSessionKeysJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateSessionKeys> for PrivateSessionKeysJsonrpcSchema {
    fn from(value: PrivateSessionKeys) -> Self {
        value.0
    }
}
impl From<&PrivateSessionKeys> for PrivateSessionKeys {
    fn from(value: &PrivateSessionKeys) -> Self {
        value.clone()
    }
}
impl From<PrivateSessionKeysJsonrpcSchema> for PrivateSessionKeys {
    fn from(value: PrivateSessionKeysJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateSessionKeysJsonrpcSchema
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
      "$ref": "#/definitions/PrivateSessionKeysRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateSessionKeysResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSessionKeysJsonrpcSchema {
    pub request: PrivateSessionKeysRequestSchema,
    pub response: PrivateSessionKeysResponseSchema,
}
impl From<&PrivateSessionKeysJsonrpcSchema> for PrivateSessionKeysJsonrpcSchema {
    fn from(value: &PrivateSessionKeysJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateSessionKeysParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "wallet"
  ],
  "properties": {
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

pub struct PrivateSessionKeysParamsSchema {
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PrivateSessionKeysParamsSchema> for PrivateSessionKeysParamsSchema {
    fn from(value: &PrivateSessionKeysParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateSessionKeysRequestSchema
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
      "const": "private/session_keys"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateSessionKeysParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSessionKeysRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateSessionKeysRequestSchemaId>,
    pub method: String,
    pub params: PrivateSessionKeysParamsSchema,
}
impl From<&PrivateSessionKeysRequestSchema> for PrivateSessionKeysRequestSchema {
    fn from(value: &PrivateSessionKeysRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateSessionKeysRequestSchemaId
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
pub enum PrivateSessionKeysRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSessionKeysRequestSchemaId> for PrivateSessionKeysRequestSchemaId {
    fn from(value: &PrivateSessionKeysRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSessionKeysRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSessionKeysRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSessionKeysRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSessionKeysRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSessionKeysRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSessionKeysRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSessionKeysResponseSchema
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
      "$ref": "#/definitions/PrivateSessionKeysResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSessionKeysResponseSchema {
    pub id: PrivateSessionKeysResponseSchemaId,
    ///
    pub result: PrivateSessionKeysResultSchema,
}
impl From<&PrivateSessionKeysResponseSchema> for PrivateSessionKeysResponseSchema {
    fn from(value: &PrivateSessionKeysResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateSessionKeysResponseSchemaId
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
pub enum PrivateSessionKeysResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSessionKeysResponseSchemaId> for PrivateSessionKeysResponseSchemaId {
    fn from(value: &PrivateSessionKeysResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSessionKeysResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSessionKeysResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSessionKeysResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSessionKeysResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSessionKeysResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSessionKeysResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSessionKeysResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "public_session_keys"
  ],
  "properties": {
    "public_session_keys": {
      "title": "public_session_keys",
      "description": "List of session keys (includes unactivated and expired keys)",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/SessionKeyResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSessionKeysResultSchema {
    ///List of session keys (includes unactivated and expired keys)
    pub public_session_keys: Vec<SessionKeyResponseSchema>,
}
impl From<&PrivateSessionKeysResultSchema> for PrivateSessionKeysResultSchema {
    fn from(value: &PrivateSessionKeysResultSchema) -> Self {
        value.clone()
    }
}
///SessionKeyResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "expiry_sec",
    "public_session_key"
  ],
  "properties": {
    "expiry_sec": {
      "title": "expiry_sec",
      "description": "Session key expiry timestamp in sec",
      "type": "integer"
    },
    "label": {
      "title": "label",
      "description": "User-defined session key label",
      "default": "",
      "type": "string"
    },
    "public_session_key": {
      "title": "public_session_key",
      "description": "Public session key address (Ethereum EOA)",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SessionKeyResponseSchema {
    ///Session key expiry timestamp in sec
    pub expiry_sec: i64,
    ///User-defined session key label
    #[serde(default)]
    pub label: String,
    ///Public session key address (Ethereum EOA)
    pub public_session_key: String,
}
impl From<&SessionKeyResponseSchema> for SessionKeyResponseSchema {
    fn from(value: &SessionKeyResponseSchema) -> Self {
        value.clone()
    }
}
