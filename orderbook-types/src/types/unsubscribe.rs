#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Unsubscribe from a list of channels, or all channels.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "unsubscribe",
  "description": "Unsubscribe from a list of channels, or all channels.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/UnsubscribeJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsubscribe(pub UnsubscribeJsonrpcSchema);
impl std::ops::Deref for Unsubscribe {
    type Target = UnsubscribeJsonrpcSchema;
    fn deref(&self) -> &UnsubscribeJsonrpcSchema {
        &self.0
    }
}
impl From<Unsubscribe> for UnsubscribeJsonrpcSchema {
    fn from(value: Unsubscribe) -> Self {
        value.0
    }
}
impl From<&Unsubscribe> for Unsubscribe {
    fn from(value: &Unsubscribe) -> Self {
        value.clone()
    }
}
impl From<UnsubscribeJsonrpcSchema> for Unsubscribe {
    fn from(value: UnsubscribeJsonrpcSchema) -> Self {
        Self(value)
    }
}
///UnsubscribeJsonrpcSchema
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
      "$ref": "#/definitions/UnsubscribeRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/UnsubscribeResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct UnsubscribeJsonrpcSchema {
    pub request: UnsubscribeRequestSchema,
    pub response: UnsubscribeResponseSchema,
}
impl From<&UnsubscribeJsonrpcSchema> for UnsubscribeJsonrpcSchema {
    fn from(value: &UnsubscribeJsonrpcSchema) -> Self {
        value.clone()
    }
}
///UnsubscribeParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "properties": {
    "channels": {
      "title": "channels",
      "description": "A list of channels names to unsubscribe from.<br />If not provided, unsubscribe from all channels.",
      "default": null,
      "type": [
        "array",
        "null"
      ],
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

pub struct UnsubscribeParamsSchema {
    ///A list of channels names to unsubscribe from.<br />If not provided, unsubscribe from all channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<String>>,
}
impl From<&UnsubscribeParamsSchema> for UnsubscribeParamsSchema {
    fn from(value: &UnsubscribeParamsSchema) -> Self {
        value.clone()
    }
}
///UnsubscribeRequestSchema
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
      "const": "unsubscribe"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/UnsubscribeParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct UnsubscribeRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<UnsubscribeRequestSchemaId>,
    pub method: String,
    pub params: UnsubscribeParamsSchema,
}
impl From<&UnsubscribeRequestSchema> for UnsubscribeRequestSchema {
    fn from(value: &UnsubscribeRequestSchema) -> Self {
        value.clone()
    }
}
///UnsubscribeRequestSchemaId
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
pub enum UnsubscribeRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&UnsubscribeRequestSchemaId> for UnsubscribeRequestSchemaId {
    fn from(value: &UnsubscribeRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for UnsubscribeRequestSchemaId {
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
impl std::convert::TryFrom<&str> for UnsubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UnsubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UnsubscribeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for UnsubscribeRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for UnsubscribeRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///UnsubscribeResponseSchema
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
      "$ref": "#/definitions/UnsubscribeResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct UnsubscribeResponseSchema {
    pub id: UnsubscribeResponseSchemaId,
    ///
    pub result: UnsubscribeResultSchema,
}
impl From<&UnsubscribeResponseSchema> for UnsubscribeResponseSchema {
    fn from(value: &UnsubscribeResponseSchema) -> Self {
        value.clone()
    }
}
///UnsubscribeResponseSchemaId
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
pub enum UnsubscribeResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&UnsubscribeResponseSchemaId> for UnsubscribeResponseSchemaId {
    fn from(value: &UnsubscribeResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for UnsubscribeResponseSchemaId {
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
impl std::convert::TryFrom<&str> for UnsubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UnsubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UnsubscribeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for UnsubscribeResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for UnsubscribeResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///UnsubscribeResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "remaining_subscriptions",
    "status"
  ],
  "properties": {
    "remaining_subscriptions": {
      "title": "remaining_subscriptions",
      "description": "A list of channels still subscribed to after the unsubscribe operation.",
      "type": "array",
      "items": {
        "title": "remaining_subscriptions",
        "type": "string"
      }
    },
    "status": {
      "title": "status",
      "description": "A mapping of `channel`&nbsp;&#11106;&nbsp;`status`.<br />Successfully unsubscribed channels will have status `ok`.",
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

pub struct UnsubscribeResultSchema {
    ///A list of channels still subscribed to after the unsubscribe operation.
    pub remaining_subscriptions: Vec<String>,
    ///A mapping of `channel`&nbsp;&#11106;&nbsp;`status`.<br />Successfully unsubscribed channels will have status `ok`.
    pub status: std::collections::HashMap<String, String>,
}
impl From<&UnsubscribeResultSchema> for UnsubscribeResultSchema {
    fn from(value: &UnsubscribeResultSchema) -> Self {
        value.clone()
    }
}
