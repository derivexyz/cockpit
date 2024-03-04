#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
/**Calculates margin for a given subaccount and (optionally) a simulated state change. Does not take into account
open orders margin requirements.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_margin",
  "description": "Calculates margin for a given subaccount and (optionally) a simulated state change. Does not take into account\nopen orders margin requirements.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetMarginJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetMargin(pub PrivateGetMarginJsonrpcSchema);
impl std::ops::Deref for PrivateGetMargin {
    type Target = PrivateGetMarginJsonrpcSchema;
    fn deref(&self) -> &PrivateGetMarginJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetMargin> for PrivateGetMarginJsonrpcSchema {
    fn from(value: PrivateGetMargin) -> Self {
        value.0
    }
}
impl From<&PrivateGetMargin> for PrivateGetMargin {
    fn from(value: &PrivateGetMargin) -> Self {
        value.clone()
    }
}
impl From<PrivateGetMarginJsonrpcSchema> for PrivateGetMargin {
    fn from(value: PrivateGetMarginJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetMarginJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetMarginRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetMarginResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMarginJsonrpcSchema {
    pub request: PrivateGetMarginRequestSchema,
    pub response: PrivateGetMarginResponseSchema,
}
impl From<&PrivateGetMarginJsonrpcSchema> for PrivateGetMarginJsonrpcSchema {
    fn from(value: &PrivateGetMarginJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMarginParamsSchema
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
    "simulated_position_changes": {
      "title": "simulated_position_changes",
      "description": "Optional, add positions to simulate a trade",
      "default": null,
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

pub struct PrivateGetMarginParamsSchema {
    ///Optional, add positions to simulate a trade
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulated_position_changes: Option<Vec<SimulatedPositionSchema>>,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetMarginParamsSchema> for PrivateGetMarginParamsSchema {
    fn from(value: &PrivateGetMarginParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMarginRequestSchema
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
      "const": "private/get_margin"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetMarginParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMarginRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetMarginRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetMarginParamsSchema,
}
impl From<&PrivateGetMarginRequestSchema> for PrivateGetMarginRequestSchema {
    fn from(value: &PrivateGetMarginRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMarginRequestSchemaId
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
pub enum PrivateGetMarginRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetMarginRequestSchemaId> for PrivateGetMarginRequestSchemaId {
    fn from(value: &PrivateGetMarginRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetMarginRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetMarginRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetMarginRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetMarginRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetMarginResponseSchema
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
      "$ref": "#/definitions/PrivateGetMarginResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMarginResponseSchema {
    pub id: PrivateGetMarginResponseSchemaId,
    ///
    pub result: PrivateGetMarginResultSchema,
}
impl From<&PrivateGetMarginResponseSchema> for PrivateGetMarginResponseSchema {
    fn from(value: &PrivateGetMarginResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMarginResponseSchemaId
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
pub enum PrivateGetMarginResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetMarginResponseSchemaId> for PrivateGetMarginResponseSchemaId {
    fn from(value: &PrivateGetMarginResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetMarginResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetMarginResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetMarginResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetMarginResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetMarginResultSchema
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

pub struct PrivateGetMarginResultSchema {
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
impl From<&PrivateGetMarginResultSchema> for PrivateGetMarginResultSchema {
    fn from(value: &PrivateGetMarginResultSchema) -> Self {
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
      "default": null,
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
