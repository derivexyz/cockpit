#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
/**Cancels RFQs given optional filters. If no filters are provided, all RFQs for the subaccount are cancelled.
All filters are combined using `AND` logic, so mutually exclusive filters will result in no RFQs being cancelled.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_batch_rfqs",
  "description": "Cancels RFQs given optional filters. If no filters are provided, all RFQs for the subaccount are cancelled.\nAll filters are combined using `AND` logic, so mutually exclusive filters will result in no RFQs being cancelled.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelBatchRfqsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelBatchRfqs(pub PrivateCancelBatchRfqsJsonrpcSchema);
impl std::ops::Deref for PrivateCancelBatchRfqs {
    type Target = PrivateCancelBatchRfqsJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelBatchRfqsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelBatchRfqs> for PrivateCancelBatchRfqsJsonrpcSchema {
    fn from(value: PrivateCancelBatchRfqs) -> Self {
        value.0
    }
}
impl From<&PrivateCancelBatchRfqs> for PrivateCancelBatchRfqs {
    fn from(value: &PrivateCancelBatchRfqs) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelBatchRfqsJsonrpcSchema> for PrivateCancelBatchRfqs {
    fn from(value: PrivateCancelBatchRfqsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelBatchRfqsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelBatchRfqsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelBatchRfqsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchRfqsJsonrpcSchema {
    pub request: PrivateCancelBatchRfqsRequestSchema,
    pub response: PrivateCancelBatchRfqsResponseSchema,
}
impl From<&PrivateCancelBatchRfqsJsonrpcSchema> for PrivateCancelBatchRfqsJsonrpcSchema {
    fn from(value: &PrivateCancelBatchRfqsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchRfqsParamsSchema
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
    "label": {
      "title": "label",
      "description": "Cancel RFQs with this label",
      "type": [
        "string",
        "null"
      ]
    },
    "nonce": {
      "title": "nonce",
      "description": "Cancel RFQ with this nonce",
      "type": [
        "integer",
        "null"
      ]
    },
    "rfq_id": {
      "title": "rfq_id",
      "description": "RFQ ID to cancel",
      "type": [
        "string",
        "null"
      ]
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

pub struct PrivateCancelBatchRfqsParamsSchema {
    ///Cancel RFQs with this label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    ///Cancel RFQ with this nonce
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
    ///RFQ ID to cancel
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelBatchRfqsParamsSchema> for PrivateCancelBatchRfqsParamsSchema {
    fn from(value: &PrivateCancelBatchRfqsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchRfqsRequestSchema
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
      "const": "private/cancel_batch_rfqs"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelBatchRfqsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchRfqsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelBatchRfqsRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelBatchRfqsParamsSchema,
}
impl From<&PrivateCancelBatchRfqsRequestSchema> for PrivateCancelBatchRfqsRequestSchema {
    fn from(value: &PrivateCancelBatchRfqsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchRfqsRequestSchemaId
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
pub enum PrivateCancelBatchRfqsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelBatchRfqsRequestSchemaId>
for PrivateCancelBatchRfqsRequestSchemaId {
    fn from(value: &PrivateCancelBatchRfqsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelBatchRfqsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelBatchRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelBatchRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelBatchRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelBatchRfqsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelBatchRfqsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelBatchRfqsResponseSchema
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
      "$ref": "#/definitions/PrivateCancelBatchRfqsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchRfqsResponseSchema {
    pub id: PrivateCancelBatchRfqsResponseSchemaId,
    ///
    pub result: PrivateCancelBatchRfqsResultSchema,
}
impl From<&PrivateCancelBatchRfqsResponseSchema>
for PrivateCancelBatchRfqsResponseSchema {
    fn from(value: &PrivateCancelBatchRfqsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchRfqsResponseSchemaId
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
pub enum PrivateCancelBatchRfqsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelBatchRfqsResponseSchemaId>
for PrivateCancelBatchRfqsResponseSchemaId {
    fn from(value: &PrivateCancelBatchRfqsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelBatchRfqsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelBatchRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelBatchRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelBatchRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelBatchRfqsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelBatchRfqsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelBatchRfqsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancelled_ids"
  ],
  "properties": {
    "cancelled_ids": {
      "title": "cancelled_ids",
      "description": "RFQ IDs of the cancelled RFQs",
      "type": "array",
      "items": {
        "title": "cancelled_ids",
        "type": "string",
        "format": "uuid"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchRfqsResultSchema {
    ///RFQ IDs of the cancelled RFQs
    pub cancelled_ids: Vec<uuid::Uuid>,
}
impl From<&PrivateCancelBatchRfqsResultSchema> for PrivateCancelBatchRfqsResultSchema {
    fn from(value: &PrivateCancelBatchRfqsResultSchema) -> Self {
        value.clone()
    }
}
