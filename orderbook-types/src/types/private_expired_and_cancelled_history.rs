#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Generate a list of URLs to retrieve archived orders
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/expired_and_cancelled_history",
  "description": "Generate a list of URLs to retrieve archived orders",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateExpiredAndCancelledHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateExpiredAndCancelledHistory(
    pub PrivateExpiredAndCancelledHistoryJsonrpcSchema,
);
impl std::ops::Deref for PrivateExpiredAndCancelledHistory {
    type Target = PrivateExpiredAndCancelledHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateExpiredAndCancelledHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateExpiredAndCancelledHistory>
for PrivateExpiredAndCancelledHistoryJsonrpcSchema {
    fn from(value: PrivateExpiredAndCancelledHistory) -> Self {
        value.0
    }
}
impl From<&PrivateExpiredAndCancelledHistory> for PrivateExpiredAndCancelledHistory {
    fn from(value: &PrivateExpiredAndCancelledHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateExpiredAndCancelledHistoryJsonrpcSchema>
for PrivateExpiredAndCancelledHistory {
    fn from(value: PrivateExpiredAndCancelledHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateExpiredAndCancelledHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateExpiredAndCancelledHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateExpiredAndCancelledHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateExpiredAndCancelledHistoryJsonrpcSchema {
    pub request: PrivateExpiredAndCancelledHistoryRequestSchema,
    pub response: PrivateExpiredAndCancelledHistoryResponseSchema,
}
impl From<&PrivateExpiredAndCancelledHistoryJsonrpcSchema>
for PrivateExpiredAndCancelledHistoryJsonrpcSchema {
    fn from(value: &PrivateExpiredAndCancelledHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateExpiredAndCancelledHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "end_timestamp",
    "expiry",
    "start_timestamp",
    "subaccount_id",
    "wallet"
  ],
  "properties": {
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "End Unix timestamp",
      "type": "integer"
    },
    "expiry": {
      "title": "expiry",
      "description": "Expiry of download link in seconds. Maximum of 604800.",
      "type": "integer"
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Start Unix timestamp",
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount to download data for",
      "type": "integer"
    },
    "wallet": {
      "title": "wallet",
      "description": "Wallet to download data for",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateExpiredAndCancelledHistoryParamsSchema {
    ///End Unix timestamp
    pub end_timestamp: i64,
    ///Expiry of download link in seconds. Maximum of 604800.
    pub expiry: i64,
    ///Start Unix timestamp
    pub start_timestamp: i64,
    ///Subaccount to download data for
    pub subaccount_id: i64,
    ///Wallet to download data for
    pub wallet: String,
}
impl From<&PrivateExpiredAndCancelledHistoryParamsSchema>
for PrivateExpiredAndCancelledHistoryParamsSchema {
    fn from(value: &PrivateExpiredAndCancelledHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateExpiredAndCancelledHistoryRequestSchema
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
      "const": "private/expired_and_cancelled_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateExpiredAndCancelledHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateExpiredAndCancelledHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateExpiredAndCancelledHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateExpiredAndCancelledHistoryParamsSchema,
}
impl From<&PrivateExpiredAndCancelledHistoryRequestSchema>
for PrivateExpiredAndCancelledHistoryRequestSchema {
    fn from(value: &PrivateExpiredAndCancelledHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateExpiredAndCancelledHistoryRequestSchemaId
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
pub enum PrivateExpiredAndCancelledHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateExpiredAndCancelledHistoryRequestSchemaId>
for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    fn from(value: &PrivateExpiredAndCancelledHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateExpiredAndCancelledHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateExpiredAndCancelledHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateExpiredAndCancelledHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateExpiredAndCancelledHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateExpiredAndCancelledHistoryResponseSchema {
    pub id: PrivateExpiredAndCancelledHistoryResponseSchemaId,
    ///
    pub result: PrivateExpiredAndCancelledHistoryResultSchema,
}
impl From<&PrivateExpiredAndCancelledHistoryResponseSchema>
for PrivateExpiredAndCancelledHistoryResponseSchema {
    fn from(value: &PrivateExpiredAndCancelledHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateExpiredAndCancelledHistoryResponseSchemaId
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
pub enum PrivateExpiredAndCancelledHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateExpiredAndCancelledHistoryResponseSchemaId>
for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    fn from(value: &PrivateExpiredAndCancelledHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateExpiredAndCancelledHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateExpiredAndCancelledHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateExpiredAndCancelledHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "presigned_urls"
  ],
  "properties": {
    "presigned_urls": {
      "title": "presigned_urls",
      "description": "List of presigned URLs to the snapshots",
      "type": "array",
      "items": {
        "title": "presigned_urls",
        "type": "string"
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateExpiredAndCancelledHistoryResultSchema {
    ///List of presigned URLs to the snapshots
    pub presigned_urls: Vec<String>,
}
impl From<&PrivateExpiredAndCancelledHistoryResultSchema>
for PrivateExpiredAndCancelledHistoryResultSchema {
    fn from(value: &PrivateExpiredAndCancelledHistoryResultSchema) -> Self {
        value.clone()
    }
}
