#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin_type",
  "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
  "type": "string",
  "enum": [
    "PM",
    "SM"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum MarginType {
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SM")]
    Sm,
}
impl From<&MarginType> for MarginType {
    fn from(value: &MarginType) -> Self {
        value.clone()
    }
}
impl ToString for MarginType {
    fn to_string(&self) -> String {
        match *self {
            Self::Pm => "PM".to_string(),
            Self::Sm => "SM".to_string(),
        }
    }
}
impl std::str::FromStr for MarginType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PM" => Ok(Self::Pm),
            "SM" => Ok(Self::Sm),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for MarginType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MarginType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MarginType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Calculates MtM and maintenance margin for a given subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/margin_watch",
  "description": "Calculates MtM and maintenance margin for a given subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicMarginWatchJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicMarginWatch(pub PublicMarginWatchJsonrpcSchema);
impl std::ops::Deref for PublicMarginWatch {
    type Target = PublicMarginWatchJsonrpcSchema;
    fn deref(&self) -> &PublicMarginWatchJsonrpcSchema {
        &self.0
    }
}
impl From<PublicMarginWatch> for PublicMarginWatchJsonrpcSchema {
    fn from(value: PublicMarginWatch) -> Self {
        value.0
    }
}
impl From<&PublicMarginWatch> for PublicMarginWatch {
    fn from(value: &PublicMarginWatch) -> Self {
        value.clone()
    }
}
impl From<PublicMarginWatchJsonrpcSchema> for PublicMarginWatch {
    fn from(value: PublicMarginWatchJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicMarginWatchJsonrpcSchema
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
      "$ref": "#/definitions/PublicMarginWatchRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicMarginWatchResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicMarginWatchJsonrpcSchema {
    pub request: PublicMarginWatchRequestSchema,
    pub response: PublicMarginWatchResponseSchema,
}
impl From<&PublicMarginWatchJsonrpcSchema> for PublicMarginWatchJsonrpcSchema {
    fn from(value: &PublicMarginWatchJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicMarginWatchParamsSchema
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
      "description": "Subaccount ID to get margin for.",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicMarginWatchParamsSchema {
    ///Subaccount ID to get margin for.
    pub subaccount_id: i64,
}
impl From<&PublicMarginWatchParamsSchema> for PublicMarginWatchParamsSchema {
    fn from(value: &PublicMarginWatchParamsSchema) -> Self {
        value.clone()
    }
}
///PublicMarginWatchRequestSchema
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
      "const": "public/margin_watch"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicMarginWatchParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicMarginWatchRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicMarginWatchRequestSchemaId>,
    pub method: String,
    pub params: PublicMarginWatchParamsSchema,
}
impl From<&PublicMarginWatchRequestSchema> for PublicMarginWatchRequestSchema {
    fn from(value: &PublicMarginWatchRequestSchema) -> Self {
        value.clone()
    }
}
///PublicMarginWatchRequestSchemaId
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
pub enum PublicMarginWatchRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicMarginWatchRequestSchemaId> for PublicMarginWatchRequestSchemaId {
    fn from(value: &PublicMarginWatchRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicMarginWatchRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicMarginWatchRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicMarginWatchRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicMarginWatchRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicMarginWatchRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicMarginWatchRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicMarginWatchResponseSchema
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
      "$ref": "#/definitions/PublicMarginWatchResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicMarginWatchResponseSchema {
    pub id: PublicMarginWatchResponseSchemaId,
    ///
    pub result: PublicMarginWatchResultSchema,
}
impl From<&PublicMarginWatchResponseSchema> for PublicMarginWatchResponseSchema {
    fn from(value: &PublicMarginWatchResponseSchema) -> Self {
        value.clone()
    }
}
///PublicMarginWatchResponseSchemaId
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
pub enum PublicMarginWatchResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicMarginWatchResponseSchemaId> for PublicMarginWatchResponseSchemaId {
    fn from(value: &PublicMarginWatchResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicMarginWatchResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicMarginWatchResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicMarginWatchResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicMarginWatchResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicMarginWatchResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicMarginWatchResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicMarginWatchResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "maintenance_margin",
    "margin_type",
    "subaccount_id",
    "subaccount_value",
    "valuation_timestamp"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency of subaccount",
      "type": "string"
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.",
      "type": "string",
      "format": "decimal"
    },
    "margin_type": {
      "title": "margin_type",
      "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
      "type": "string",
      "enum": [
        "PM",
        "SM"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "subaccount_value": {
      "title": "subaccount_value",
      "description": "Total mark-to-market value of all positions and collaterals",
      "type": "string",
      "format": "decimal"
    },
    "valuation_timestamp": {
      "title": "valuation_timestamp",
      "description": "Timestamp (in seconds since epoch) of when margin and MtM were computed.",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicMarginWatchResultSchema {
    ///Currency of subaccount
    pub currency: String,
    ///Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
    pub margin_type: MarginType,
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Total mark-to-market value of all positions and collaterals
    pub subaccount_value: bigdecimal::BigDecimal,
    ///Timestamp (in seconds since epoch) of when margin and MtM were computed.
    pub valuation_timestamp: i64,
}
impl From<&PublicMarginWatchResultSchema> for PublicMarginWatchResultSchema {
    fn from(value: &PublicMarginWatchResultSchema) -> Self {
        value.clone()
    }
}
