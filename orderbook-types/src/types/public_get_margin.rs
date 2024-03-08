#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///`PM` (Portfolio Margin) or `SM` (Standard Margin)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin_type",
  "description": "`PM` (Portfolio Margin) or `SM` (Standard Margin)",
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
/**Calculates margin for a given portfolio and (optionally) a simulated state change. Does not take into account
open orders margin requirements.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_margin",
  "description": "Calculates margin for a given portfolio and (optionally) a simulated state change. Does not take into account\nopen orders margin requirements.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetMarginJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetMargin(pub PublicGetMarginJsonrpcSchema);
impl std::ops::Deref for PublicGetMargin {
    type Target = PublicGetMarginJsonrpcSchema;
    fn deref(&self) -> &PublicGetMarginJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetMargin> for PublicGetMarginJsonrpcSchema {
    fn from(value: PublicGetMargin) -> Self {
        value.0
    }
}
impl From<&PublicGetMargin> for PublicGetMargin {
    fn from(value: &PublicGetMargin) -> Self {
        value.clone()
    }
}
impl From<PublicGetMarginJsonrpcSchema> for PublicGetMargin {
    fn from(value: PublicGetMarginJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetMarginJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetMarginRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetMarginResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetMarginJsonrpcSchema {
    pub request: PublicGetMarginRequestSchema,
    pub response: PublicGetMarginResponseSchema,
}
impl From<&PublicGetMarginJsonrpcSchema> for PublicGetMarginJsonrpcSchema {
    fn from(value: &PublicGetMarginJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetMarginParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "margin_type",
    "simulated_positions"
  ],
  "properties": {
    "margin_type": {
      "title": "margin_type",
      "description": "`PM` (Portfolio Margin) or `SM` (Standard Margin)",
      "type": "string",
      "enum": [
        "PM",
        "SM"
      ]
    },
    "simulated_position_changes": {
      "title": "simulated_position_changes",
      "description": "Optional, add positions to simulate a trade",
      "type": [
        "array",
        "null"
      ],
      "items": {
        "type": "object",
        "$ref": "#/definitions/SimulatedPositionSchema",
        "field_many": false
      }
    },
    "simulated_positions": {
      "title": "simulated_positions",
      "description": "List of positions in a simulated portfolio",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/SimulatedPositionSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetMarginParamsSchema {
    ///`PM` (Portfolio Margin) or `SM` (Standard Margin)
    pub margin_type: MarginType,
    ///Optional, add positions to simulate a trade
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulated_position_changes: Option<Vec<SimulatedPositionSchema>>,
    ///List of positions in a simulated portfolio
    pub simulated_positions: Vec<SimulatedPositionSchema>,
}
impl From<&PublicGetMarginParamsSchema> for PublicGetMarginParamsSchema {
    fn from(value: &PublicGetMarginParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetMarginRequestSchema
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
      "const": "public/get_margin"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetMarginParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetMarginRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetMarginRequestSchemaId>,
    pub method: String,
    pub params: PublicGetMarginParamsSchema,
}
impl From<&PublicGetMarginRequestSchema> for PublicGetMarginRequestSchema {
    fn from(value: &PublicGetMarginRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetMarginRequestSchemaId
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
pub enum PublicGetMarginRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetMarginRequestSchemaId> for PublicGetMarginRequestSchemaId {
    fn from(value: &PublicGetMarginRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetMarginRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetMarginRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetMarginRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetMarginResponseSchema
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
      "$ref": "#/definitions/PublicGetMarginResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetMarginResponseSchema {
    pub id: PublicGetMarginResponseSchemaId,
    ///
    pub result: PublicGetMarginResultSchema,
}
impl From<&PublicGetMarginResponseSchema> for PublicGetMarginResponseSchema {
    fn from(value: &PublicGetMarginResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetMarginResponseSchemaId
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
pub enum PublicGetMarginResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetMarginResponseSchemaId> for PublicGetMarginResponseSchemaId {
    fn from(value: &PublicGetMarginResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetMarginResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetMarginResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetMarginResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetMarginResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "is_valid_trade",
    "post_initial_margin",
    "post_maintenance_margin",
    "pre_initial_margin",
    "pre_maintenance_margin",
    "subaccount_id"
  ],
  "properties": {
    "is_valid_trade": {
      "title": "is_valid_trade",
      "description": "True if trade passes margin requirement",
      "type": "boolean"
    },
    "post_initial_margin": {
      "title": "post_initial_margin",
      "description": "Initial margin requirement post trade",
      "type": "string",
      "format": "decimal"
    },
    "post_maintenance_margin": {
      "title": "post_maintenance_margin",
      "description": "Maintenance margin requirement post trade",
      "type": "string",
      "format": "decimal"
    },
    "pre_initial_margin": {
      "title": "pre_initial_margin",
      "description": "Initial margin requirement before trade",
      "type": "string",
      "format": "decimal"
    },
    "pre_maintenance_margin": {
      "title": "pre_maintenance_margin",
      "description": "Maintenance margin requirement before trade",
      "type": "string",
      "format": "decimal"
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

pub struct PublicGetMarginResultSchema {
    ///True if trade passes margin requirement
    pub is_valid_trade: bool,
    ///Initial margin requirement post trade
    pub post_initial_margin: bigdecimal::BigDecimal,
    ///Maintenance margin requirement post trade
    pub post_maintenance_margin: bigdecimal::BigDecimal,
    ///Initial margin requirement before trade
    pub pre_initial_margin: bigdecimal::BigDecimal,
    ///Maintenance margin requirement before trade
    pub pre_maintenance_margin: bigdecimal::BigDecimal,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PublicGetMarginResultSchema> for PublicGetMarginResultSchema {
    fn from(value: &PublicGetMarginResultSchema) -> Self {
        value.clone()
    }
}
///SimulatedPositionSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "instrument_name"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Position amount to simulate",
      "type": "string",
      "format": "decimal"
    },
    "entry_price": {
      "title": "entry_price",
      "description": "Only for perps. Entry price to use in the simulation. Mark price is used if not provided.",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SimulatedPositionSchema {
    ///Position amount to simulate
    pub amount: bigdecimal::BigDecimal,
    ///Only for perps. Entry price to use in the simulation. Mark price is used if not provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_price: Option<bigdecimal::BigDecimal>,
    ///Instrument name
    pub instrument_name: String,
}
impl From<&SimulatedPositionSchema> for SimulatedPositionSchema {
    fn from(value: &SimulatedPositionSchema) -> Self {
        value.clone()
    }
}
