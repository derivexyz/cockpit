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
  "title": "private/change_session_key_label",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateChangeSessionKeyLabelJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateChangeSessionKeyLabel(pub PrivateChangeSessionKeyLabelJsonrpcSchema);
impl std::ops::Deref for PrivateChangeSessionKeyLabel {
    type Target = PrivateChangeSessionKeyLabelJsonrpcSchema;
    fn deref(&self) -> &PrivateChangeSessionKeyLabelJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateChangeSessionKeyLabel> for PrivateChangeSessionKeyLabelJsonrpcSchema {
    fn from(value: PrivateChangeSessionKeyLabel) -> Self {
        value.0
    }
}
impl From<&PrivateChangeSessionKeyLabel> for PrivateChangeSessionKeyLabel {
    fn from(value: &PrivateChangeSessionKeyLabel) -> Self {
        value.clone()
    }
}
impl From<PrivateChangeSessionKeyLabelJsonrpcSchema> for PrivateChangeSessionKeyLabel {
    fn from(value: PrivateChangeSessionKeyLabelJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateChangeSessionKeyLabelJsonrpcSchema
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
      "$ref": "#/definitions/PrivateChangeSessionKeyLabelRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateChangeSessionKeyLabelResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSessionKeyLabelJsonrpcSchema {
    pub request: PrivateChangeSessionKeyLabelRequestSchema,
    pub response: PrivateChangeSessionKeyLabelResponseSchema,
}
impl From<&PrivateChangeSessionKeyLabelJsonrpcSchema>
for PrivateChangeSessionKeyLabelJsonrpcSchema {
    fn from(value: &PrivateChangeSessionKeyLabelJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSessionKeyLabelParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label"
  ],
  "properties": {
    "label": {
      "title": "label",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSessionKeyLabelParamsSchema {
    pub label: String,
}
impl From<&PrivateChangeSessionKeyLabelParamsSchema>
for PrivateChangeSessionKeyLabelParamsSchema {
    fn from(value: &PrivateChangeSessionKeyLabelParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSessionKeyLabelRequestSchema
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
      "const": "private/change_session_key_label"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateChangeSessionKeyLabelParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSessionKeyLabelRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateChangeSessionKeyLabelRequestSchemaId>,
    pub method: String,
    pub params: PrivateChangeSessionKeyLabelParamsSchema,
}
impl From<&PrivateChangeSessionKeyLabelRequestSchema>
for PrivateChangeSessionKeyLabelRequestSchema {
    fn from(value: &PrivateChangeSessionKeyLabelRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSessionKeyLabelRequestSchemaId
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
pub enum PrivateChangeSessionKeyLabelRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateChangeSessionKeyLabelRequestSchemaId>
for PrivateChangeSessionKeyLabelRequestSchemaId {
    fn from(value: &PrivateChangeSessionKeyLabelRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateChangeSessionKeyLabelRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateChangeSessionKeyLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateChangeSessionKeyLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateChangeSessionKeyLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateChangeSessionKeyLabelRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateChangeSessionKeyLabelRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateChangeSessionKeyLabelResponseSchema
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
      "$ref": "#/definitions/PrivateChangeSessionKeyLabelResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSessionKeyLabelResponseSchema {
    pub id: PrivateChangeSessionKeyLabelResponseSchemaId,
    ///
    pub result: PrivateChangeSessionKeyLabelResultSchema,
}
impl From<&PrivateChangeSessionKeyLabelResponseSchema>
for PrivateChangeSessionKeyLabelResponseSchema {
    fn from(value: &PrivateChangeSessionKeyLabelResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSessionKeyLabelResponseSchemaId
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
pub enum PrivateChangeSessionKeyLabelResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateChangeSessionKeyLabelResponseSchemaId>
for PrivateChangeSessionKeyLabelResponseSchemaId {
    fn from(value: &PrivateChangeSessionKeyLabelResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateChangeSessionKeyLabelResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateChangeSessionKeyLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateChangeSessionKeyLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateChangeSessionKeyLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateChangeSessionKeyLabelResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateChangeSessionKeyLabelResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateChangeSessionKeyLabelResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label"
  ],
  "properties": {
    "label": {
      "title": "label",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSessionKeyLabelResultSchema {
    pub label: String,
}
impl From<&PrivateChangeSessionKeyLabelResultSchema>
for PrivateChangeSessionKeyLabelResultSchema {
    fn from(value: &PrivateChangeSessionKeyLabelResultSchema) -> Self {
        value.clone()
    }
}
