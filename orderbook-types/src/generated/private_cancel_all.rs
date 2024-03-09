#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Cancel all open orders for a given subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_all",
  "description": "Cancel all open orders for a given subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelAllJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelAll(pub PrivateCancelAllJsonrpcSchema);
impl std::ops::Deref for PrivateCancelAll {
    type Target = PrivateCancelAllJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelAllJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelAll> for PrivateCancelAllJsonrpcSchema {
    fn from(value: PrivateCancelAll) -> Self {
        value.0
    }
}
impl From<&PrivateCancelAll> for PrivateCancelAll {
    fn from(value: &PrivateCancelAll) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelAllJsonrpcSchema> for PrivateCancelAll {
    fn from(value: PrivateCancelAllJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelAllJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelAllRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelAllResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelAllJsonrpcSchema {
    pub request: PrivateCancelAllRequestSchema,
    pub response: PrivateCancelAllResponseSchema,
}
impl From<&PrivateCancelAllJsonrpcSchema> for PrivateCancelAllJsonrpcSchema {
    fn from(value: &PrivateCancelAllJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelAllParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id"
  ],
  "properties": {
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelAllParamsSchema {
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelAllParamsSchema> for PrivateCancelAllParamsSchema {
    fn from(value: &PrivateCancelAllParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelAllRequestSchema
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
      "const": "private/cancel_all"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelAllParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelAllRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelAllRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelAllParamsSchema,
}
impl From<&PrivateCancelAllRequestSchema> for PrivateCancelAllRequestSchema {
    fn from(value: &PrivateCancelAllRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelAllRequestSchemaId
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
pub enum PrivateCancelAllRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelAllRequestSchemaId> for PrivateCancelAllRequestSchemaId {
    fn from(value: &PrivateCancelAllRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelAllRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelAllRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelAllRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelAllRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelAllRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelAllRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelAllResponseSchema
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
      "$ref": "#/definitions/PrivateCancelAllResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelAllResponseSchema {
    pub id: PrivateCancelAllResponseSchemaId,
    ///
    pub result: PrivateCancelAllResultSchema,
}
impl From<&PrivateCancelAllResponseSchema> for PrivateCancelAllResponseSchema {
    fn from(value: &PrivateCancelAllResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelAllResponseSchemaId
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
pub enum PrivateCancelAllResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelAllResponseSchemaId> for PrivateCancelAllResponseSchemaId {
    fn from(value: &PrivateCancelAllResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelAllResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelAllResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelAllResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelAllResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelAllResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelAllResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelAllResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancelled_orders"
  ],
  "properties": {
    "cancelled_orders": {
      "title": "cancelled_orders",
      "description": "Number of cancelled orders",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelAllResultSchema {
    ///Number of cancelled orders
    pub cancelled_orders: i64,
}
impl From<&PrivateCancelAllResultSchema> for PrivateCancelAllResultSchema {
    fn from(value: &PrivateCancelAllResultSchema) -> Self {
        value.clone()
    }
}
