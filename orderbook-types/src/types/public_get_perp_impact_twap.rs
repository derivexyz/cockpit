#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get the TWAPs for the perp mid price, ask impact price, and bid impact price
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_perp_impact_twap",
  "description": "Get the TWAPs for the perp mid price, ask impact price, and bid impact price",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetPerpImpactTwapJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetPerpImpactTwap(pub PublicGetPerpImpactTwapJsonrpcSchema);
impl std::ops::Deref for PublicGetPerpImpactTwap {
    type Target = PublicGetPerpImpactTwapJsonrpcSchema;
    fn deref(&self) -> &PublicGetPerpImpactTwapJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetPerpImpactTwap> for PublicGetPerpImpactTwapJsonrpcSchema {
    fn from(value: PublicGetPerpImpactTwap) -> Self {
        value.0
    }
}
impl From<&PublicGetPerpImpactTwap> for PublicGetPerpImpactTwap {
    fn from(value: &PublicGetPerpImpactTwap) -> Self {
        value.clone()
    }
}
impl From<PublicGetPerpImpactTwapJsonrpcSchema> for PublicGetPerpImpactTwap {
    fn from(value: PublicGetPerpImpactTwapJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetPerpImpactTwapJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetPerpImpactTwapRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetPerpImpactTwapResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetPerpImpactTwapJsonrpcSchema {
    pub request: PublicGetPerpImpactTwapRequestSchema,
    pub response: PublicGetPerpImpactTwapResponseSchema,
}
impl From<&PublicGetPerpImpactTwapJsonrpcSchema>
for PublicGetPerpImpactTwapJsonrpcSchema {
    fn from(value: &PublicGetPerpImpactTwapJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetPerpImpactTwapParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "end_time",
    "start_time"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    },
    "end_time": {
      "title": "end_time",
      "description": "End time",
      "type": "integer"
    },
    "start_time": {
      "title": "start_time",
      "description": "Start time",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetPerpImpactTwapParamsSchema {
    ///Currency
    pub currency: String,
    ///End time
    pub end_time: i64,
    ///Start time
    pub start_time: i64,
}
impl From<&PublicGetPerpImpactTwapParamsSchema> for PublicGetPerpImpactTwapParamsSchema {
    fn from(value: &PublicGetPerpImpactTwapParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetPerpImpactTwapRequestSchema
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
      "const": "public/get_perp_impact_twap"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetPerpImpactTwapParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetPerpImpactTwapRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetPerpImpactTwapRequestSchemaId>,
    pub method: String,
    pub params: PublicGetPerpImpactTwapParamsSchema,
}
impl From<&PublicGetPerpImpactTwapRequestSchema>
for PublicGetPerpImpactTwapRequestSchema {
    fn from(value: &PublicGetPerpImpactTwapRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetPerpImpactTwapRequestSchemaId
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
pub enum PublicGetPerpImpactTwapRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetPerpImpactTwapRequestSchemaId>
for PublicGetPerpImpactTwapRequestSchemaId {
    fn from(value: &PublicGetPerpImpactTwapRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetPerpImpactTwapRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetPerpImpactTwapRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetPerpImpactTwapRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetPerpImpactTwapRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetPerpImpactTwapRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetPerpImpactTwapRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetPerpImpactTwapResponseSchema
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
      "$ref": "#/definitions/PublicGetPerpImpactTwapResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetPerpImpactTwapResponseSchema {
    pub id: PublicGetPerpImpactTwapResponseSchemaId,
    ///
    pub result: PublicGetPerpImpactTwapResultSchema,
}
impl From<&PublicGetPerpImpactTwapResponseSchema>
for PublicGetPerpImpactTwapResponseSchema {
    fn from(value: &PublicGetPerpImpactTwapResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetPerpImpactTwapResponseSchemaId
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
pub enum PublicGetPerpImpactTwapResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetPerpImpactTwapResponseSchemaId>
for PublicGetPerpImpactTwapResponseSchemaId {
    fn from(value: &PublicGetPerpImpactTwapResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetPerpImpactTwapResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetPerpImpactTwapResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetPerpImpactTwapResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetPerpImpactTwapResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetPerpImpactTwapResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetPerpImpactTwapResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetPerpImpactTwapResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "ask_impact_diff_twap",
    "bid_impact_diff_twap",
    "currency",
    "mid_price_diff_twap"
  ],
  "properties": {
    "ask_impact_diff_twap": {
      "title": "ask_impact_diff_twap",
      "description": "TWAP of the difference of ask impact price and spot price",
      "type": "string",
      "format": "decimal"
    },
    "bid_impact_diff_twap": {
      "title": "bid_impact_diff_twap",
      "description": "TWAP of the difference of bid impact price and spot price",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "Currency",
      "type": "string"
    },
    "mid_price_diff_twap": {
      "title": "mid_price_diff_twap",
      "description": "TWAP of the difference of mid price and spot price",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetPerpImpactTwapResultSchema {
    ///TWAP of the difference of ask impact price and spot price
    pub ask_impact_diff_twap: bigdecimal::BigDecimal,
    ///TWAP of the difference of bid impact price and spot price
    pub bid_impact_diff_twap: bigdecimal::BigDecimal,
    ///Currency
    pub currency: String,
    ///TWAP of the difference of mid price and spot price
    pub mid_price_diff_twap: bigdecimal::BigDecimal,
}
impl From<&PublicGetPerpImpactTwapResultSchema> for PublicGetPerpImpactTwapResultSchema {
    fn from(value: &PublicGetPerpImpactTwapResultSchema) -> Self {
        value.clone()
    }
}
