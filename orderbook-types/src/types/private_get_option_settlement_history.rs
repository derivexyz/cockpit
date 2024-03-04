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
///Get expired option settlement history for a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_option_settlement_history",
  "description": "Get expired option settlement history for a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetOptionSettlementHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetOptionSettlementHistory(
    pub PrivateGetOptionSettlementHistoryJsonrpcSchema,
);
impl std::ops::Deref for PrivateGetOptionSettlementHistory {
    type Target = PrivateGetOptionSettlementHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetOptionSettlementHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetOptionSettlementHistory>
for PrivateGetOptionSettlementHistoryJsonrpcSchema {
    fn from(value: PrivateGetOptionSettlementHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetOptionSettlementHistory> for PrivateGetOptionSettlementHistory {
    fn from(value: &PrivateGetOptionSettlementHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetOptionSettlementHistoryJsonrpcSchema>
for PrivateGetOptionSettlementHistory {
    fn from(value: PrivateGetOptionSettlementHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetOptionSettlementHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetOptionSettlementHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetOptionSettlementHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOptionSettlementHistoryJsonrpcSchema {
    pub request: PrivateGetOptionSettlementHistoryRequestSchema,
    pub response: PrivateGetOptionSettlementHistoryResponseSchema,
}
impl From<&PrivateGetOptionSettlementHistoryJsonrpcSchema>
for PrivateGetOptionSettlementHistoryJsonrpcSchema {
    fn from(value: &PrivateGetOptionSettlementHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOptionSettlementHistoryParamsSchema
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
      "description": "Subaccount ID for which to get expired option settlement history",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOptionSettlementHistoryParamsSchema {
    ///Subaccount ID for which to get expired option settlement history
    pub subaccount_id: i64,
}
impl From<&PrivateGetOptionSettlementHistoryParamsSchema>
for PrivateGetOptionSettlementHistoryParamsSchema {
    fn from(value: &PrivateGetOptionSettlementHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOptionSettlementHistoryRequestSchema
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
      "const": "private/get_option_settlement_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetOptionSettlementHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOptionSettlementHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetOptionSettlementHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetOptionSettlementHistoryParamsSchema,
}
impl From<&PrivateGetOptionSettlementHistoryRequestSchema>
for PrivateGetOptionSettlementHistoryRequestSchema {
    fn from(value: &PrivateGetOptionSettlementHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOptionSettlementHistoryRequestSchemaId
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
pub enum PrivateGetOptionSettlementHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetOptionSettlementHistoryRequestSchemaId>
for PrivateGetOptionSettlementHistoryRequestSchemaId {
    fn from(value: &PrivateGetOptionSettlementHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetOptionSettlementHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PrivateGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetOptionSettlementHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetOptionSettlementHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetOptionSettlementHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetOptionSettlementHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetOptionSettlementHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOptionSettlementHistoryResponseSchema {
    pub id: PrivateGetOptionSettlementHistoryResponseSchemaId,
    ///
    pub result: PrivateGetOptionSettlementHistoryResultSchema,
}
impl From<&PrivateGetOptionSettlementHistoryResponseSchema>
for PrivateGetOptionSettlementHistoryResponseSchema {
    fn from(value: &PrivateGetOptionSettlementHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOptionSettlementHistoryResponseSchemaId
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
pub enum PrivateGetOptionSettlementHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetOptionSettlementHistoryResponseSchemaId>
for PrivateGetOptionSettlementHistoryResponseSchemaId {
    fn from(value: &PrivateGetOptionSettlementHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetOptionSettlementHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String>
for PrivateGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String>
for PrivateGetOptionSettlementHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetOptionSettlementHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetOptionSettlementHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetOptionSettlementHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "settlements",
    "subaccount_id"
  ],
  "properties": {
    "settlements": {
      "title": "settlements",
      "description": "List of expired option settlements",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/OptionSettlementResponseSchema",
        "field_many": true
      }
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to get expired option settlement history",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOptionSettlementHistoryResultSchema {
    ///List of expired option settlements
    pub settlements: Vec<OptionSettlementResponseSchema>,
    ///Subaccount_id for which to get expired option settlement history
    pub subaccount_id: i64,
}
impl From<&PrivateGetOptionSettlementHistoryResultSchema>
for PrivateGetOptionSettlementHistoryResultSchema {
    fn from(value: &PrivateGetOptionSettlementHistoryResultSchema) -> Self {
        value.clone()
    }
}
