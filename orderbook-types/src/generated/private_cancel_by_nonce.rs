#![allow(unused_variables)]
#![allow(unused_imports)]
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;
///Cancel an order with a given subaccount and a given nonce.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_by_nonce",
  "description": "Cancel an order with a given subaccount and a given nonce.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelByNonceJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelByNonce(pub PrivateCancelByNonceJsonrpcSchema);
impl std::ops::Deref for PrivateCancelByNonce {
    type Target = PrivateCancelByNonceJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelByNonceJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelByNonce> for PrivateCancelByNonceJsonrpcSchema {
    fn from(value: PrivateCancelByNonce) -> Self {
        value.0
    }
}
impl From<&PrivateCancelByNonce> for PrivateCancelByNonce {
    fn from(value: &PrivateCancelByNonce) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelByNonceJsonrpcSchema> for PrivateCancelByNonce {
    fn from(value: PrivateCancelByNonceJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelByNonceJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelByNonceRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByNonceResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByNonceJsonrpcSchema {
    pub request: PrivateCancelByNonceRequestSchema,
    pub response: PrivateCancelByNonceResponseSchema,
}
impl From<&PrivateCancelByNonceJsonrpcSchema> for PrivateCancelByNonceJsonrpcSchema {
    fn from(value: &PrivateCancelByNonceJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByNonceParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "nonce",
    "subaccount_id"
  ],
  "properties": {
    "nonce": {
      "title": "nonce",
      "description": "Cancel an order with this nonce",
      "type": "integer"
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

pub struct PrivateCancelByNonceParamsSchema {
    ///Cancel an order with this nonce
    pub nonce: i64,
    ///Subaccount ID
    pub subaccount_id: i64,
    pub wallet: String,
    pub instrument_name: String,
}
impl From<&PrivateCancelByNonceParamsSchema> for PrivateCancelByNonceParamsSchema {
    fn from(value: &PrivateCancelByNonceParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByNonceRequestSchema
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
      "const": "private/cancel_by_nonce"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByNonceParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByNonceRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelByNonceRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelByNonceParamsSchema,
}
impl From<&PrivateCancelByNonceRequestSchema> for PrivateCancelByNonceRequestSchema {
    fn from(value: &PrivateCancelByNonceRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByNonceRequestSchemaId
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
pub enum PrivateCancelByNonceRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByNonceRequestSchemaId> for PrivateCancelByNonceRequestSchemaId {
    fn from(value: &PrivateCancelByNonceRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByNonceRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByNonceRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByNonceRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByNonceRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByNonceRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByNonceRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByNonceResponseSchema
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
      "$ref": "#/definitions/PrivateCancelByNonceResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByNonceResponseSchema {
    pub id: PrivateCancelByNonceResponseSchemaId,
    ///
    pub result: PrivateCancelByNonceResultSchema,
}
impl From<&PrivateCancelByNonceResponseSchema> for PrivateCancelByNonceResponseSchema {
    fn from(value: &PrivateCancelByNonceResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByNonceResponseSchemaId
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
pub enum PrivateCancelByNonceResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByNonceResponseSchemaId> for PrivateCancelByNonceResponseSchemaId {
    fn from(value: &PrivateCancelByNonceResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByNonceResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByNonceResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByNonceResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByNonceResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByNonceResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByNonceResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByNonceResultSchema
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

pub struct PrivateCancelByNonceResultSchema {
    ///Number of cancelled orders
    pub cancelled_orders: i64,
}
impl From<&PrivateCancelByNonceResultSchema> for PrivateCancelByNonceResultSchema {
    fn from(value: &PrivateCancelByNonceResultSchema) -> Self {
        value.clone()
    }
}
