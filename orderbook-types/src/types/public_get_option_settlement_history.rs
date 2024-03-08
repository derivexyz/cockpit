#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///OptionSettlementResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "expiry",
    "instrument_name",
    "option_settlement_pnl",
    "settlement_price",
    "subaccount_id"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount that was settled",
      "type": "string",
      "format": "decimal"
    },
    "expiry": {
      "title": "expiry",
      "description": "Expiry timestamp of the option",
      "type": "integer"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "option_settlement_pnl": {
      "title": "option_settlement_pnl",
      "description": "USD profit or loss from option settlements calculated as: settlement value - (average price x amount)",
      "type": "string",
      "format": "decimal"
    },
    "settlement_price": {
      "title": "settlement_price",
      "description": "Price of option settlement",
      "type": "string",
      "format": "decimal"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID of the settlement event",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OptionSettlementResponseSchema {
    ///Amount that was settled
    pub amount: bigdecimal::BigDecimal,
    ///Expiry timestamp of the option
    pub expiry: i64,
    ///Instrument name
    pub instrument_name: String,
    ///USD profit or loss from option settlements calculated as: settlement value - (average price x amount)
    pub option_settlement_pnl: bigdecimal::BigDecimal,
    ///Price of option settlement
    pub settlement_price: bigdecimal::BigDecimal,
    ///Subaccount ID of the settlement event
    pub subaccount_id: i64,
}
impl From<&OptionSettlementResponseSchema> for OptionSettlementResponseSchema {
    fn from(value: &OptionSettlementResponseSchema) -> Self {
        value.clone()
    }
}
///PaginationInfoSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "count",
    "num_pages"
  ],
  "properties": {
    "count": {
      "title": "count",
      "description": "Total number of items, across all pages",
      "type": "integer"
    },
    "num_pages": {
      "title": "num_pages",
      "description": "Number of pages",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PaginationInfoSchema {
    ///Total number of items, across all pages
    pub count: i64,
    ///Number of pages
    pub num_pages: i64,
}
impl From<&PaginationInfoSchema> for PaginationInfoSchema {
    fn from(value: &PaginationInfoSchema) -> Self {
        value.clone()
    }
}
///Get expired option settlement history for a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_option_settlement_history",
  "description": "Get expired option settlement history for a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetOptionSettlementHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetOptionSettlementHistory(
    pub PublicGetOptionSettlementHistoryJsonrpcSchema,
);
impl std::ops::Deref for PublicGetOptionSettlementHistory {
    type Target = PublicGetOptionSettlementHistoryJsonrpcSchema;
    fn deref(&self) -> &PublicGetOptionSettlementHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetOptionSettlementHistory>
for PublicGetOptionSettlementHistoryJsonrpcSchema {
    fn from(value: PublicGetOptionSettlementHistory) -> Self {
        value.0
    }
}
impl From<&PublicGetOptionSettlementHistory> for PublicGetOptionSettlementHistory {
    fn from(value: &PublicGetOptionSettlementHistory) -> Self {
        value.clone()
    }
}
impl From<PublicGetOptionSettlementHistoryJsonrpcSchema>
for PublicGetOptionSettlementHistory {
    fn from(value: PublicGetOptionSettlementHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetOptionSettlementHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetOptionSettlementHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetOptionSettlementHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetOptionSettlementHistoryJsonrpcSchema {
    pub request: PublicGetOptionSettlementHistoryRequestSchema,
    pub response: PublicGetOptionSettlementHistoryResponseSchema,
}
impl From<&PublicGetOptionSettlementHistoryJsonrpcSchema>
for PublicGetOptionSettlementHistoryJsonrpcSchema {
    fn from(value: &PublicGetOptionSettlementHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetOptionSettlementHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "properties": {
    "page": {
      "title": "page",
      "description": "Page number of results to return (default 1, returns last if above `num_pages`)",
      "default": 1,
      "type": "integer"
    },
    "page_size": {
      "title": "page_size",
      "description": "Number of results per page (default 100, max 1000)",
      "default": 100,
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID filter (defaults to all if not provided)",
      "type": [
        "integer",
        "null"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetOptionSettlementHistoryParamsSchema {
    ///Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page (default 100, max 1000)
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
    ///Subaccount ID filter (defaults to all if not provided)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<i64>,
}
impl From<&PublicGetOptionSettlementHistoryParamsSchema>
for PublicGetOptionSettlementHistoryParamsSchema {
    fn from(value: &PublicGetOptionSettlementHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetOptionSettlementHistoryRequestSchema
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
      "const": "public/get_option_settlement_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetOptionSettlementHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetOptionSettlementHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetOptionSettlementHistoryRequestSchemaId>,
    pub method: String,
    pub params: PublicGetOptionSettlementHistoryParamsSchema,
}
impl From<&PublicGetOptionSettlementHistoryRequestSchema>
for PublicGetOptionSettlementHistoryRequestSchema {
    fn from(value: &PublicGetOptionSettlementHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetOptionSettlementHistoryRequestSchemaId
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
pub enum PublicGetOptionSettlementHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetOptionSettlementHistoryRequestSchemaId>
for PublicGetOptionSettlementHistoryRequestSchemaId {
    fn from(value: &PublicGetOptionSettlementHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetOptionSettlementHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetOptionSettlementHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetOptionSettlementHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetOptionSettlementHistoryResponseSchema
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
      "$ref": "#/definitions/PublicGetOptionSettlementHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetOptionSettlementHistoryResponseSchema {
    pub id: PublicGetOptionSettlementHistoryResponseSchemaId,
    ///
    pub result: PublicGetOptionSettlementHistoryResultSchema,
}
impl From<&PublicGetOptionSettlementHistoryResponseSchema>
for PublicGetOptionSettlementHistoryResponseSchema {
    fn from(value: &PublicGetOptionSettlementHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetOptionSettlementHistoryResponseSchemaId
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
pub enum PublicGetOptionSettlementHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetOptionSettlementHistoryResponseSchemaId>
for PublicGetOptionSettlementHistoryResponseSchemaId {
    fn from(value: &PublicGetOptionSettlementHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetOptionSettlementHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PublicGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetOptionSettlementHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetOptionSettlementHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetOptionSettlementHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "pagination",
    "settlements"
  ],
  "properties": {
    "pagination": {
      "description": "Pagination info",
      "type": "object",
      "$ref": "#/definitions/PaginationInfoSchema",
      "field_many": false
    },
    "settlements": {
      "title": "settlements",
      "description": "List of expired option settlements",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/OptionSettlementResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetOptionSettlementHistoryResultSchema {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///List of expired option settlements
    pub settlements: Vec<OptionSettlementResponseSchema>,
}
impl From<&PublicGetOptionSettlementHistoryResultSchema>
for PublicGetOptionSettlementHistoryResultSchema {
    fn from(value: &PublicGetOptionSettlementHistoryResultSchema) -> Self {
        value.clone()
    }
}
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
