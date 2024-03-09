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
  "title": "public/get_time",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetTimeJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetTime(pub PublicGetTimeJsonrpcSchema);
impl std::ops::Deref for PublicGetTime {
    type Target = PublicGetTimeJsonrpcSchema;
    fn deref(&self) -> &PublicGetTimeJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetTime> for PublicGetTimeJsonrpcSchema {
    fn from(value: PublicGetTime) -> Self {
        value.0
    }
}
impl From<&PublicGetTime> for PublicGetTime {
    fn from(value: &PublicGetTime) -> Self {
        value.clone()
    }
}
impl From<PublicGetTimeJsonrpcSchema> for PublicGetTime {
    fn from(value: PublicGetTimeJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetTimeJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetTimeRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTimeResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTimeJsonrpcSchema {
    pub request: PublicGetTimeRequestSchema,
    pub response: PublicGetTimeResponseSchema,
}
impl From<&PublicGetTimeJsonrpcSchema> for PublicGetTimeJsonrpcSchema {
    fn from(value: &PublicGetTimeJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetTimeParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTimeParamsSchema {}
impl From<&PublicGetTimeParamsSchema> for PublicGetTimeParamsSchema {
    fn from(value: &PublicGetTimeParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetTimeRequestSchema
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
      "const": "public/get_time"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTimeParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTimeRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetTimeRequestSchemaId>,
    pub method: String,
    pub params: PublicGetTimeParamsSchema,
}
impl From<&PublicGetTimeRequestSchema> for PublicGetTimeRequestSchema {
    fn from(value: &PublicGetTimeRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetTimeRequestSchemaId
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
pub enum PublicGetTimeRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTimeRequestSchemaId> for PublicGetTimeRequestSchemaId {
    fn from(value: &PublicGetTimeRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTimeRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTimeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTimeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTimeRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTimeRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTimeRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetTimeResponseSchema
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
      "description": "Current time in milliseconds since UNIX epoch",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTimeResponseSchema {
    pub id: PublicGetTimeResponseSchemaId,
    ///Current time in milliseconds since UNIX epoch
    pub result: i64,
}
impl From<&PublicGetTimeResponseSchema> for PublicGetTimeResponseSchema {
    fn from(value: &PublicGetTimeResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetTimeResponseSchemaId
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
pub enum PublicGetTimeResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTimeResponseSchemaId> for PublicGetTimeResponseSchemaId {
    fn from(value: &PublicGetTimeResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTimeResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTimeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTimeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTimeResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTimeResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTimeResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
