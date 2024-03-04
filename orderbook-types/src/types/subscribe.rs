#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Subscribe to a list of channels.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "subscribe",
  "description": "Subscribe to a list of channels.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/SubscribeJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Subscribe(pub SubscribeJsonrpcSchema);
impl std::ops::Deref for Subscribe {
    type Target = SubscribeJsonrpcSchema;
    fn deref(&self) -> &SubscribeJsonrpcSchema {
        &self.0
    }
}
impl From<Subscribe> for SubscribeJsonrpcSchema {
    fn from(value: Subscribe) -> Self {
        value.0
    }
}
impl From<&Subscribe> for Subscribe {
    fn from(value: &Subscribe) -> Self {
        value.clone()
    }
}
impl From<SubscribeJsonrpcSchema> for Subscribe {
    fn from(value: SubscribeJsonrpcSchema) -> Self {
        Self(value)
    }
}
///SubscribeJsonrpcSchema
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
      "$ref": "#/definitions/SubscribeRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/SubscribeResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubscribeJsonrpcSchema {
    pub request: SubscribeRequestSchema,
    pub response: SubscribeResponseSchema,
}
impl From<&SubscribeJsonrpcSchema> for SubscribeJsonrpcSchema {
    fn from(value: &SubscribeJsonrpcSchema) -> Self {
        value.clone()
    }
}
///SubscribeParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channels"
  ],
  "properties": {
    "channels": {
      "title": "channels",
      "description": "A list of channels names to subscribe to",
      "type": "array",
      "items": {
        "title": "channels",
        "type": "string"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubscribeParamsSchema {
    ///A list of channels names to subscribe to
    pub channels: Vec<String>,
}
impl From<&SubscribeParamsSchema> for SubscribeParamsSchema {
    fn from(value: &SubscribeParamsSchema) -> Self {
        value.clone()
    }
}
///SubscribeRequestSchema
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
      "const": "subscribe"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/SubscribeParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubscribeRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<SubscribeRequestSchemaId>,
    pub method: String,
    pub params: SubscribeParamsSchema,
}
impl From<&SubscribeRequestSchema> for SubscribeRequestSchema {
    fn from(value: &SubscribeRequestSchema) -> Self {
        value.clone()
    }
}
///SubscribeRequestSchemaId
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
pub enum SubscribeRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&SubscribeRequestSchemaId> for SubscribeRequestSchemaId {
    fn from(value: &SubscribeRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SubscribeRequestSchemaId {
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
impl std::convert::TryFrom<&str> for SubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for SubscribeRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for SubscribeRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///SubscribeResponseSchema
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
      "$ref": "#/definitions/SubscribeResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubscribeResponseSchema {
    pub id: SubscribeResponseSchemaId,
    ///
    pub result: SubscribeResultSchema,
}
impl From<&SubscribeResponseSchema> for SubscribeResponseSchema {
    fn from(value: &SubscribeResponseSchema) -> Self {
        value.clone()
    }
}
///SubscribeResponseSchemaId
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
pub enum SubscribeResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&SubscribeResponseSchemaId> for SubscribeResponseSchemaId {
    fn from(value: &SubscribeResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for SubscribeResponseSchemaId {
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
impl std::convert::TryFrom<&str> for SubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for SubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for SubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for SubscribeResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for SubscribeResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///SubscribeResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "current_subscriptions",
    "status"
  ],
  "properties": {
    "current_subscriptions": {
      "title": "current_subscriptions",
      "description": "A list of channels subscribed to after the subscribe operation.",
      "type": "array",
      "items": {
        "title": "current_subscriptions",
        "type": "string"
      }
    },
    "status": {
      "title": "status",
      "description": "A mapping of `channel`&nbsp;&#11106;&nbsp;`status`.<br />Successful subscriptions will have status `ok`.<br />Failed subscriptions will contain an error message.",
      "type": "object",
      "additionalProperties": {
        "title": "status",
        "type": "string"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubscribeResultSchema {
    ///A list of channels subscribed to after the subscribe operation.
    pub current_subscriptions: Vec<String>,
    ///A mapping of `channel`&nbsp;&#11106;&nbsp;`status`.<br />Successful subscriptions will have status `ok`.<br />Failed subscriptions will contain an error message.
    pub status: std::collections::HashMap<String, String>,
}
impl From<&SubscribeResultSchema> for SubscribeResultSchema {
    fn from(value: &SubscribeResultSchema) -> Self {
        value.clone()
    }
}
