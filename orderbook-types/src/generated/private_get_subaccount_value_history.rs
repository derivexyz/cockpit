#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get the value history of a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_subaccount_value_history",
  "description": "Get the value history of a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetSubaccountValueHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetSubaccountValueHistory(
    pub PrivateGetSubaccountValueHistoryJsonrpcSchema,
);
impl std::ops::Deref for PrivateGetSubaccountValueHistory {
    type Target = PrivateGetSubaccountValueHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetSubaccountValueHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetSubaccountValueHistory>
for PrivateGetSubaccountValueHistoryJsonrpcSchema {
    fn from(value: PrivateGetSubaccountValueHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetSubaccountValueHistory> for PrivateGetSubaccountValueHistory {
    fn from(value: &PrivateGetSubaccountValueHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetSubaccountValueHistoryJsonrpcSchema>
for PrivateGetSubaccountValueHistory {
    fn from(value: PrivateGetSubaccountValueHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetSubaccountValueHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountValueHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountValueHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountValueHistoryJsonrpcSchema {
    pub request: PrivateGetSubaccountValueHistoryRequestSchema,
    pub response: PrivateGetSubaccountValueHistoryResponseSchema,
}
impl From<&PrivateGetSubaccountValueHistoryJsonrpcSchema>
for PrivateGetSubaccountValueHistoryJsonrpcSchema {
    fn from(value: &PrivateGetSubaccountValueHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountValueHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "end_timestamp",
    "period",
    "start_timestamp",
    "subaccount_id"
  ],
  "properties": {
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "End timestamp",
      "type": "integer"
    },
    "period": {
      "title": "period",
      "description": "Period",
      "type": "integer"
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Start timestamp",
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountValueHistoryParamsSchema {
    ///End timestamp
    pub end_timestamp: i64,
    ///Period
    pub period: i64,
    ///Start timestamp
    pub start_timestamp: i64,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetSubaccountValueHistoryParamsSchema>
for PrivateGetSubaccountValueHistoryParamsSchema {
    fn from(value: &PrivateGetSubaccountValueHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountValueHistoryRequestSchema
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
      "const": "private/get_subaccount_value_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountValueHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountValueHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetSubaccountValueHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetSubaccountValueHistoryParamsSchema,
}
impl From<&PrivateGetSubaccountValueHistoryRequestSchema>
for PrivateGetSubaccountValueHistoryRequestSchema {
    fn from(value: &PrivateGetSubaccountValueHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountValueHistoryRequestSchemaId
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
pub enum PrivateGetSubaccountValueHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountValueHistoryRequestSchemaId>
for PrivateGetSubaccountValueHistoryRequestSchemaId {
    fn from(value: &PrivateGetSubaccountValueHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountValueHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountValueHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetSubaccountValueHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountValueHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountValueHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountValueHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountValueHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountValueHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountValueHistoryResponseSchema {
    pub id: PrivateGetSubaccountValueHistoryResponseSchemaId,
    ///
    pub result: PrivateGetSubaccountValueHistoryResultSchema,
}
impl From<&PrivateGetSubaccountValueHistoryResponseSchema>
for PrivateGetSubaccountValueHistoryResponseSchema {
    fn from(value: &PrivateGetSubaccountValueHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountValueHistoryResponseSchemaId
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
pub enum PrivateGetSubaccountValueHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountValueHistoryResponseSchemaId>
for PrivateGetSubaccountValueHistoryResponseSchemaId {
    fn from(value: &PrivateGetSubaccountValueHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountValueHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountValueHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PrivateGetSubaccountValueHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountValueHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountValueHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountValueHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountValueHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id",
    "subaccount_value_history"
  ],
  "properties": {
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "subaccount_value_history": {
      "title": "subaccount_value_history",
      "description": "Subaccount value history",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/SubAccountValueHistoryResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountValueHistoryResultSchema {
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Subaccount value history
    pub subaccount_value_history: Vec<SubAccountValueHistoryResponseSchema>,
}
impl From<&PrivateGetSubaccountValueHistoryResultSchema>
for PrivateGetSubaccountValueHistoryResultSchema {
    fn from(value: &PrivateGetSubaccountValueHistoryResultSchema) -> Self {
        value.clone()
    }
}
///SubAccountValueHistoryResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_value",
    "timestamp"
  ],
  "properties": {
    "subaccount_value": {
      "title": "subaccount_value",
      "description": "Total mark-to-market value of all positions and collaterals",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of when the subaccount value was recorded into the database",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubAccountValueHistoryResponseSchema {
    ///Total mark-to-market value of all positions and collaterals
    pub subaccount_value: bigdecimal::BigDecimal,
    ///Timestamp of when the subaccount value was recorded into the database
    pub timestamp: i64,
}
impl From<&SubAccountValueHistoryResponseSchema>
for SubAccountValueHistoryResponseSchema {
    fn from(value: &SubAccountValueHistoryResponseSchema) -> Self {
        value.clone()
    }
}
