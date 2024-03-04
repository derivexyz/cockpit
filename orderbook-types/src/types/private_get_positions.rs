#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///`erc20`, `option`, or `perp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "instrument_type",
  "description": "`erc20`, `option`, or `perp`",
  "type": "string",
  "enum": [
    "erc20",
    "option",
    "perp"
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
pub enum InstrumentType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}
impl From<&InstrumentType> for InstrumentType {
    fn from(value: &InstrumentType) -> Self {
        value.clone()
    }
}
impl ToString for InstrumentType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}
impl std::str::FromStr for InstrumentType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "erc20" => Ok(Self::Erc20),
            "option" => Ok(Self::Option),
            "perp" => Ok(Self::Perp),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///PositionResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "average_price",
    "creation_timestamp",
    "cumulative_funding",
    "delta",
    "gamma",
    "index_price",
    "initial_margin",
    "instrument_name",
    "instrument_type",
    "leverage",
    "liquidation_price",
    "maintenance_margin",
    "mark_price",
    "mark_value",
    "net_settlements",
    "open_orders_margin",
    "pending_funding",
    "realized_pnl",
    "theta",
    "unrealized_pnl",
    "vega"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Position amount held by subaccount",
      "type": "string",
      "format": "decimal"
    },
    "average_price": {
      "title": "average_price",
      "description": "Average price of whole position",
      "type": "string",
      "format": "decimal"
    },
    "creation_timestamp": {
      "title": "creation_timestamp",
      "description": "Timestamp of when the position was opened (in ms since Unix epoch)",
      "type": "integer"
    },
    "cumulative_funding": {
      "title": "cumulative_funding",
      "description": "Cumulative funding for the position (only for perpetuals).",
      "type": "string",
      "format": "decimal"
    },
    "delta": {
      "title": "delta",
      "description": "Asset delta (w.r.t. forward price for options, `1.0` for perps)",
      "type": "string",
      "format": "decimal"
    },
    "gamma": {
      "title": "gamma",
      "description": "Asset gamma (zero for non-options)",
      "type": "string",
      "format": "decimal"
    },
    "index_price": {
      "title": "index_price",
      "description": "Current index (oracle) price for position's currency",
      "type": "string",
      "format": "decimal"
    },
    "initial_margin": {
      "title": "initial_margin",
      "description": "USD initial margin requirement for this position",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name (same as the base Asset name)",
      "type": "string"
    },
    "instrument_type": {
      "title": "instrument_type",
      "description": "`erc20`, `option`, or `perp`",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "leverage": {
      "title": "leverage",
      "description": "Only for perps. Leverage of the position, defined as `abs(notional) / collateral net of options margin`",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "liquidation_price": {
      "title": "liquidation_price",
      "description": "Index price at which position will be liquidated",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "USD maintenance margin requirement for this position",
      "type": "string",
      "format": "decimal"
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Current mark price for position's instrument",
      "type": "string",
      "format": "decimal"
    },
    "mark_value": {
      "title": "mark_value",
      "description": "USD value of the position; this represents how much USD can be recieved by fully closing the position at the current oracle price",
      "type": "string",
      "format": "decimal"
    },
    "net_settlements": {
      "title": "net_settlements",
      "description": "Net amount of USD from position settlements that has been paid to the user's subaccount. This number is subtracted from the portfolio value for margin calculations purposes.<br />Positive values mean the user has recieved USD from settlements, or is awaiting settlement of USD losses. Negative values mean the user has paid USD for settlements, or is awaiting settlement of USD gains.",
      "type": "string",
      "format": "decimal"
    },
    "open_orders_margin": {
      "title": "open_orders_margin",
      "description": "USD margin requirement for all open orders for this asset / instrument",
      "type": "string",
      "format": "decimal"
    },
    "pending_funding": {
      "title": "pending_funding",
      "description": "A portion of funding payments that has not yet been settled into cash balance (only for perpetuals). This number is added to the portfolio value for margin calculations purposes.",
      "type": "string",
      "format": "decimal"
    },
    "realized_pnl": {
      "title": "realized_pnl",
      "description": "Realized trading profit or loss of the position.",
      "type": "string",
      "format": "decimal"
    },
    "theta": {
      "title": "theta",
      "description": "Asset theta (zero for non-options)",
      "type": "string",
      "format": "decimal"
    },
    "unrealized_pnl": {
      "title": "unrealized_pnl",
      "description": "Unrealized trading profit or loss of the position.",
      "type": "string",
      "format": "decimal"
    },
    "vega": {
      "title": "vega",
      "description": "Asset vega (zero for non-options)",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PositionResponseSchema {
    ///Position amount held by subaccount
    pub amount: bigdecimal::BigDecimal,
    ///Average price of whole position
    pub average_price: bigdecimal::BigDecimal,
    ///Timestamp of when the position was opened (in ms since Unix epoch)
    pub creation_timestamp: i64,
    ///Cumulative funding for the position (only for perpetuals).
    pub cumulative_funding: bigdecimal::BigDecimal,
    ///Asset delta (w.r.t. forward price for options, `1.0` for perps)
    pub delta: bigdecimal::BigDecimal,
    ///Asset gamma (zero for non-options)
    pub gamma: bigdecimal::BigDecimal,
    ///Current index (oracle) price for position's currency
    pub index_price: bigdecimal::BigDecimal,
    ///USD initial margin requirement for this position
    pub initial_margin: bigdecimal::BigDecimal,
    ///Instrument name (same as the base Asset name)
    pub instrument_name: String,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
    ///Only for perps. Leverage of the position, defined as `abs(notional) / collateral net of options margin`
    pub leverage: Option<bigdecimal::BigDecimal>,
    ///Index price at which position will be liquidated
    pub liquidation_price: Option<bigdecimal::BigDecimal>,
    ///USD maintenance margin requirement for this position
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Current mark price for position's instrument
    pub mark_price: bigdecimal::BigDecimal,
    ///USD value of the position; this represents how much USD can be recieved by fully closing the position at the current oracle price
    pub mark_value: bigdecimal::BigDecimal,
    ///Net amount of USD from position settlements that has been paid to the user's subaccount. This number is subtracted from the portfolio value for margin calculations purposes.<br />Positive values mean the user has recieved USD from settlements, or is awaiting settlement of USD losses. Negative values mean the user has paid USD for settlements, or is awaiting settlement of USD gains.
    pub net_settlements: bigdecimal::BigDecimal,
    ///USD margin requirement for all open orders for this asset / instrument
    pub open_orders_margin: bigdecimal::BigDecimal,
    ///A portion of funding payments that has not yet been settled into cash balance (only for perpetuals). This number is added to the portfolio value for margin calculations purposes.
    pub pending_funding: bigdecimal::BigDecimal,
    ///Realized trading profit or loss of the position.
    pub realized_pnl: bigdecimal::BigDecimal,
    ///Asset theta (zero for non-options)
    pub theta: bigdecimal::BigDecimal,
    ///Unrealized trading profit or loss of the position.
    pub unrealized_pnl: bigdecimal::BigDecimal,
    ///Asset vega (zero for non-options)
    pub vega: bigdecimal::BigDecimal,
}
impl From<&PositionResponseSchema> for PositionResponseSchema {
    fn from(value: &PositionResponseSchema) -> Self {
        value.clone()
    }
}
///Get active positions of a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_positions",
  "description": "Get active positions of a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetPositionsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetPositions(pub PrivateGetPositionsJsonrpcSchema);
impl std::ops::Deref for PrivateGetPositions {
    type Target = PrivateGetPositionsJsonrpcSchema;
    fn deref(&self) -> &PrivateGetPositionsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetPositions> for PrivateGetPositionsJsonrpcSchema {
    fn from(value: PrivateGetPositions) -> Self {
        value.0
    }
}
impl From<&PrivateGetPositions> for PrivateGetPositions {
    fn from(value: &PrivateGetPositions) -> Self {
        value.clone()
    }
}
impl From<PrivateGetPositionsJsonrpcSchema> for PrivateGetPositions {
    fn from(value: PrivateGetPositionsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetPositionsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetPositionsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetPositionsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetPositionsJsonrpcSchema {
    pub request: PrivateGetPositionsRequestSchema,
    pub response: PrivateGetPositionsResponseSchema,
}
impl From<&PrivateGetPositionsJsonrpcSchema> for PrivateGetPositionsJsonrpcSchema {
    fn from(value: &PrivateGetPositionsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetPositionsParamsSchema
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
      "description": "Subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetPositionsParamsSchema {
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetPositionsParamsSchema> for PrivateGetPositionsParamsSchema {
    fn from(value: &PrivateGetPositionsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetPositionsRequestSchema
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
      "const": "private/get_positions"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetPositionsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetPositionsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetPositionsRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetPositionsParamsSchema,
}
impl From<&PrivateGetPositionsRequestSchema> for PrivateGetPositionsRequestSchema {
    fn from(value: &PrivateGetPositionsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetPositionsRequestSchemaId
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
pub enum PrivateGetPositionsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetPositionsRequestSchemaId> for PrivateGetPositionsRequestSchemaId {
    fn from(value: &PrivateGetPositionsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetPositionsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetPositionsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetPositionsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetPositionsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetPositionsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetPositionsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetPositionsResponseSchema
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
      "$ref": "#/definitions/PrivateGetPositionsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetPositionsResponseSchema {
    pub id: PrivateGetPositionsResponseSchemaId,
    ///
    pub result: PrivateGetPositionsResultSchema,
}
impl From<&PrivateGetPositionsResponseSchema> for PrivateGetPositionsResponseSchema {
    fn from(value: &PrivateGetPositionsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetPositionsResponseSchemaId
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
pub enum PrivateGetPositionsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetPositionsResponseSchemaId> for PrivateGetPositionsResponseSchemaId {
    fn from(value: &PrivateGetPositionsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetPositionsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetPositionsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetPositionsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetPositionsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetPositionsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetPositionsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetPositionsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "positions",
    "subaccount_id"
  ],
  "properties": {
    "positions": {
      "title": "positions",
      "description": "All active positions of subaccount",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/PositionResponseSchema",
        "field_many": true
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

pub struct PrivateGetPositionsResultSchema {
    ///All active positions of subaccount
    pub positions: Vec<PositionResponseSchema>,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetPositionsResultSchema> for PrivateGetPositionsResultSchema {
    fn from(value: &PrivateGetPositionsResultSchema) -> Self {
        value.clone()
    }
}
