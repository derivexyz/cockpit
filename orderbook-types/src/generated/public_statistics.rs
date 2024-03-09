#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get statistics for a specific instrument or instrument type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/statistics",
  "description": "Get statistics for a specific instrument or instrument type",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicStatisticsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicStatistics(pub PublicStatisticsJsonrpcSchema);
impl std::ops::Deref for PublicStatistics {
    type Target = PublicStatisticsJsonrpcSchema;
    fn deref(&self) -> &PublicStatisticsJsonrpcSchema {
        &self.0
    }
}
impl From<PublicStatistics> for PublicStatisticsJsonrpcSchema {
    fn from(value: PublicStatistics) -> Self {
        value.0
    }
}
impl From<&PublicStatistics> for PublicStatistics {
    fn from(value: &PublicStatistics) -> Self {
        value.clone()
    }
}
impl From<PublicStatisticsJsonrpcSchema> for PublicStatistics {
    fn from(value: PublicStatisticsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicStatisticsJsonrpcSchema
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
      "$ref": "#/definitions/PublicStatisticsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicStatisticsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicStatisticsJsonrpcSchema {
    pub request: PublicStatisticsRequestSchema,
    pub response: PublicStatisticsResponseSchema,
}
impl From<&PublicStatisticsJsonrpcSchema> for PublicStatisticsJsonrpcSchema {
    fn from(value: &PublicStatisticsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicStatisticsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_name"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency for stats",
      "type": [
        "string",
        "null"
      ]
    },
    "end_time": {
      "title": "end_time",
      "description": "End time for statistics",
      "type": [
        "integer",
        "null"
      ]
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name or 'ALL', 'OPTION', 'PERP'",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicStatisticsParamsSchema {
    ///Currency for stats
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///End time for statistics
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    ///Instrument name or 'ALL', 'OPTION', 'PERP'
    pub instrument_name: String,
}
impl From<&PublicStatisticsParamsSchema> for PublicStatisticsParamsSchema {
    fn from(value: &PublicStatisticsParamsSchema) -> Self {
        value.clone()
    }
}
///PublicStatisticsRequestSchema
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
      "const": "public/statistics"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicStatisticsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicStatisticsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicStatisticsRequestSchemaId>,
    pub method: String,
    pub params: PublicStatisticsParamsSchema,
}
impl From<&PublicStatisticsRequestSchema> for PublicStatisticsRequestSchema {
    fn from(value: &PublicStatisticsRequestSchema) -> Self {
        value.clone()
    }
}
///PublicStatisticsRequestSchemaId
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
pub enum PublicStatisticsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicStatisticsRequestSchemaId> for PublicStatisticsRequestSchemaId {
    fn from(value: &PublicStatisticsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicStatisticsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicStatisticsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicStatisticsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicStatisticsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicStatisticsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicStatisticsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicStatisticsResponseSchema
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
      "$ref": "#/definitions/PublicStatisticsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicStatisticsResponseSchema {
    pub id: PublicStatisticsResponseSchemaId,
    ///
    pub result: PublicStatisticsResultSchema,
}
impl From<&PublicStatisticsResponseSchema> for PublicStatisticsResponseSchema {
    fn from(value: &PublicStatisticsResponseSchema) -> Self {
        value.clone()
    }
}
///PublicStatisticsResponseSchemaId
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
pub enum PublicStatisticsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicStatisticsResponseSchemaId> for PublicStatisticsResponseSchemaId {
    fn from(value: &PublicStatisticsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicStatisticsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicStatisticsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicStatisticsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicStatisticsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicStatisticsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicStatisticsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicStatisticsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "daily_fees",
    "daily_notional_volume",
    "daily_premium_volume",
    "daily_trades",
    "open_interest",
    "total_fees",
    "total_notional_volume",
    "total_premium_volume",
    "total_trades"
  ],
  "properties": {
    "daily_fees": {
      "title": "daily_fees",
      "description": "24h Fees",
      "type": "string",
      "format": "decimal"
    },
    "daily_notional_volume": {
      "title": "daily_notional_volume",
      "description": "24h Notional volume",
      "type": "string",
      "format": "decimal"
    },
    "daily_premium_volume": {
      "title": "daily_premium_volume",
      "description": "24h Premium volume",
      "type": "string",
      "format": "decimal"
    },
    "daily_trades": {
      "title": "daily_trades",
      "description": "24h Trades",
      "type": "integer"
    },
    "open_interest": {
      "title": "open_interest",
      "description": "Open interest",
      "type": "string",
      "format": "decimal"
    },
    "total_fees": {
      "title": "total_fees",
      "description": "Total fees",
      "type": "string",
      "format": "decimal"
    },
    "total_notional_volume": {
      "title": "total_notional_volume",
      "description": "Total notional volume",
      "type": "string",
      "format": "decimal"
    },
    "total_premium_volume": {
      "title": "total_premium_volume",
      "description": "Total premium volume",
      "type": "string",
      "format": "decimal"
    },
    "total_trades": {
      "title": "total_trades",
      "description": "Total trades",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicStatisticsResultSchema {
    ///24h Fees
    pub daily_fees: bigdecimal::BigDecimal,
    ///24h Notional volume
    pub daily_notional_volume: bigdecimal::BigDecimal,
    ///24h Premium volume
    pub daily_premium_volume: bigdecimal::BigDecimal,
    ///24h Trades
    pub daily_trades: i64,
    ///Open interest
    pub open_interest: bigdecimal::BigDecimal,
    ///Total fees
    pub total_fees: bigdecimal::BigDecimal,
    ///Total notional volume
    pub total_notional_volume: bigdecimal::BigDecimal,
    ///Total premium volume
    pub total_premium_volume: bigdecimal::BigDecimal,
    ///Total trades
    pub total_trades: i64,
}
impl From<&PublicStatisticsResultSchema> for PublicStatisticsResultSchema {
    fn from(value: &PublicStatisticsResultSchema) -> Self {
        value.clone()
    }
}
