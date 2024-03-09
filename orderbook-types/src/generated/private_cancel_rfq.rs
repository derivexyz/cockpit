#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Cancels a single RFQ by id.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_rfq",
  "description": "Cancels a single RFQ by id.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelRfqJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelRfq(pub PrivateCancelRfqJsonrpcSchema);
impl std::ops::Deref for PrivateCancelRfq {
    type Target = PrivateCancelRfqJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelRfqJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelRfq> for PrivateCancelRfqJsonrpcSchema {
    fn from(value: PrivateCancelRfq) -> Self {
        value.0
    }
}
impl From<&PrivateCancelRfq> for PrivateCancelRfq {
    fn from(value: &PrivateCancelRfq) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelRfqJsonrpcSchema> for PrivateCancelRfq {
    fn from(value: PrivateCancelRfqJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelRfqJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelRfqRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelRfqResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelRfqJsonrpcSchema {
    pub request: PrivateCancelRfqRequestSchema,
    pub response: PrivateCancelRfqResponseSchema,
}
impl From<&PrivateCancelRfqJsonrpcSchema> for PrivateCancelRfqJsonrpcSchema {
    fn from(value: &PrivateCancelRfqJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRfqParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "rfq_id",
    "subaccount_id"
  ],
  "properties": {
    "rfq_id": {
      "title": "rfq_id",
      "description": "RFQ ID to cancel",
      "type": "string",
      "format": "uuid"
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

pub struct PrivateCancelRfqParamsSchema {
    ///RFQ ID to cancel
    pub rfq_id: uuid::Uuid,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelRfqParamsSchema> for PrivateCancelRfqParamsSchema {
    fn from(value: &PrivateCancelRfqParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRfqRequestSchema
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
      "const": "private/cancel_rfq"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelRfqParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelRfqRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelRfqRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelRfqParamsSchema,
}
impl From<&PrivateCancelRfqRequestSchema> for PrivateCancelRfqRequestSchema {
    fn from(value: &PrivateCancelRfqRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRfqRequestSchemaId
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
pub enum PrivateCancelRfqRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelRfqRequestSchemaId> for PrivateCancelRfqRequestSchemaId {
    fn from(value: &PrivateCancelRfqRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelRfqRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelRfqRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelRfqRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelRfqResponseSchema
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
      "description": "The result of this method call, `ok` if successful",
      "type": "string",
      "const": "ok"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelRfqResponseSchema {
    pub id: PrivateCancelRfqResponseSchemaId,
    ///The result of this method call, `ok` if successful
    pub result: String,
}
impl From<&PrivateCancelRfqResponseSchema> for PrivateCancelRfqResponseSchema {
    fn from(value: &PrivateCancelRfqResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRfqResponseSchemaId
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
pub enum PrivateCancelRfqResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelRfqResponseSchemaId> for PrivateCancelRfqResponseSchemaId {
    fn from(value: &PrivateCancelRfqResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelRfqResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelRfqResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelRfqResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
