#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///CurrencyResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "spot_price"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "spot_price": {
      "title": "spot_price",
      "description": "Spot price of the currency",
      "type": "string",
      "format": "decimal"
    },
    "spot_price_24h": {
      "title": "spot_price_24h",
      "description": "Spot price of the currency 24 hours ago",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct CurrencyResponseSchema {
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///Spot price of the currency
    pub spot_price: bigdecimal::BigDecimal,
    ///Spot price of the currency 24 hours ago
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spot_price_24h: Option<bigdecimal::BigDecimal>,
}
impl From<&CurrencyResponseSchema> for CurrencyResponseSchema {
    fn from(value: &CurrencyResponseSchema) -> Self {
        value.clone()
    }
}
/**Get all active currencies with their spot price, spot price 24hrs ago.

For real-time updates, recommend using channels -> ticker or orderbook.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_all_currencies",
  "description": "Get all active currencies with their spot price, spot price 24hrs ago.\n\nFor real-time updates, recommend using channels -> ticker or orderbook.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetAllCurrenciesJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetAllCurrencies(pub PublicGetAllCurrenciesJsonrpcSchema);
impl std::ops::Deref for PublicGetAllCurrencies {
    type Target = PublicGetAllCurrenciesJsonrpcSchema;
    fn deref(&self) -> &PublicGetAllCurrenciesJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetAllCurrencies> for PublicGetAllCurrenciesJsonrpcSchema {
    fn from(value: PublicGetAllCurrencies) -> Self {
        value.0
    }
}
impl From<&PublicGetAllCurrencies> for PublicGetAllCurrencies {
    fn from(value: &PublicGetAllCurrencies) -> Self {
        value.clone()
    }
}
impl From<PublicGetAllCurrenciesJsonrpcSchema> for PublicGetAllCurrencies {
    fn from(value: PublicGetAllCurrenciesJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetAllCurrenciesJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetAllCurrenciesRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetAllCurrenciesResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAllCurrenciesJsonrpcSchema {
    pub request: PublicGetAllCurrenciesRequestSchema,
    pub response: PublicGetAllCurrenciesResponseSchema,
}
impl From<&PublicGetAllCurrenciesJsonrpcSchema> for PublicGetAllCurrenciesJsonrpcSchema {
    fn from(value: &PublicGetAllCurrenciesJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetAllCurrenciesParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAllCurrenciesParamsSchema {}
impl From<&PublicGetAllCurrenciesParamsSchema> for PublicGetAllCurrenciesParamsSchema {
    fn from(value: &PublicGetAllCurrenciesParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetAllCurrenciesRequestSchema
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
      "const": "public/get_all_currencies"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetAllCurrenciesParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAllCurrenciesRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetAllCurrenciesRequestSchemaId>,
    pub method: String,
    pub params: PublicGetAllCurrenciesParamsSchema,
}
impl From<&PublicGetAllCurrenciesRequestSchema> for PublicGetAllCurrenciesRequestSchema {
    fn from(value: &PublicGetAllCurrenciesRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetAllCurrenciesRequestSchemaId
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
pub enum PublicGetAllCurrenciesRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetAllCurrenciesRequestSchemaId>
for PublicGetAllCurrenciesRequestSchemaId {
    fn from(value: &PublicGetAllCurrenciesRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetAllCurrenciesRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetAllCurrenciesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetAllCurrenciesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetAllCurrenciesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetAllCurrenciesRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetAllCurrenciesRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetAllCurrenciesResponseSchema
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
      "description": "",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/CurrencyResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAllCurrenciesResponseSchema {
    pub id: PublicGetAllCurrenciesResponseSchemaId,
    ///
    pub result: Vec<CurrencyResponseSchema>,
}
impl From<&PublicGetAllCurrenciesResponseSchema>
for PublicGetAllCurrenciesResponseSchema {
    fn from(value: &PublicGetAllCurrenciesResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetAllCurrenciesResponseSchemaId
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
pub enum PublicGetAllCurrenciesResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetAllCurrenciesResponseSchemaId>
for PublicGetAllCurrenciesResponseSchemaId {
    fn from(value: &PublicGetAllCurrenciesResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetAllCurrenciesResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetAllCurrenciesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetAllCurrenciesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetAllCurrenciesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetAllCurrenciesResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetAllCurrenciesResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
