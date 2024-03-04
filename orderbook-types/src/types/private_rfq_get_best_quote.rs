#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Cancel reason, if any
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "cancel_reason",
  "description": "Cancel reason, if any",
  "type": "string",
  "enum": [
    "",
    "user_request",
    "insufficient_margin",
    "signed_max_fee_too_low",
    "mmp_trigger",
    "cancel_on_disconnect",
    "session_key_deregistered",
    "subaccount_withdrawn",
    "rfq_no_longer_open",
    "compliance"
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
pub enum CancelReason {
    #[serde(rename = "")]
    X,
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "insufficient_margin")]
    InsufficientMargin,
    #[serde(rename = "signed_max_fee_too_low")]
    SignedMaxFeeTooLow,
    #[serde(rename = "mmp_trigger")]
    MmpTrigger,
    #[serde(rename = "cancel_on_disconnect")]
    CancelOnDisconnect,
    #[serde(rename = "session_key_deregistered")]
    SessionKeyDeregistered,
    #[serde(rename = "subaccount_withdrawn")]
    SubaccountWithdrawn,
    #[serde(rename = "rfq_no_longer_open")]
    RfqNoLongerOpen,
    #[serde(rename = "compliance")]
    Compliance,
}
impl From<&CancelReason> for CancelReason {
    fn from(value: &CancelReason) -> Self {
        value.clone()
    }
}
impl ToString for CancelReason {
    fn to_string(&self) -> String {
        match *self {
            Self::X => "".to_string(),
            Self::UserRequest => "user_request".to_string(),
            Self::InsufficientMargin => "insufficient_margin".to_string(),
            Self::SignedMaxFeeTooLow => "signed_max_fee_too_low".to_string(),
            Self::MmpTrigger => "mmp_trigger".to_string(),
            Self::CancelOnDisconnect => "cancel_on_disconnect".to_string(),
            Self::SessionKeyDeregistered => "session_key_deregistered".to_string(),
            Self::SubaccountWithdrawn => "subaccount_withdrawn".to_string(),
            Self::RfqNoLongerOpen => "rfq_no_longer_open".to_string(),
            Self::Compliance => "compliance".to_string(),
        }
    }
}
impl std::str::FromStr for CancelReason {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "" => Ok(Self::X),
            "user_request" => Ok(Self::UserRequest),
            "insufficient_margin" => Ok(Self::InsufficientMargin),
            "signed_max_fee_too_low" => Ok(Self::SignedMaxFeeTooLow),
            "mmp_trigger" => Ok(Self::MmpTrigger),
            "cancel_on_disconnect" => Ok(Self::CancelOnDisconnect),
            "session_key_deregistered" => Ok(Self::SessionKeyDeregistered),
            "subaccount_withdrawn" => Ok(Self::SubaccountWithdrawn),
            "rfq_no_longer_open" => Ok(Self::RfqNoLongerOpen),
            "compliance" => Ok(Self::Compliance),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for CancelReason {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for CancelReason {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for CancelReason {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Leg direction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "direction",
  "description": "Leg direction",
  "type": "string",
  "enum": [
    "buy",
    "sell"
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
pub enum Direction {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl From<&Direction> for Direction {
    fn from(value: &Direction) -> Self {
        value.clone()
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Self::Buy => "buy".to_string(),
            Self::Sell => "sell".to_string(),
        }
    }
}
impl std::str::FromStr for Direction {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Direction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Direction {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Direction {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Reason for the RFQ being invalid, if any.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "invalid_reason",
  "description": "Reason for the RFQ being invalid, if any.",
  "default": null,
  "type": "string",
  "enum": [
    "Account is currently under maintenance margin requirements, trading is frozen.",
    "This order would cause account to fall under maintenance margin requirements.",
    "Insufficient buying power, only a single risk-reducing open order is allowed.",
    "Insufficient buying power, consider reducing order size.",
    "Insufficient buying power, consider reducing order size or canceling other orders.",
    "Consider canceling other limit orders or using IOC, FOK, or market orders. This order is risk-reducing, but if filled with other open orders, buying power might be insufficient.",
    "Insufficient buying power."
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
pub enum InvalidReason {
    #[serde(
        rename = "Account is currently under maintenance margin requirements, trading is frozen."
    )]
    AccountIsCurrentlyUnderMaintenanceMarginRequirementsTradingIsFrozen,
    #[serde(
        rename = "This order would cause account to fall under maintenance margin requirements."
    )]
    ThisOrderWouldCauseAccountToFallUnderMaintenanceMarginRequirements,
    #[serde(
        rename = "Insufficient buying power, only a single risk-reducing open order is allowed."
    )]
    InsufficientBuyingPowerOnlyASingleRiskReducingOpenOrderIsAllowed,
    #[serde(rename = "Insufficient buying power, consider reducing order size.")]
    InsufficientBuyingPowerConsiderReducingOrderSize,
    #[serde(
        rename = "Insufficient buying power, consider reducing order size or canceling other orders."
    )]
    InsufficientBuyingPowerConsiderReducingOrderSizeOrCancelingOtherOrders,
    #[serde(
        rename = "Consider canceling other limit orders or using IOC, FOK, or market orders. This order is risk-reducing, but if filled with other open orders, buying power might be insufficient."
    )]
    ConsiderCancelingOtherLimitOrdersOrUsingIocFokOrMarketOrdersThisOrderIsRiskReducingButIfFilledWithOtherOpenOrdersBuyingPowerMightBeInsufficient,
    #[serde(rename = "Insufficient buying power.")]
    InsufficientBuyingPower,
}
impl From<&InvalidReason> for InvalidReason {
    fn from(value: &InvalidReason) -> Self {
        value.clone()
    }
}
impl ToString for InvalidReason {
    fn to_string(&self) -> String {
        match *self {
            Self::AccountIsCurrentlyUnderMaintenanceMarginRequirementsTradingIsFrozen => {
                "Account is currently under maintenance margin requirements, trading is frozen."
                    .to_string()
            }
            Self::ThisOrderWouldCauseAccountToFallUnderMaintenanceMarginRequirements => {
                "This order would cause account to fall under maintenance margin requirements."
                    .to_string()
            }
            Self::InsufficientBuyingPowerOnlyASingleRiskReducingOpenOrderIsAllowed => {
                "Insufficient buying power, only a single risk-reducing open order is allowed."
                    .to_string()
            }
            Self::InsufficientBuyingPowerConsiderReducingOrderSize => {
                "Insufficient buying power, consider reducing order size.".to_string()
            }
            Self::InsufficientBuyingPowerConsiderReducingOrderSizeOrCancelingOtherOrders => {
                "Insufficient buying power, consider reducing order size or canceling other orders."
                    .to_string()
            }
            Self::ConsiderCancelingOtherLimitOrdersOrUsingIocFokOrMarketOrdersThisOrderIsRiskReducingButIfFilledWithOtherOpenOrdersBuyingPowerMightBeInsufficient => {
                "Consider canceling other limit orders or using IOC, FOK, or market orders. This order is risk-reducing, but if filled with other open orders, buying power might be insufficient."
                    .to_string()
            }
            Self::InsufficientBuyingPower => "Insufficient buying power.".to_string(),
        }
    }
}
impl std::str::FromStr for InvalidReason {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "Account is currently under maintenance margin requirements, trading is frozen." => {
                Ok(
                    Self::AccountIsCurrentlyUnderMaintenanceMarginRequirementsTradingIsFrozen,
                )
            }
            "This order would cause account to fall under maintenance margin requirements." => {
                Ok(
                    Self::ThisOrderWouldCauseAccountToFallUnderMaintenanceMarginRequirements,
                )
            }
            "Insufficient buying power, only a single risk-reducing open order is allowed." => {
                Ok(
                    Self::InsufficientBuyingPowerOnlyASingleRiskReducingOpenOrderIsAllowed,
                )
            }
            "Insufficient buying power, consider reducing order size." => {
                Ok(Self::InsufficientBuyingPowerConsiderReducingOrderSize)
            }
            "Insufficient buying power, consider reducing order size or canceling other orders." => {
                Ok(
                    Self::InsufficientBuyingPowerConsiderReducingOrderSizeOrCancelingOtherOrders,
                )
            }
            "Consider canceling other limit orders or using IOC, FOK, or market orders. This order is risk-reducing, but if filled with other open orders, buying power might be insufficient." => {
                Ok(
                    Self::ConsiderCancelingOtherLimitOrdersOrUsingIocFokOrMarketOrdersThisOrderIsRiskReducingButIfFilledWithOtherOpenOrdersBuyingPowerMightBeInsufficient,
                )
            }
            "Insufficient buying power." => Ok(Self::InsufficientBuyingPower),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for InvalidReason {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InvalidReason {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InvalidReason {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///LegPricedSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "direction",
    "instrument_name",
    "price"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount in units of the base",
      "type": "string",
      "format": "decimal"
    },
    "direction": {
      "title": "direction",
      "description": "Leg direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "price": {
      "title": "price",
      "description": "Leg price",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct LegPricedSchema {
    ///Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Leg direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
    ///Leg price
    pub price: bigdecimal::BigDecimal,
}
impl From<&LegPricedSchema> for LegPricedSchema {
    fn from(value: &LegPricedSchema) -> Self {
        value.clone()
    }
}
///LegUnpricedSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "direction",
    "instrument_name"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount in units of the base",
      "type": "string",
      "format": "decimal"
    },
    "direction": {
      "title": "direction",
      "description": "Leg direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
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

pub struct LegUnpricedSchema {
    ///Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Leg direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
}
impl From<&LegUnpricedSchema> for LegUnpricedSchema {
    fn from(value: &LegUnpricedSchema) -> Self {
        value.clone()
    }
}
///Liquidity role
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "liquidity_role",
  "description": "Liquidity role",
  "type": "string",
  "enum": [
    "maker",
    "taker"
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
pub enum LiquidityRole {
    #[serde(rename = "maker")]
    Maker,
    #[serde(rename = "taker")]
    Taker,
}
impl From<&LiquidityRole> for LiquidityRole {
    fn from(value: &LiquidityRole) -> Self {
        value.clone()
    }
}
impl ToString for LiquidityRole {
    fn to_string(&self) -> String {
        match *self {
            Self::Maker => "maker".to_string(),
            Self::Taker => "taker".to_string(),
        }
    }
}
impl std::str::FromStr for LiquidityRole {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "maker" => Ok(Self::Maker),
            "taker" => Ok(Self::Taker),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
/**Performs a "dry run" on an RFQ, returning the estimated fee and whether the trade is expected to pass.

Should any exception be raised in the process of evaluating the trade, a standard RPC error will be returned
with the error details.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/rfq_get_best_quote",
  "description": "Performs a \"dry run\" on an RFQ, returning the estimated fee and whether the trade is expected to pass.\n\nShould any exception be raised in the process of evaluating the trade, a standard RPC error will be returned\nwith the error details.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateRfqGetBestQuoteJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateRfqGetBestQuote(pub PrivateRfqGetBestQuoteJsonrpcSchema);
impl std::ops::Deref for PrivateRfqGetBestQuote {
    type Target = PrivateRfqGetBestQuoteJsonrpcSchema;
    fn deref(&self) -> &PrivateRfqGetBestQuoteJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateRfqGetBestQuote> for PrivateRfqGetBestQuoteJsonrpcSchema {
    fn from(value: PrivateRfqGetBestQuote) -> Self {
        value.0
    }
}
impl From<&PrivateRfqGetBestQuote> for PrivateRfqGetBestQuote {
    fn from(value: &PrivateRfqGetBestQuote) -> Self {
        value.clone()
    }
}
impl From<PrivateRfqGetBestQuoteJsonrpcSchema> for PrivateRfqGetBestQuote {
    fn from(value: PrivateRfqGetBestQuoteJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateRfqGetBestQuoteJsonrpcSchema
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
      "$ref": "#/definitions/PrivateRfqGetBestQuoteRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateRfqGetBestQuoteResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateRfqGetBestQuoteJsonrpcSchema {
    pub request: PrivateRfqGetBestQuoteRequestSchema,
    pub response: PrivateRfqGetBestQuoteResponseSchema,
}
impl From<&PrivateRfqGetBestQuoteJsonrpcSchema> for PrivateRfqGetBestQuoteJsonrpcSchema {
    fn from(value: &PrivateRfqGetBestQuoteJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateRfqGetBestQuoteParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "legs",
    "subaccount_id"
  ],
  "properties": {
    "direction": {
      "title": "direction",
      "description": "Planned execution direction (default `buy`)",
      "default": "buy",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the RFQ",
      "default": "",
      "type": "string"
    },
    "legs": {
      "title": "legs",
      "description": "RFQ legs",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/LegUnpricedSchema",
        "field_many": true
      }
    },
    "max_total_cost": {
      "title": "max_total_cost",
      "description": "An optional max total cost for the RFQ. Only used when the RFQ sender executes as buyer. Polling endpoints and channels will ignore quotes where the total cost across all legs is above this value. Positive values mean the RFQ sender expects to pay $, negative mean the RFQ sender expects to receive $.This field is not disclosed to the market makers.",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "min_total_cost": {
      "title": "min_total_cost",
      "description": "An optional min total cost for the RFQ. Only used when the RFQ sender executes as seller. Polling endpoints and channels will ignore quotes where the total cost across all legs is below this value. Positive values mean the RFQ sender expects to receive $, negative mean the RFQ sender expects to pay $.This field is not disclosed to the market makers.",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "rfq_id": {
      "title": "rfq_id",
      "description": "RFQ ID to get best quote for. If not provided, will return estimates based on mark prices",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateRfqGetBestQuoteParamsSchema {
    ///Planned execution direction (default `buy`)
    #[serde(default = "defaults::private_rfq_get_best_quote_params_schema_direction")]
    pub direction: Direction,
    ///Optional user-defined label for the RFQ
    #[serde(default)]
    pub label: String,
    ///RFQ legs
    pub legs: Vec<LegUnpricedSchema>,
    ///An optional max total cost for the RFQ. Only used when the RFQ sender executes as buyer. Polling endpoints and channels will ignore quotes where the total cost across all legs is above this value. Positive values mean the RFQ sender expects to pay $, negative mean the RFQ sender expects to receive $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<bigdecimal::BigDecimal>,
    ///An optional min total cost for the RFQ. Only used when the RFQ sender executes as seller. Polling endpoints and channels will ignore quotes where the total cost across all legs is below this value. Positive values mean the RFQ sender expects to receive $, negative mean the RFQ sender expects to pay $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<bigdecimal::BigDecimal>,
    ///RFQ ID to get best quote for. If not provided, will return estimates based on mark prices
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateRfqGetBestQuoteParamsSchema> for PrivateRfqGetBestQuoteParamsSchema {
    fn from(value: &PrivateRfqGetBestQuoteParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateRfqGetBestQuoteRequestSchema
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
      "const": "private/rfq_get_best_quote"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateRfqGetBestQuoteParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateRfqGetBestQuoteRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateRfqGetBestQuoteRequestSchemaId>,
    pub method: String,
    pub params: PrivateRfqGetBestQuoteParamsSchema,
}
impl From<&PrivateRfqGetBestQuoteRequestSchema> for PrivateRfqGetBestQuoteRequestSchema {
    fn from(value: &PrivateRfqGetBestQuoteRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateRfqGetBestQuoteRequestSchemaId
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
pub enum PrivateRfqGetBestQuoteRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateRfqGetBestQuoteRequestSchemaId>
for PrivateRfqGetBestQuoteRequestSchemaId {
    fn from(value: &PrivateRfqGetBestQuoteRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateRfqGetBestQuoteRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateRfqGetBestQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateRfqGetBestQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateRfqGetBestQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateRfqGetBestQuoteRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateRfqGetBestQuoteRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateRfqGetBestQuoteResponseSchema
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
      "$ref": "#/definitions/PrivateRfqGetBestQuoteResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateRfqGetBestQuoteResponseSchema {
    pub id: PrivateRfqGetBestQuoteResponseSchemaId,
    ///
    pub result: PrivateRfqGetBestQuoteResultSchema,
}
impl From<&PrivateRfqGetBestQuoteResponseSchema>
for PrivateRfqGetBestQuoteResponseSchema {
    fn from(value: &PrivateRfqGetBestQuoteResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateRfqGetBestQuoteResponseSchemaId
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
pub enum PrivateRfqGetBestQuoteResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateRfqGetBestQuoteResponseSchemaId>
for PrivateRfqGetBestQuoteResponseSchemaId {
    fn from(value: &PrivateRfqGetBestQuoteResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateRfqGetBestQuoteResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateRfqGetBestQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateRfqGetBestQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateRfqGetBestQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateRfqGetBestQuoteResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateRfqGetBestQuoteResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateRfqGetBestQuoteResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "best_quote",
    "estimated_fee",
    "estimated_realized_pnl",
    "estimated_total_cost",
    "invalid_reason",
    "is_valid",
    "post_initial_margin",
    "post_liquidation_price",
    "pre_initial_margin",
    "suggested_max_fee"
  ],
  "properties": {
    "best_quote": {
      "description": "Best quote for the RFQ (or null if RFQ is not created yet or quotes do not exist). This object should be used to sign a taker quote and call into `execute_quote` RPC.",
      "oneOf": [
        {
          "description": "Best quote for the RFQ (or null if RFQ is not created yet or quotes do not exist). This object should be used to sign a taker quote and call into `execute_quote` RPC.",
          "default": {
            "tx_hash": null,
            "tx_status": null
          },
          "type": "object",
          "$ref": "#/definitions/QuoteResultPublicSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "estimated_fee": {
      "title": "estimated_fee",
      "description": "An estimate for how much the user will pay in fees ($ for the whole trade).",
      "type": "string",
      "format": "decimal"
    },
    "estimated_realized_pnl": {
      "title": "estimated_realized_pnl",
      "description": "An estimate for the realized PnL of the trade.",
      "type": "string",
      "format": "decimal"
    },
    "estimated_total_cost": {
      "title": "estimated_total_cost",
      "description": "An estimate for the total $ cost of the trade.",
      "type": "string",
      "format": "decimal"
    },
    "invalid_reason": {
      "title": "invalid_reason",
      "description": "Reason for the RFQ being invalid, if any.",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "Account is currently under maintenance margin requirements, trading is frozen.",
        "This order would cause account to fall under maintenance margin requirements.",
        "Insufficient buying power, only a single risk-reducing open order is allowed.",
        "Insufficient buying power, consider reducing order size.",
        "Insufficient buying power, consider reducing order size or canceling other orders.",
        "Consider canceling other limit orders or using IOC, FOK, or market orders. This order is risk-reducing, but if filled with other open orders, buying power might be insufficient.",
        "Insufficient buying power."
      ]
    },
    "is_valid": {
      "title": "is_valid",
      "description": "`True` if RFQ is expected to pass margin requirements.",
      "type": "boolean"
    },
    "post_initial_margin": {
      "title": "post_initial_margin",
      "description": "User's hypothetical margin balance if the trade were to get executed.",
      "type": "string",
      "format": "decimal"
    },
    "post_liquidation_price": {
      "title": "post_liquidation_price",
      "description": "Liquidation price if the trade were to be filled. If both upside and downside liquidation prices exist, returns the closest one to the current index price.",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "pre_initial_margin": {
      "title": "pre_initial_margin",
      "description": "User's initial margin balance before the trade.",
      "type": "string",
      "format": "decimal"
    },
    "suggested_max_fee": {
      "title": "suggested_max_fee",
      "description": "Recommended value for `max_fee` of the trade.",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateRfqGetBestQuoteResultSchema {
    ///Best quote for the RFQ (or null if RFQ is not created yet or quotes do not exist). This object should be used to sign a taker quote and call into `execute_quote` RPC.
    pub best_quote: Option<QuoteResultPublicSchema>,
    ///An estimate for how much the user will pay in fees ($ for the whole trade).
    pub estimated_fee: bigdecimal::BigDecimal,
    ///An estimate for the realized PnL of the trade.
    pub estimated_realized_pnl: bigdecimal::BigDecimal,
    ///An estimate for the total $ cost of the trade.
    pub estimated_total_cost: bigdecimal::BigDecimal,
    ///Reason for the RFQ being invalid, if any.
    pub invalid_reason: Option<InvalidReason>,
    ///`True` if RFQ is expected to pass margin requirements.
    pub is_valid: bool,
    ///User's hypothetical margin balance if the trade were to get executed.
    pub post_initial_margin: bigdecimal::BigDecimal,
    ///Liquidation price if the trade were to be filled. If both upside and downside liquidation prices exist, returns the closest one to the current index price.
    pub post_liquidation_price: Option<bigdecimal::BigDecimal>,
    ///User's initial margin balance before the trade.
    pub pre_initial_margin: bigdecimal::BigDecimal,
    ///Recommended value for `max_fee` of the trade.
    pub suggested_max_fee: bigdecimal::BigDecimal,
}
impl From<&PrivateRfqGetBestQuoteResultSchema> for PrivateRfqGetBestQuoteResultSchema {
    fn from(value: &PrivateRfqGetBestQuoteResultSchema) -> Self {
        value.clone()
    }
}
///QuoteResultPublicSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancel_reason",
    "creation_timestamp",
    "direction",
    "last_update_timestamp",
    "legs",
    "legs_hash",
    "liquidity_role",
    "quote_id",
    "rfq_id",
    "status",
    "subaccount_id",
    "tx_hash",
    "tx_status"
  ],
  "properties": {
    "cancel_reason": {
      "title": "cancel_reason",
      "description": "Cancel reason, if any",
      "type": "string",
      "enum": [
        "",
        "user_request",
        "insufficient_margin",
        "signed_max_fee_too_low",
        "mmp_trigger",
        "cancel_on_disconnect",
        "session_key_deregistered",
        "subaccount_withdrawn",
        "rfq_no_longer_open",
        "compliance"
      ]
    },
    "creation_timestamp": {
      "title": "creation_timestamp",
      "description": "Creation timestamp in ms since Unix epoch",
      "type": "integer"
    },
    "direction": {
      "title": "direction",
      "description": "Quote direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "last_update_timestamp": {
      "title": "last_update_timestamp",
      "description": "Last update timestamp in ms since Unix epoch",
      "type": "integer"
    },
    "legs": {
      "title": "legs",
      "description": "Quote legs",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/LegPricedSchema",
        "field_many": true
      }
    },
    "legs_hash": {
      "title": "legs_hash",
      "description": "Hash of the legs of the best quote to be signed by the taker.",
      "type": "string"
    },
    "liquidity_role": {
      "title": "liquidity_role",
      "description": "Liquidity role",
      "type": "string",
      "enum": [
        "maker",
        "taker"
      ]
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID",
      "type": "string",
      "format": "uuid"
    },
    "rfq_id": {
      "title": "rfq_id",
      "description": "RFQ ID",
      "type": "string",
      "format": "uuid"
    },
    "status": {
      "title": "status",
      "description": "Status",
      "type": "string",
      "enum": [
        "open",
        "filled",
        "cancelled",
        "expired"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Blockchain transaction hash (only for executed quotes)",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status (only for executed quotes)",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "requested",
        "pending",
        "settled",
        "reverted",
        "ignored"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct QuoteResultPublicSchema {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///Quote direction
    pub direction: Direction,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///Quote legs
    pub legs: Vec<LegPricedSchema>,
    ///Hash of the legs of the best quote to be signed by the taker.
    pub legs_hash: String,
    ///Liquidity role
    pub liquidity_role: LiquidityRole,
    ///Quote ID
    pub quote_id: uuid::Uuid,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Status
    pub status: Status,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Blockchain transaction hash (only for executed quotes)
    pub tx_hash: Option<String>,
    ///Blockchain transaction status (only for executed quotes)
    pub tx_status: Option<TxStatus>,
}
impl From<&QuoteResultPublicSchema> for QuoteResultPublicSchema {
    fn from(value: &QuoteResultPublicSchema) -> Self {
        value.clone()
    }
}
///Status
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "Status",
  "type": "string",
  "enum": [
    "open",
    "filled",
    "cancelled",
    "expired"
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
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match *self {
            Self::Open => "open".to_string(),
            Self::Filled => "filled".to_string(),
            Self::Cancelled => "cancelled".to_string(),
            Self::Expired => "expired".to_string(),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "open" => Ok(Self::Open),
            "filled" => Ok(Self::Filled),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Status {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Status {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Status {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Blockchain transaction status (only for executed quotes)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Blockchain transaction status (only for executed quotes)",
  "default": null,
  "type": "string",
  "enum": [
    "requested",
    "pending",
    "settled",
    "reverted",
    "ignored"
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
pub enum TxStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
    #[serde(rename = "ignored")]
    Ignored,
}
impl From<&TxStatus> for TxStatus {
    fn from(value: &TxStatus) -> Self {
        value.clone()
    }
}
impl ToString for TxStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Requested => "requested".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
            Self::Ignored => "ignored".to_string(),
        }
    }
}
impl std::str::FromStr for TxStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "requested" => Ok(Self::Requested),
            "pending" => Ok(Self::Pending),
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            "ignored" => Ok(Self::Ignored),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
pub mod defaults {
    pub(super) fn private_rfq_get_best_quote_params_schema_direction() -> super::Direction {
        super::Direction::Buy
    }
}
