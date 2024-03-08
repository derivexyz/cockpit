#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
/**Cancels quotes given optional filters. If no filters are provided, all quotes by the subaccount are cancelled.
All filters are combined using `AND` logic, so mutually exclusive filters will result in no quotes being cancelled.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_batch_quotes",
  "description": "Cancels quotes given optional filters. If no filters are provided, all quotes by the subaccount are cancelled.\nAll filters are combined using `AND` logic, so mutually exclusive filters will result in no quotes being cancelled.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelBatchQuotesJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelBatchQuotes(pub PrivateCancelBatchQuotesJsonrpcSchema);
impl std::ops::Deref for PrivateCancelBatchQuotes {
    type Target = PrivateCancelBatchQuotesJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelBatchQuotesJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelBatchQuotes> for PrivateCancelBatchQuotesJsonrpcSchema {
    fn from(value: PrivateCancelBatchQuotes) -> Self {
        value.0
    }
}
impl From<&PrivateCancelBatchQuotes> for PrivateCancelBatchQuotes {
    fn from(value: &PrivateCancelBatchQuotes) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelBatchQuotesJsonrpcSchema> for PrivateCancelBatchQuotes {
    fn from(value: PrivateCancelBatchQuotesJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelBatchQuotesJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelBatchQuotesRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelBatchQuotesResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchQuotesJsonrpcSchema {
    pub request: PrivateCancelBatchQuotesRequestSchema,
    pub response: PrivateCancelBatchQuotesResponseSchema,
}
impl From<&PrivateCancelBatchQuotesJsonrpcSchema>
for PrivateCancelBatchQuotesJsonrpcSchema {
    fn from(value: &PrivateCancelBatchQuotesJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchQuotesParamsSchema
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
      "description": "Cancel quotes with this label",
      "type": [
        "string",
        "null"
      ]
    },
    "nonce": {
      "title": "nonce",
      "description": "Cancel quote with this nonce",
      "type": [
        "integer",
        "null"
      ]
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID to cancel",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "rfq_id": {
      "title": "rfq_id",
      "description": "Cancel quotes for this RFQ ID",
      "type": [
        "string",
        "null"
      ],
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

pub struct PrivateCancelBatchQuotesParamsSchema {
    ///Cancel quotes with this label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    ///Cancel quote with this nonce
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
    ///Quote ID to cancel
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    ///Cancel quotes for this RFQ ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelBatchQuotesParamsSchema>
for PrivateCancelBatchQuotesParamsSchema {
    fn from(value: &PrivateCancelBatchQuotesParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchQuotesRequestSchema
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
      "const": "private/cancel_batch_quotes"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelBatchQuotesParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchQuotesRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelBatchQuotesRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelBatchQuotesParamsSchema,
}
impl From<&PrivateCancelBatchQuotesRequestSchema>
for PrivateCancelBatchQuotesRequestSchema {
    fn from(value: &PrivateCancelBatchQuotesRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchQuotesRequestSchemaId
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
pub enum PrivateCancelBatchQuotesRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelBatchQuotesRequestSchemaId>
for PrivateCancelBatchQuotesRequestSchemaId {
    fn from(value: &PrivateCancelBatchQuotesRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelBatchQuotesRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelBatchQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelBatchQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelBatchQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelBatchQuotesRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelBatchQuotesRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelBatchQuotesResponseSchema
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
      "$ref": "#/definitions/PrivateCancelBatchQuotesResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelBatchQuotesResponseSchema {
    pub id: PrivateCancelBatchQuotesResponseSchemaId,
    ///
    pub result: PrivateCancelBatchQuotesResultSchema,
}
impl From<&PrivateCancelBatchQuotesResponseSchema>
for PrivateCancelBatchQuotesResponseSchema {
    fn from(value: &PrivateCancelBatchQuotesResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelBatchQuotesResponseSchemaId
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
pub enum PrivateCancelBatchQuotesResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelBatchQuotesResponseSchemaId>
for PrivateCancelBatchQuotesResponseSchemaId {
    fn from(value: &PrivateCancelBatchQuotesResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelBatchQuotesResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelBatchQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelBatchQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelBatchQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelBatchQuotesResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelBatchQuotesResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelBatchQuotesResultSchema
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
      "description": "Quote IDs of the cancelled quotes",
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

pub struct PrivateCancelBatchQuotesResultSchema {
    ///Quote IDs of the cancelled quotes
    pub cancelled_ids: Vec<uuid::Uuid>,
}
impl From<&PrivateCancelBatchQuotesResultSchema>
for PrivateCancelBatchQuotesResultSchema {
    fn from(value: &PrivateCancelBatchQuotesResultSchema) -> Self {
        value.clone()
    }
}
