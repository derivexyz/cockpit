#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///InstrumentPublicResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount_step",
    "base_asset_address",
    "base_asset_sub_id",
    "base_currency",
    "base_fee",
    "instrument_name",
    "instrument_type",
    "is_active",
    "maker_fee_rate",
    "maximum_amount",
    "minimum_amount",
    "option_details",
    "perp_details",
    "quote_currency",
    "scheduled_activation",
    "scheduled_deactivation",
    "taker_fee_rate",
    "tick_size"
  ],
  "properties": {
    "amount_step": {
      "title": "amount_step",
      "description": "Minimum valid increment of order amount",
      "type": "string",
      "format": "decimal"
    },
    "base_asset_address": {
      "title": "base_asset_address",
      "description": "Blockchain address of the base asset",
      "type": "string"
    },
    "base_asset_sub_id": {
      "title": "base_asset_sub_id",
      "description": "Sub ID of the specific base asset as defined in Asset.sol",
      "type": "string"
    },
    "base_currency": {
      "title": "base_currency",
      "description": "Underlying currency of base asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "base_fee": {
      "title": "base_fee",
      "description": "$ base fee added to every taker order",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
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
    "is_active": {
      "title": "is_active",
      "description": "If `True`: instrument is tradeable within `activation` and `deactivation` timestamps",
      "type": "boolean"
    },
    "maker_fee_rate": {
      "title": "maker_fee_rate",
      "description": "Percent of spot price fee rate for makers",
      "type": "string",
      "format": "decimal"
    },
    "mark_price_fee_rate_cap": {
      "title": "mark_price_fee_rate_cap",
      "description": "Percent of option price fee cap, e.g. 12.5%, null if not applicable",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "maximum_amount": {
      "title": "maximum_amount",
      "description": "Maximum valid amount of contracts / tokens per trade",
      "type": "string",
      "format": "decimal"
    },
    "minimum_amount": {
      "title": "minimum_amount",
      "description": "Minimum valid amount of contracts / tokens per trade",
      "type": "string",
      "format": "decimal"
    },
    "option_details": {
      "description": "Details of the option asset (if applicable)",
      "oneOf": [
        {
          "description": "Details of the option asset (if applicable)",
          "default": {
            "settlement_price": null
          },
          "type": "object",
          "$ref": "#/definitions/OptionPublicDetailsSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "perp_details": {
      "description": "Details of the perp asset (if applicable)",
      "oneOf": [
        {
          "description": "Details of the perp asset (if applicable)",
          "default": {},
          "type": "object",
          "$ref": "#/definitions/PerpPublicDetailsSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "quote_currency": {
      "title": "quote_currency",
      "description": "Quote currency (`USD` for perps, `USDC` for options)",
      "type": "string"
    },
    "scheduled_activation": {
      "title": "scheduled_activation",
      "description": "Timestamp at which became or will become active (if applicable)",
      "type": "integer"
    },
    "scheduled_deactivation": {
      "title": "scheduled_deactivation",
      "description": "Scheduled deactivation time for instrument (if applicable)",
      "type": "integer"
    },
    "taker_fee_rate": {
      "title": "taker_fee_rate",
      "description": "Percent of spot price fee rate for takers",
      "type": "string",
      "format": "decimal"
    },
    "tick_size": {
      "title": "tick_size",
      "description": "Tick size of the instrument, i.e. minimum price increment",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct InstrumentPublicResponseSchema {
    ///Minimum valid increment of order amount
    pub amount_step: bigdecimal::BigDecimal,
    ///Blockchain address of the base asset
    pub base_asset_address: String,
    ///Sub ID of the specific base asset as defined in Asset.sol
    pub base_asset_sub_id: String,
    ///Underlying currency of base asset (`ETH`, `BTC`, etc)
    pub base_currency: String,
    ///$ base fee added to every taker order
    pub base_fee: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
    ///If `True`: instrument is tradeable within `activation` and `deactivation` timestamps
    pub is_active: bool,
    ///Percent of spot price fee rate for makers
    pub maker_fee_rate: bigdecimal::BigDecimal,
    ///Percent of option price fee cap, e.g. 12.5%, null if not applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_price_fee_rate_cap: Option<bigdecimal::BigDecimal>,
    ///Maximum valid amount of contracts / tokens per trade
    pub maximum_amount: bigdecimal::BigDecimal,
    ///Minimum valid amount of contracts / tokens per trade
    pub minimum_amount: bigdecimal::BigDecimal,
    ///Details of the option asset (if applicable)
    pub option_details: Option<OptionPublicDetailsSchema>,
    ///Details of the perp asset (if applicable)
    pub perp_details: Option<PerpPublicDetailsSchema>,
    ///Quote currency (`USD` for perps, `USDC` for options)
    pub quote_currency: String,
    ///Timestamp at which became or will become active (if applicable)
    pub scheduled_activation: i64,
    ///Scheduled deactivation time for instrument (if applicable)
    pub scheduled_deactivation: i64,
    ///Percent of spot price fee rate for takers
    pub taker_fee_rate: bigdecimal::BigDecimal,
    ///Tick size of the instrument, i.e. minimum price increment
    pub tick_size: bigdecimal::BigDecimal,
}
impl From<&InstrumentPublicResponseSchema> for InstrumentPublicResponseSchema {
    fn from(value: &InstrumentPublicResponseSchema) -> Self {
        value.clone()
    }
}
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
///OptionPublicDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "expiry",
    "index",
    "option_type",
    "strike"
  ],
  "properties": {
    "expiry": {
      "title": "expiry",
      "description": "Unix timestamp of expiry date (in seconds)",
      "type": "integer"
    },
    "index": {
      "title": "index",
      "description": "Underlying settlement price index",
      "type": "string"
    },
    "option_type": {
      "title": "option_type",
      "type": "string",
      "enum": [
        "C",
        "P"
      ]
    },
    "settlement_price": {
      "title": "settlement_price",
      "description": "Settlement price of the option",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "strike": {
      "title": "strike",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OptionPublicDetailsSchema {
    ///Unix timestamp of expiry date (in seconds)
    pub expiry: i64,
    ///Underlying settlement price index
    pub index: String,
    pub option_type: OptionType,
    ///Settlement price of the option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<bigdecimal::BigDecimal>,
    pub strike: bigdecimal::BigDecimal,
}
impl From<&OptionPublicDetailsSchema> for OptionPublicDetailsSchema {
    fn from(value: &OptionPublicDetailsSchema) -> Self {
        value.clone()
    }
}
///OptionType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "option_type",
  "type": "string",
  "enum": [
    "C",
    "P"
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
pub enum OptionType {
    C,
    P,
}
impl From<&OptionType> for OptionType {
    fn from(value: &OptionType) -> Self {
        value.clone()
    }
}
impl ToString for OptionType {
    fn to_string(&self) -> String {
        match *self {
            Self::C => "C".to_string(),
            Self::P => "P".to_string(),
        }
    }
}
impl std::str::FromStr for OptionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "C" => Ok(Self::C),
            "P" => Ok(Self::P),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OptionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OptionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OptionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///PerpPublicDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "aggregate_funding",
    "funding_rate",
    "index",
    "max_rate_per_hour",
    "min_rate_per_hour",
    "static_interest_rate"
  ],
  "properties": {
    "aggregate_funding": {
      "title": "aggregate_funding",
      "description": "Latest aggregated funding as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "funding_rate": {
      "title": "funding_rate",
      "description": "Current hourly funding rate as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "index": {
      "title": "index",
      "description": "Underlying spot price index for funding rate",
      "type": "string"
    },
    "max_rate_per_hour": {
      "title": "max_rate_per_hour",
      "description": "Max rate per hour as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "min_rate_per_hour": {
      "title": "min_rate_per_hour",
      "description": "Min rate per hour as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "static_interest_rate": {
      "title": "static_interest_rate",
      "description": "Static interest rate as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PerpPublicDetailsSchema {
    ///Latest aggregated funding as per `PerpAsset.sol`
    pub aggregate_funding: bigdecimal::BigDecimal,
    ///Current hourly funding rate as per `PerpAsset.sol`
    pub funding_rate: bigdecimal::BigDecimal,
    ///Underlying spot price index for funding rate
    pub index: String,
    ///Max rate per hour as per `PerpAsset.sol`
    pub max_rate_per_hour: bigdecimal::BigDecimal,
    ///Min rate per hour as per `PerpAsset.sol`
    pub min_rate_per_hour: bigdecimal::BigDecimal,
    ///Static interest rate as per `PerpAsset.sol`
    pub static_interest_rate: bigdecimal::BigDecimal,
}
impl From<&PerpPublicDetailsSchema> for PerpPublicDetailsSchema {
    fn from(value: &PerpPublicDetailsSchema) -> Self {
        value.clone()
    }
}
///Get all active instruments for a given `currency` and `type`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_instruments",
  "description": "Get all active instruments for a given `currency` and `type`",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetInstrumentsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetInstruments(pub PublicGetInstrumentsJsonrpcSchema);
impl std::ops::Deref for PublicGetInstruments {
    type Target = PublicGetInstrumentsJsonrpcSchema;
    fn deref(&self) -> &PublicGetInstrumentsJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetInstruments> for PublicGetInstrumentsJsonrpcSchema {
    fn from(value: PublicGetInstruments) -> Self {
        value.0
    }
}
impl From<&PublicGetInstruments> for PublicGetInstruments {
    fn from(value: &PublicGetInstruments) -> Self {
        value.clone()
    }
}
impl From<PublicGetInstrumentsJsonrpcSchema> for PublicGetInstruments {
    fn from(value: PublicGetInstrumentsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetInstrumentsJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetInstrumentsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetInstrumentsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetInstrumentsJsonrpcSchema {
    pub request: PublicGetInstrumentsRequestSchema,
    pub response: PublicGetInstrumentsResponseSchema,
}
impl From<&PublicGetInstrumentsJsonrpcSchema> for PublicGetInstrumentsJsonrpcSchema {
    fn from(value: &PublicGetInstrumentsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetInstrumentsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "expired",
    "instrument_type"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "expired": {
      "title": "expired",
      "description": "If `True`: include expired assets",
      "type": "boolean"
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
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetInstrumentsParamsSchema {
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///If `True`: include expired assets
    pub expired: bool,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
}
impl From<&PublicGetInstrumentsParamsSchema> for PublicGetInstrumentsParamsSchema {
    fn from(value: &PublicGetInstrumentsParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetInstrumentsRequestSchema
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
      "const": "public/get_instruments"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetInstrumentsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetInstrumentsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetInstrumentsRequestSchemaId>,
    pub method: String,
    pub params: PublicGetInstrumentsParamsSchema,
}
impl From<&PublicGetInstrumentsRequestSchema> for PublicGetInstrumentsRequestSchema {
    fn from(value: &PublicGetInstrumentsRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetInstrumentsRequestSchemaId
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
pub enum PublicGetInstrumentsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetInstrumentsRequestSchemaId> for PublicGetInstrumentsRequestSchemaId {
    fn from(value: &PublicGetInstrumentsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetInstrumentsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetInstrumentsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetInstrumentsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetInstrumentsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetInstrumentsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetInstrumentsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetInstrumentsResponseSchema
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
        "$ref": "#/definitions/InstrumentPublicResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetInstrumentsResponseSchema {
    pub id: PublicGetInstrumentsResponseSchemaId,
    ///
    pub result: Vec<InstrumentPublicResponseSchema>,
}
impl From<&PublicGetInstrumentsResponseSchema> for PublicGetInstrumentsResponseSchema {
    fn from(value: &PublicGetInstrumentsResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetInstrumentsResponseSchemaId
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
pub enum PublicGetInstrumentsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetInstrumentsResponseSchemaId>
for PublicGetInstrumentsResponseSchemaId {
    fn from(value: &PublicGetInstrumentsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetInstrumentsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetInstrumentsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetInstrumentsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetInstrumentsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetInstrumentsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetInstrumentsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
