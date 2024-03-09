#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Cancel all open orders for a given subaccount and a given label.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_by_label",
  "description": "Cancel all open orders for a given subaccount and a given label.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelByLabelJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelByLabel(pub PrivateCancelByLabelJsonrpcSchema);
impl std::ops::Deref for PrivateCancelByLabel {
    type Target = PrivateCancelByLabelJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelByLabelJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelByLabel> for PrivateCancelByLabelJsonrpcSchema {
    fn from(value: PrivateCancelByLabel) -> Self {
        value.0
    }
}
impl From<&PrivateCancelByLabel> for PrivateCancelByLabel {
    fn from(value: &PrivateCancelByLabel) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelByLabelJsonrpcSchema> for PrivateCancelByLabel {
    fn from(value: PrivateCancelByLabelJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelByLabelJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelByLabelRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByLabelResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByLabelJsonrpcSchema {
    pub request: PrivateCancelByLabelRequestSchema,
    pub response: PrivateCancelByLabelResponseSchema,
}
impl From<&PrivateCancelByLabelJsonrpcSchema> for PrivateCancelByLabelJsonrpcSchema {
    fn from(value: &PrivateCancelByLabelJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByLabelParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label",
    "subaccount_id"
  ],
  "properties": {
    "label": {
      "title": "label",
      "description": "Cancel all orders for this label",
      "type": "string"
    },
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

pub struct PrivateCancelByLabelParamsSchema {
    ///Cancel all orders for this label
    pub label: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelByLabelParamsSchema> for PrivateCancelByLabelParamsSchema {
    fn from(value: &PrivateCancelByLabelParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByLabelRequestSchema
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
      "const": "private/cancel_by_label"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByLabelParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByLabelRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelByLabelRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelByLabelParamsSchema,
}
impl From<&PrivateCancelByLabelRequestSchema> for PrivateCancelByLabelRequestSchema {
    fn from(value: &PrivateCancelByLabelRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByLabelRequestSchemaId
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
pub enum PrivateCancelByLabelRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByLabelRequestSchemaId> for PrivateCancelByLabelRequestSchemaId {
    fn from(value: &PrivateCancelByLabelRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByLabelRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByLabelRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByLabelRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByLabelResponseSchema
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
      "$ref": "#/definitions/PrivateCancelByLabelResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByLabelResponseSchema {
    pub id: PrivateCancelByLabelResponseSchemaId,
    ///
    pub result: PrivateCancelByLabelResultSchema,
}
impl From<&PrivateCancelByLabelResponseSchema> for PrivateCancelByLabelResponseSchema {
    fn from(value: &PrivateCancelByLabelResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByLabelResponseSchemaId
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
pub enum PrivateCancelByLabelResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByLabelResponseSchemaId>
for PrivateCancelByLabelResponseSchemaId {
    fn from(value: &PrivateCancelByLabelResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByLabelResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByLabelResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByLabelResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByLabelResultSchema
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

pub struct PrivateCancelByLabelResultSchema {
    ///Number of cancelled orders
    pub cancelled_orders: i64,
}
impl From<&PrivateCancelByLabelResultSchema> for PrivateCancelByLabelResultSchema {
    fn from(value: &PrivateCancelByLabelResultSchema) -> Self {
        value.clone()
    }
}
