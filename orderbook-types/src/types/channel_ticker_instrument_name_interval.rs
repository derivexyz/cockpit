#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///AggregateTradingStatsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "contract_volume",
    "high",
    "low",
    "num_trades",
    "open_interest",
    "percent_change",
    "usd_change"
  ],
  "properties": {
    "contract_volume": {
      "title": "contract_volume",
      "description": "Number of contracts traded during last 24 hours",
      "type": "string",
      "format": "decimal"
    },
    "high": {
      "title": "high",
      "description": "Highest trade price during last 24h",
      "type": "string",
      "format": "decimal"
    },
    "low": {
      "title": "low",
      "description": "Lowest trade price during last 24h",
      "type": "string",
      "format": "decimal"
    },
    "num_trades": {
      "title": "num_trades",
      "description": "Number of trades during last 24h ",
      "type": "string",
      "format": "decimal"
    },
    "open_interest": {
      "title": "open_interest",
      "description": "Current total open interest",
      "type": "string",
      "format": "decimal"
    },
    "percent_change": {
      "title": "percent_change",
      "description": "24-hour price change expressed as a percentage. Options: percent change in vol; Perps: percent change in mark price",
      "type": "string",
      "format": "decimal"
    },
    "usd_change": {
      "title": "usd_change",
      "description": "24-hour price change in USD.",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AggregateTradingStatsSchema {
    ///Number of contracts traded during last 24 hours
    pub contract_volume: bigdecimal::BigDecimal,
    ///Highest trade price during last 24h
    pub high: bigdecimal::BigDecimal,
    ///Lowest trade price during last 24h
    pub low: bigdecimal::BigDecimal,
    ///Number of trades during last 24h
    pub num_trades: bigdecimal::BigDecimal,
    ///Current total open interest
    pub open_interest: bigdecimal::BigDecimal,
    ///24-hour price change expressed as a percentage. Options: percent change in vol; Perps: percent change in mark price
    pub percent_change: bigdecimal::BigDecimal,
    ///24-hour price change in USD.
    pub usd_change: bigdecimal::BigDecimal,
}
impl From<&AggregateTradingStatsSchema> for AggregateTradingStatsSchema {
    fn from(value: &AggregateTradingStatsSchema) -> Self {
        value.clone()
    }
}
///InstrumentTickerSchema
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
    "best_ask_amount",
    "best_ask_price",
    "best_bid_amount",
    "best_bid_price",
    "index_price",
    "instrument_name",
    "instrument_type",
    "is_active",
    "maker_fee_rate",
    "mark_price",
    "max_price",
    "maximum_amount",
    "min_price",
    "minimum_amount",
    "option_details",
    "option_pricing",
    "perp_details",
    "quote_currency",
    "scheduled_activation",
    "scheduled_deactivation",
    "stats",
    "taker_fee_rate",
    "tick_size",
    "timestamp"
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
    "best_ask_amount": {
      "title": "best_ask_amount",
      "description": "Amount of contracts / tokens available at best ask price",
      "type": "string",
      "format": "decimal"
    },
    "best_ask_price": {
      "title": "best_ask_price",
      "description": "Best ask price",
      "type": "string",
      "format": "decimal"
    },
    "best_bid_amount": {
      "title": "best_bid_amount",
      "description": "Amount of contracts / tokens available at best bid price",
      "type": "string",
      "format": "decimal"
    },
    "best_bid_price": {
      "title": "best_bid_price",
      "description": "Best bid price",
      "type": "string",
      "format": "decimal"
    },
    "index_price": {
      "title": "index_price",
      "description": "Index price",
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
    "mark_price": {
      "title": "mark_price",
      "description": "Mark price",
      "type": "string",
      "format": "decimal"
    },
    "mark_price_fee_rate_cap": {
      "title": "mark_price_fee_rate_cap",
      "description": "Percent of option price fee cap, e.g. 12.5%, null if not applicable",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "max_price": {
      "title": "max_price",
      "description": "Maximum price at which an agressive buyer can be matched. Any portion of a market order that would execute above this price will be cancelled. A limit buy order with limit price above this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).",
      "type": "string",
      "format": "decimal"
    },
    "maximum_amount": {
      "title": "maximum_amount",
      "description": "Maximum valid amount of contracts / tokens per trade",
      "type": "string",
      "format": "decimal"
    },
    "min_price": {
      "title": "min_price",
      "description": "Minimum price at which an agressive seller can be matched. Any portion of a market order that would execute below this price will be cancelled. A limit sell order with limit price below this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).",
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
    "option_pricing": {
      "description": "Greeks, forward price, iv and mark price of the instrument (options only)",
      "oneOf": [
        {
          "description": "Greeks, forward price, iv and mark price of the instrument (options only)",
          "default": {},
          "type": "object",
          "$ref": "#/definitions/OptionPricingSchema",
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
    "stats": {
      "description": "Aggregate trading stats for the last 24 hours",
      "type": "object",
      "$ref": "#/definitions/AggregateTradingStatsSchema",
      "field_many": false
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
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the ticker feed snapshot",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct InstrumentTickerSchema {
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
    ///Amount of contracts / tokens available at best ask price
    pub best_ask_amount: bigdecimal::BigDecimal,
    ///Best ask price
    pub best_ask_price: bigdecimal::BigDecimal,
    ///Amount of contracts / tokens available at best bid price
    pub best_bid_amount: bigdecimal::BigDecimal,
    ///Best bid price
    pub best_bid_price: bigdecimal::BigDecimal,
    ///Index price
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
    ///If `True`: instrument is tradeable within `activation` and `deactivation` timestamps
    pub is_active: bool,
    ///Percent of spot price fee rate for makers
    pub maker_fee_rate: bigdecimal::BigDecimal,
    ///Mark price
    pub mark_price: bigdecimal::BigDecimal,
    ///Percent of option price fee cap, e.g. 12.5%, null if not applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_price_fee_rate_cap: Option<bigdecimal::BigDecimal>,
    ///Maximum price at which an agressive buyer can be matched. Any portion of a market order that would execute above this price will be cancelled. A limit buy order with limit price above this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).
    pub max_price: bigdecimal::BigDecimal,
    ///Maximum valid amount of contracts / tokens per trade
    pub maximum_amount: bigdecimal::BigDecimal,
    ///Minimum price at which an agressive seller can be matched. Any portion of a market order that would execute below this price will be cancelled. A limit sell order with limit price below this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).
    pub min_price: bigdecimal::BigDecimal,
    ///Minimum valid amount of contracts / tokens per trade
    pub minimum_amount: bigdecimal::BigDecimal,
    ///Details of the option asset (if applicable)
    pub option_details: Option<OptionPublicDetailsSchema>,
    ///Greeks, forward price, iv and mark price of the instrument (options only)
    pub option_pricing: Option<OptionPricingSchema>,
    ///Details of the perp asset (if applicable)
    pub perp_details: Option<PerpPublicDetailsSchema>,
    ///Quote currency (`USD` for perps, `USDC` for options)
    pub quote_currency: String,
    ///Timestamp at which became or will become active (if applicable)
    pub scheduled_activation: i64,
    ///Scheduled deactivation time for instrument (if applicable)
    pub scheduled_deactivation: i64,
    ///Aggregate trading stats for the last 24 hours
    pub stats: AggregateTradingStatsSchema,
    ///Percent of spot price fee rate for takers
    pub taker_fee_rate: bigdecimal::BigDecimal,
    ///Tick size of the instrument, i.e. minimum price increment
    pub tick_size: bigdecimal::BigDecimal,
    ///Timestamp of the ticker feed snapshot
    pub timestamp: i64,
}
impl From<&InstrumentTickerSchema> for InstrumentTickerSchema {
    fn from(value: &InstrumentTickerSchema) -> Self {
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
///Interval in milliseconds
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "interval",
  "description": "Interval in milliseconds",
  "type": "string",
  "enum": [
    "100",
    "1000"
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
pub enum Interval {
    #[serde(rename = "100")]
    _100,
    #[serde(rename = "1000")]
    _1000,
}
impl From<&Interval> for Interval {
    fn from(value: &Interval) -> Self {
        value.clone()
    }
}
impl ToString for Interval {
    fn to_string(&self) -> String {
        match *self {
            Self::_100 => "100".to_string(),
            Self::_1000 => "1000".to_string(),
        }
    }
}
impl std::str::FromStr for Interval {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "100" => Ok(Self::_100),
            "1000" => Ok(Self::_1000),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Interval {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Interval {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Interval {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///OptionPricingSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "ask_iv",
    "bid_iv",
    "delta",
    "forward_price",
    "gamma",
    "iv",
    "mark_price",
    "rho",
    "theta",
    "vega"
  ],
  "properties": {
    "ask_iv": {
      "title": "ask_iv",
      "description": "Implied volatility of the current best ask",
      "type": "string",
      "format": "decimal"
    },
    "bid_iv": {
      "title": "bid_iv",
      "description": "Implied volatility of the current best bid",
      "type": "string",
      "format": "decimal"
    },
    "delta": {
      "title": "delta",
      "description": "Delta of the option",
      "type": "string",
      "format": "decimal"
    },
    "forward_price": {
      "title": "forward_price",
      "description": "Forward price used to calculate option premium",
      "type": "string",
      "format": "decimal"
    },
    "gamma": {
      "title": "gamma",
      "description": "Gamma of the option",
      "type": "string",
      "format": "decimal"
    },
    "iv": {
      "title": "iv",
      "description": "Implied volatility of the option",
      "type": "string",
      "format": "decimal"
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Mark price of the option",
      "type": "string",
      "format": "decimal"
    },
    "rho": {
      "title": "rho",
      "description": "Rho of the option",
      "type": "string",
      "format": "decimal"
    },
    "theta": {
      "title": "theta",
      "description": "Theta of the option",
      "type": "string",
      "format": "decimal"
    },
    "vega": {
      "title": "vega",
      "description": "Vega of the option",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OptionPricingSchema {
    ///Implied volatility of the current best ask
    pub ask_iv: bigdecimal::BigDecimal,
    ///Implied volatility of the current best bid
    pub bid_iv: bigdecimal::BigDecimal,
    ///Delta of the option
    pub delta: bigdecimal::BigDecimal,
    ///Forward price used to calculate option premium
    pub forward_price: bigdecimal::BigDecimal,
    ///Gamma of the option
    pub gamma: bigdecimal::BigDecimal,
    ///Implied volatility of the option
    pub iv: bigdecimal::BigDecimal,
    ///Mark price of the option
    pub mark_price: bigdecimal::BigDecimal,
    ///Rho of the option
    pub rho: bigdecimal::BigDecimal,
    ///Theta of the option
    pub theta: bigdecimal::BigDecimal,
    ///Vega of the option
    pub vega: bigdecimal::BigDecimal,
}
impl From<&OptionPricingSchema> for OptionPricingSchema {
    fn from(value: &OptionPricingSchema) -> Self {
        value.clone()
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
      "default": null,
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
///
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "ticker.{instrument_name}.{interval}",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/TickerInstrumentNameIntervalPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerInstrumentNameInterval(pub TickerInstrumentNameIntervalPubSubSchema);
impl std::ops::Deref for TickerInstrumentNameInterval {
    type Target = TickerInstrumentNameIntervalPubSubSchema;
    fn deref(&self) -> &TickerInstrumentNameIntervalPubSubSchema {
        &self.0
    }
}
impl From<TickerInstrumentNameInterval> for TickerInstrumentNameIntervalPubSubSchema {
    fn from(value: TickerInstrumentNameInterval) -> Self {
        value.0
    }
}
impl From<&TickerInstrumentNameInterval> for TickerInstrumentNameInterval {
    fn from(value: &TickerInstrumentNameInterval) -> Self {
        value.clone()
    }
}
impl From<TickerInstrumentNameIntervalPubSubSchema> for TickerInstrumentNameInterval {
    fn from(value: TickerInstrumentNameIntervalPubSubSchema) -> Self {
        Self(value)
    }
}
///TickerInstrumentNameIntervalChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_name",
    "interval"
  ],
  "properties": {
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "interval": {
      "title": "interval",
      "description": "Interval in milliseconds",
      "type": "string",
      "enum": [
        "100",
        "1000"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TickerInstrumentNameIntervalChannelSchema {
    ///Instrument name
    pub instrument_name: String,
    ///Interval in milliseconds
    pub interval: Interval,
}
impl From<&TickerInstrumentNameIntervalChannelSchema>
for TickerInstrumentNameIntervalChannelSchema {
    fn from(value: &TickerInstrumentNameIntervalChannelSchema) -> Self {
        value.clone()
    }
}
///TickerInstrumentNameIntervalNotificationParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel",
    "data"
  ],
  "properties": {
    "channel": {
      "title": "channel",
      "description": "Subscribed channel name",
      "type": "string"
    },
    "data": {
      "type": "object",
      "$ref": "#/definitions/TickerInstrumentNameIntervalPublisherDataSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TickerInstrumentNameIntervalNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: TickerInstrumentNameIntervalPublisherDataSchema,
}
impl From<&TickerInstrumentNameIntervalNotificationParamsSchema>
for TickerInstrumentNameIntervalNotificationParamsSchema {
    fn from(value: &TickerInstrumentNameIntervalNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///TickerInstrumentNameIntervalNotificationSchema
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
    "method": {
      "title": "method",
      "type": "string"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/TickerInstrumentNameIntervalNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TickerInstrumentNameIntervalNotificationSchema {
    pub method: String,
    pub params: TickerInstrumentNameIntervalNotificationParamsSchema,
}
impl From<&TickerInstrumentNameIntervalNotificationSchema>
for TickerInstrumentNameIntervalNotificationSchema {
    fn from(value: &TickerInstrumentNameIntervalNotificationSchema) -> Self {
        value.clone()
    }
}
///TickerInstrumentNameIntervalPubSubSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel_params",
    "notification"
  ],
  "properties": {
    "channel_params": {
      "type": "object",
      "$ref": "#/definitions/TickerInstrumentNameIntervalChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/TickerInstrumentNameIntervalNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TickerInstrumentNameIntervalPubSubSchema {
    pub channel_params: TickerInstrumentNameIntervalChannelSchema,
    pub notification: TickerInstrumentNameIntervalNotificationSchema,
}
impl From<&TickerInstrumentNameIntervalPubSubSchema>
for TickerInstrumentNameIntervalPubSubSchema {
    fn from(value: &TickerInstrumentNameIntervalPubSubSchema) -> Self {
        value.clone()
    }
}
///TickerInstrumentNameIntervalPublisherDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_ticker",
    "timestamp"
  ],
  "properties": {
    "instrument_ticker": {
      "description": "Instrument of the ticker feed snapshot",
      "type": "object",
      "$ref": "#/definitions/InstrumentTickerSchema",
      "field_many": false
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the ticker feed snapshot",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TickerInstrumentNameIntervalPublisherDataSchema {
    ///Instrument of the ticker feed snapshot
    pub instrument_ticker: InstrumentTickerSchema,
    ///Timestamp of the ticker feed snapshot
    pub timestamp: i64,
}
impl From<&TickerInstrumentNameIntervalPublisherDataSchema>
for TickerInstrumentNameIntervalPublisherDataSchema {
    fn from(value: &TickerInstrumentNameIntervalPublisherDataSchema) -> Self {
        value.clone()
    }
}
