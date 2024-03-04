#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Order direction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "direction",
  "description": "Order direction",
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
///An estimate for the order status after submission. Fully filled orders will be marked as `filled`, unfilled and partially filled limit orders will be marked as `open` and partially filled market / IOC orders will be marked as `cancelled`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "estimated_order_status",
  "description": "An estimate for the order status after submission. Fully filled orders will be marked as `filled`, unfilled and partially filled limit orders will be marked as `open` and partially filled market / IOC orders will be marked as `cancelled`.",
  "type": "string",
  "enum": [
    "open",
    "filled",
    "rejected",
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
pub enum EstimatedOrderStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl From<&EstimatedOrderStatus> for EstimatedOrderStatus {
    fn from(value: &EstimatedOrderStatus) -> Self {
        value.clone()
    }
}
impl ToString for EstimatedOrderStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Open => "open".to_string(),
            Self::Filled => "filled".to_string(),
            Self::Rejected => "rejected".to_string(),
            Self::Cancelled => "cancelled".to_string(),
            Self::Expired => "expired".to_string(),
        }
    }
}
impl std::str::FromStr for EstimatedOrderStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "open" => Ok(Self::Open),
            "filled" => Ok(Self::Filled),
            "rejected" => Ok(Self::Rejected),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for EstimatedOrderStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for EstimatedOrderStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for EstimatedOrderStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Reason for the order being invalid, if any.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "invalid_reason",
  "description": "Reason for the order being invalid, if any.",
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
///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "order_type",
  "description": "Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled",
  "default": "limit",
  "type": "string",
  "enum": [
    "limit",
    "market"
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
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}
impl From<&OrderType> for OrderType {
    fn from(value: &OrderType) -> Self {
        value.clone()
    }
}
impl ToString for OrderType {
    fn to_string(&self) -> String {
        match *self {
            Self::Limit => "limit".to_string(),
            Self::Market => "market".to_string(),
        }
    }
}
impl std::str::FromStr for OrderType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "limit" => Ok(Self::Limit),
            "market" => Ok(Self::Market),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrderType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrderType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrderType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}
/**Performs a "risk dry run" on an order, returning the estimated fee and whether the order is expected to pass.

For market orders, the fill is assumed to happen at the supplied limit price. For accurate results the caller
should calculate an expected fill price from orderbook state and add a small buffer to it to account for latency.

Should any exception be raised in the process of evaluating the order, a standard RPC error will be returned
with the error details.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/order_quote",
  "description": "Performs a \"risk dry run\" on an order, returning the estimated fee and whether the order is expected to pass.\n\nFor market orders, the fill is assumed to happen at the supplied limit price. For accurate results the caller\nshould calculate an expected fill price from orderbook state and add a small buffer to it to account for latency.\n\nShould any exception be raised in the process of evaluating the order, a standard RPC error will be returned\nwith the error details.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateOrderQuoteJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateOrderQuote(pub PrivateOrderQuoteJsonrpcSchema);
impl std::ops::Deref for PrivateOrderQuote {
    type Target = PrivateOrderQuoteJsonrpcSchema;
    fn deref(&self) -> &PrivateOrderQuoteJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateOrderQuote> for PrivateOrderQuoteJsonrpcSchema {
    fn from(value: PrivateOrderQuote) -> Self {
        value.0
    }
}
impl From<&PrivateOrderQuote> for PrivateOrderQuote {
    fn from(value: &PrivateOrderQuote) -> Self {
        value.clone()
    }
}
impl From<PrivateOrderQuoteJsonrpcSchema> for PrivateOrderQuote {
    fn from(value: PrivateOrderQuoteJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateOrderQuoteJsonrpcSchema
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
      "$ref": "#/definitions/PrivateOrderQuoteRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateOrderQuoteResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderQuoteJsonrpcSchema {
    pub request: PrivateOrderQuoteRequestSchema,
    pub response: PrivateOrderQuoteResponseSchema,
}
impl From<&PrivateOrderQuoteJsonrpcSchema> for PrivateOrderQuoteJsonrpcSchema {
    fn from(value: &PrivateOrderQuoteJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderQuoteParamsSchema
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
    "limit_price",
    "max_fee",
    "nonce",
    "signature",
    "signature_expiry_sec",
    "signer",
    "subaccount_id"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Order amount in units of the base",
      "type": "string",
      "format": "decimal"
    },
    "direction": {
      "title": "direction",
      "description": "Order direction",
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
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "default": "",
      "type": "string"
    },
    "limit_price": {
      "title": "limit_price",
      "description": "Limit price in quote currency.<br />This field is still required for market orders because it is a component of the signature. However, market orders will not leave a resting order in the book in case of a partial fill.",
      "type": "string",
      "format": "decimal"
    },
    "max_fee": {
      "title": "max_fee",
      "description": "Max fee in units of the quote currency. Order will be rejected if the supplied max fee is below the estimated fee for this order.",
      "type": "string",
      "format": "decimal"
    },
    "mmp": {
      "title": "mmp",
      "description": "Whether the order is tagged for market maker protections (default false)",
      "default": false,
      "type": "boolean"
    },
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "order_type": {
      "title": "order_type",
      "description": "Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled",
      "default": "limit",
      "type": "string",
      "enum": [
        "limit",
        "market"
      ]
    },
    "reduce_only": {
      "title": "reduce_only",
      "description": "If true, the order will not be able to increase position's size (default false). If the order amount exceeds available position size, the order will be filled up to the position size and the remainder will be cancelled. This flag is only supported for market orders or non-resting limit orders (IOC or FOK)",
      "default": false,
      "type": "boolean"
    },
    "referral_code": {
      "title": "referral_code",
      "description": "Optional referral code for the order",
      "default": "",
      "type": "string"
    },
    "reject_timestamp": {
      "title": "reject_timestamp",
      "description": "UTC timestamp in ms, if provided the matching engine will reject the order with an error if `reject_timestamp` < `server_time`. Note that the timestamp must be consistent with the server time: use `public/get_time` method to obtain current server time.",
      "default": 9223372036854775807,
      "type": "integer"
    },
    "replaced_order_id": {
      "title": "replaced_order_id",
      "description": "If replaced, ID of the order that was replaced",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "signature": {
      "title": "signature",
      "description": "Etherium signature of the order",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Owner wallet address or registered session key that signed order",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "time_in_force": {
      "title": "time_in_force",
      "description": "Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.",
      "default": "gtc",
      "type": "string",
      "enum": [
        "gtc",
        "post_only",
        "fok",
        "ioc"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderQuoteParamsSchema {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Order direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
    ///Optional user-defined label for the order
    #[serde(default)]
    pub label: String,
    ///Limit price in quote currency.<br />This field is still required for market orders because it is a component of the signature. However, market orders will not leave a resting order in the book in case of a partial fill.
    pub limit_price: bigdecimal::BigDecimal,
    ///Max fee in units of the quote currency. Order will be rejected if the supplied max fee is below the estimated fee for this order.
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the order is tagged for market maker protections (default false)
    #[serde(default)]
    pub mmp: bool,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
    #[serde(default = "defaults::private_order_quote_params_schema_order_type")]
    pub order_type: OrderType,
    ///If true, the order will not be able to increase position's size (default false). If the order amount exceeds available position size, the order will be filled up to the position size and the remainder will be cancelled. This flag is only supported for market orders or non-resting limit orders (IOC or FOK)
    #[serde(default)]
    pub reduce_only: bool,
    ///Optional referral code for the order
    #[serde(default)]
    pub referral_code: String,
    ///UTC timestamp in ms, if provided the matching engine will reject the order with an error if `reject_timestamp` < `server_time`. Note that the timestamp must be consistent with the server time: use `public/get_time` method to obtain current server time.
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub reject_timestamp: i64,
    ///If replaced, ID of the order that was replaced
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<uuid::Uuid>,
    ///Etherium signature of the order
    pub signature: String,
    ///Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.
    #[serde(default = "defaults::private_order_quote_params_schema_time_in_force")]
    pub time_in_force: TimeInForce,
}
impl From<&PrivateOrderQuoteParamsSchema> for PrivateOrderQuoteParamsSchema {
    fn from(value: &PrivateOrderQuoteParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderQuoteRequestSchema
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
      "const": "private/order_quote"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateOrderQuoteParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderQuoteRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateOrderQuoteRequestSchemaId>,
    pub method: String,
    pub params: PrivateOrderQuoteParamsSchema,
}
impl From<&PrivateOrderQuoteRequestSchema> for PrivateOrderQuoteRequestSchema {
    fn from(value: &PrivateOrderQuoteRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderQuoteRequestSchemaId
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
pub enum PrivateOrderQuoteRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateOrderQuoteRequestSchemaId> for PrivateOrderQuoteRequestSchemaId {
    fn from(value: &PrivateOrderQuoteRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateOrderQuoteRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateOrderQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateOrderQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateOrderQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateOrderQuoteRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateOrderQuoteRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateOrderQuoteResponseSchema
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
      "$ref": "#/definitions/PrivateOrderQuoteResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderQuoteResponseSchema {
    pub id: PrivateOrderQuoteResponseSchemaId,
    ///
    pub result: PrivateOrderQuoteResultSchema,
}
impl From<&PrivateOrderQuoteResponseSchema> for PrivateOrderQuoteResponseSchema {
    fn from(value: &PrivateOrderQuoteResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderQuoteResponseSchemaId
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
pub enum PrivateOrderQuoteResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateOrderQuoteResponseSchemaId> for PrivateOrderQuoteResponseSchemaId {
    fn from(value: &PrivateOrderQuoteResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateOrderQuoteResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateOrderQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateOrderQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateOrderQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateOrderQuoteResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateOrderQuoteResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateOrderQuoteResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "estimated_fee",
    "estimated_fill_amount",
    "estimated_fill_price",
    "estimated_order_status",
    "estimated_realized_pnl",
    "invalid_reason",
    "is_valid",
    "post_initial_margin",
    "post_liquidation_price",
    "pre_initial_margin",
    "suggested_max_fee"
  ],
  "properties": {
    "estimated_fee": {
      "title": "estimated_fee",
      "description": "An estimate for how much the user will actually pay in fees for the order ($ for the whole trade).",
      "type": "string",
      "format": "decimal"
    },
    "estimated_fill_amount": {
      "title": "estimated_fill_amount",
      "description": "An estimate the amount that will be instantly crossed upon order submission.",
      "type": "string",
      "format": "decimal"
    },
    "estimated_fill_price": {
      "title": "estimated_fill_price",
      "description": "An estimate for the average fill price of the order.",
      "type": "string",
      "format": "decimal"
    },
    "estimated_order_status": {
      "title": "estimated_order_status",
      "description": "An estimate for the order status after submission. Fully filled orders will be marked as `filled`, unfilled and partially filled limit orders will be marked as `open` and partially filled market / IOC orders will be marked as `cancelled`.",
      "type": "string",
      "enum": [
        "open",
        "filled",
        "rejected",
        "cancelled",
        "expired"
      ]
    },
    "estimated_realized_pnl": {
      "title": "estimated_realized_pnl",
      "description": "An estimate for the realized PnL of the order. For orders with an estimated `cancelled` status this only includes PnL of the filled amount.",
      "type": "string",
      "format": "decimal"
    },
    "invalid_reason": {
      "title": "invalid_reason",
      "description": "Reason for the order being invalid, if any.",
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
      "description": "`True` if order is expected to pass margin requirements.",
      "type": "boolean"
    },
    "post_initial_margin": {
      "title": "post_initial_margin",
      "description": "User's hypothetical margin balance if the order or trade were to get accepted. For maker orders (limit and not IOC/FOK) this also includes the margin locked for other outstanding orders.",
      "type": "string",
      "format": "decimal"
    },
    "post_liquidation_price": {
      "title": "post_liquidation_price",
      "description": "Subaccounts's liquidation price if the order were to be fully filled.",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "pre_initial_margin": {
      "title": "pre_initial_margin",
      "description": "User's initial margin balance before the order or trade. For maker orders (limit and not IOC/FOK) this also includes the margin locked for other outstanding orders.",
      "type": "string",
      "format": "decimal"
    },
    "suggested_max_fee": {
      "title": "suggested_max_fee",
      "description": "Recommended value for `max_fee` of the order (per contract).",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderQuoteResultSchema {
    ///An estimate for how much the user will actually pay in fees for the order ($ for the whole trade).
    pub estimated_fee: bigdecimal::BigDecimal,
    ///An estimate the amount that will be instantly crossed upon order submission.
    pub estimated_fill_amount: bigdecimal::BigDecimal,
    ///An estimate for the average fill price of the order.
    pub estimated_fill_price: bigdecimal::BigDecimal,
    ///An estimate for the order status after submission. Fully filled orders will be marked as `filled`, unfilled and partially filled limit orders will be marked as `open` and partially filled market / IOC orders will be marked as `cancelled`.
    pub estimated_order_status: EstimatedOrderStatus,
    ///An estimate for the realized PnL of the order. For orders with an estimated `cancelled` status this only includes PnL of the filled amount.
    pub estimated_realized_pnl: bigdecimal::BigDecimal,
    ///Reason for the order being invalid, if any.
    pub invalid_reason: Option<InvalidReason>,
    ///`True` if order is expected to pass margin requirements.
    pub is_valid: bool,
    ///User's hypothetical margin balance if the order or trade were to get accepted. For maker orders (limit and not IOC/FOK) this also includes the margin locked for other outstanding orders.
    pub post_initial_margin: bigdecimal::BigDecimal,
    ///Subaccounts's liquidation price if the order were to be fully filled.
    pub post_liquidation_price: Option<bigdecimal::BigDecimal>,
    ///User's initial margin balance before the order or trade. For maker orders (limit and not IOC/FOK) this also includes the margin locked for other outstanding orders.
    pub pre_initial_margin: bigdecimal::BigDecimal,
    ///Recommended value for `max_fee` of the order (per contract).
    pub suggested_max_fee: bigdecimal::BigDecimal,
}
impl From<&PrivateOrderQuoteResultSchema> for PrivateOrderQuoteResultSchema {
    fn from(value: &PrivateOrderQuoteResultSchema) -> Self {
        value.clone()
    }
}
///Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "time_in_force",
  "description": "Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.",
  "default": "gtc",
  "type": "string",
  "enum": [
    "gtc",
    "post_only",
    "fok",
    "ioc"
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
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    Fok,
    #[serde(rename = "ioc")]
    Ioc,
}
impl From<&TimeInForce> for TimeInForce {
    fn from(value: &TimeInForce) -> Self {
        value.clone()
    }
}
impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match *self {
            Self::Gtc => "gtc".to_string(),
            Self::PostOnly => "post_only".to_string(),
            Self::Fok => "fok".to_string(),
            Self::Ioc => "ioc".to_string(),
        }
    }
}
impl std::str::FromStr for TimeInForce {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "gtc" => Ok(Self::Gtc),
            "post_only" => Ok(Self::PostOnly),
            "fok" => Ok(Self::Fok),
            "ioc" => Ok(Self::Ioc),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::Gtc
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
    pub(super) fn private_order_quote_params_schema_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn private_order_quote_params_schema_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
}
