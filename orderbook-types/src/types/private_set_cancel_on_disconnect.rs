#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Enables cancel on disconnect for the account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/set_cancel_on_disconnect",
  "description": "Enables cancel on disconnect for the account",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateSetCancelOnDisconnectJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateSetCancelOnDisconnect(pub PrivateSetCancelOnDisconnectJsonrpcSchema);
impl std::ops::Deref for PrivateSetCancelOnDisconnect {
    type Target = PrivateSetCancelOnDisconnectJsonrpcSchema;
    fn deref(&self) -> &PrivateSetCancelOnDisconnectJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateSetCancelOnDisconnect> for PrivateSetCancelOnDisconnectJsonrpcSchema {
    fn from(value: PrivateSetCancelOnDisconnect) -> Self {
        value.0
    }
}
impl From<&PrivateSetCancelOnDisconnect> for PrivateSetCancelOnDisconnect {
    fn from(value: &PrivateSetCancelOnDisconnect) -> Self {
        value.clone()
    }
}
impl From<PrivateSetCancelOnDisconnectJsonrpcSchema> for PrivateSetCancelOnDisconnect {
    fn from(value: PrivateSetCancelOnDisconnectJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateSetCancelOnDisconnectJsonrpcSchema
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
      "$ref": "#/definitions/PrivateSetCancelOnDisconnectRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateSetCancelOnDisconnectResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetCancelOnDisconnectJsonrpcSchema {
    pub request: PrivateSetCancelOnDisconnectRequestSchema,
    pub response: PrivateSetCancelOnDisconnectResponseSchema,
}
impl From<&PrivateSetCancelOnDisconnectJsonrpcSchema>
for PrivateSetCancelOnDisconnectJsonrpcSchema {
    fn from(value: &PrivateSetCancelOnDisconnectJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateSetCancelOnDisconnectParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "enabled",
    "wallet"
  ],
  "properties": {
    "enabled": {
      "title": "enabled",
      "description": "Whether to enable or disable cancel on disconnect",
      "type": "boolean"
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

pub struct PrivateSetCancelOnDisconnectParamsSchema {
    ///Whether to enable or disable cancel on disconnect
    pub enabled: bool,
    ///Public key (wallet) of the account
    pub wallet: String,
}
impl From<&PrivateSetCancelOnDisconnectParamsSchema>
for PrivateSetCancelOnDisconnectParamsSchema {
    fn from(value: &PrivateSetCancelOnDisconnectParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateSetCancelOnDisconnectRequestSchema
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
      "const": "private/set_cancel_on_disconnect"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateSetCancelOnDisconnectParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetCancelOnDisconnectRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateSetCancelOnDisconnectRequestSchemaId>,
    pub method: String,
    pub params: PrivateSetCancelOnDisconnectParamsSchema,
}
impl From<&PrivateSetCancelOnDisconnectRequestSchema>
for PrivateSetCancelOnDisconnectRequestSchema {
    fn from(value: &PrivateSetCancelOnDisconnectRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateSetCancelOnDisconnectRequestSchemaId
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
pub enum PrivateSetCancelOnDisconnectRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSetCancelOnDisconnectRequestSchemaId>
for PrivateSetCancelOnDisconnectRequestSchemaId {
    fn from(value: &PrivateSetCancelOnDisconnectRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSetCancelOnDisconnectRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSetCancelOnDisconnectRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSetCancelOnDisconnectRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSetCancelOnDisconnectRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSetCancelOnDisconnectRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSetCancelOnDisconnectRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSetCancelOnDisconnectResponseSchema
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
      "description": "",
      "type": "string",
      "const": "ok"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetCancelOnDisconnectResponseSchema {
    pub id: PrivateSetCancelOnDisconnectResponseSchemaId,
    ///
    pub result: String,
}
impl From<&PrivateSetCancelOnDisconnectResponseSchema>
for PrivateSetCancelOnDisconnectResponseSchema {
    fn from(value: &PrivateSetCancelOnDisconnectResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateSetCancelOnDisconnectResponseSchemaId
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
pub enum PrivateSetCancelOnDisconnectResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSetCancelOnDisconnectResponseSchemaId>
for PrivateSetCancelOnDisconnectResponseSchemaId {
    fn from(value: &PrivateSetCancelOnDisconnectResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSetCancelOnDisconnectResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSetCancelOnDisconnectResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSetCancelOnDisconnectResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSetCancelOnDisconnectResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSetCancelOnDisconnectResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSetCancelOnDisconnectResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
