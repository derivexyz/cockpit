#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Authenticate a websocket connection. Unavailable via HTTP.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/login",
  "description": "Authenticate a websocket connection. Unavailable via HTTP.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicLoginJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicLogin(pub PublicLoginJsonrpcSchema);
impl std::ops::Deref for PublicLogin {
    type Target = PublicLoginJsonrpcSchema;
    fn deref(&self) -> &PublicLoginJsonrpcSchema {
        &self.0
    }
}
impl From<PublicLogin> for PublicLoginJsonrpcSchema {
    fn from(value: PublicLogin) -> Self {
        value.0
    }
}
impl From<&PublicLogin> for PublicLogin {
    fn from(value: &PublicLogin) -> Self {
        value.clone()
    }
}
impl From<PublicLoginJsonrpcSchema> for PublicLogin {
    fn from(value: PublicLoginJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicLoginJsonrpcSchema
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
      "$ref": "#/definitions/PublicLoginRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicLoginResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicLoginJsonrpcSchema {
    pub request: PublicLoginRequestSchema,
    pub response: PublicLoginResponseSchema,
}
impl From<&PublicLoginJsonrpcSchema> for PublicLoginJsonrpcSchema {
    fn from(value: &PublicLoginJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicLoginParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "signature",
    "timestamp",
    "wallet"
  ],
  "properties": {
    "signature": {
      "title": "signature",
      "description": "Signature of the timestamp, signed with the wallet's private key or a session key",
      "type": "string"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Message that was signed, in the form of a timestamp in ms since Unix epoch",
      "type": "string"
    },
    "wallet": {
      "title": "wallet",
      "description": "Public key (wallet) of the account",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicLoginParamsSchema {
    ///Signature of the timestamp, signed with the wallet's private key or a session key
    pub signature: String,
    ///Message that was signed, in the form of a timestamp in ms since Unix epoch
    pub timestamp: String,
    ///Public key (wallet) of the account
    pub wallet: String,
}
impl From<&PublicLoginParamsSchema> for PublicLoginParamsSchema {
    fn from(value: &PublicLoginParamsSchema) -> Self {
        value.clone()
    }
}
///PublicLoginRequestSchema
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
      "const": "public/login"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicLoginParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicLoginRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicLoginRequestSchemaId>,
    pub method: String,
    pub params: PublicLoginParamsSchema,
}
impl From<&PublicLoginRequestSchema> for PublicLoginRequestSchema {
    fn from(value: &PublicLoginRequestSchema) -> Self {
        value.clone()
    }
}
///PublicLoginRequestSchemaId
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
pub enum PublicLoginRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicLoginRequestSchemaId> for PublicLoginRequestSchemaId {
    fn from(value: &PublicLoginRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicLoginRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicLoginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicLoginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicLoginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicLoginRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicLoginRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicLoginResponseSchema
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
      "title": "result",
      "description": "List of subaccount IDs that have been authenticated",
      "type": "array",
      "items": {
        "title": "result",
        "type": "integer"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicLoginResponseSchema {
    pub id: PublicLoginResponseSchemaId,
    ///List of subaccount IDs that have been authenticated
    pub result: Vec<i64>,
}
impl From<&PublicLoginResponseSchema> for PublicLoginResponseSchema {
    fn from(value: &PublicLoginResponseSchema) -> Self {
        value.clone()
    }
}
///PublicLoginResponseSchemaId
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
pub enum PublicLoginResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicLoginResponseSchemaId> for PublicLoginResponseSchemaId {
    fn from(value: &PublicLoginResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicLoginResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicLoginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicLoginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicLoginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicLoginResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicLoginResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
