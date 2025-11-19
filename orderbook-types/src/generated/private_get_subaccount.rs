#![allow(unused_variables)]
#![allow(unused_imports)]
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;
///Type of asset collateral (currently always `erc20`)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "asset_type",
  "description": "Type of asset collateral (currently always `erc20`)",
  "type": "string",
  "enum": [
    "erc20",
    "option",
    "perp"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AssetType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}
impl From<&AssetType> for AssetType {
    fn from(value: &AssetType) -> Self {
        value.clone()
    }
}
impl ToString for AssetType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}
impl std::str::FromStr for AssetType {
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
impl std::convert::TryFrom<&str> for AssetType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///If cancelled, reason behind order cancellation
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "cancel_reason",
  "description": "If cancelled, reason behind order cancellation",
  "type": "string",
  "enum": [
    "",
    "user_request",
    "mmp_trigger",
    "insufficient_margin",
    "signed_max_fee_too_low",
    "cancel_on_disconnect",
    "ioc_or_market_partial_fill",
    "session_key_deregistered",
    "subaccount_withdrawn",
    "compliance"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CancelReason {
    #[serde(rename = "")]
    X,
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "mmp_trigger")]
    MmpTrigger,
    #[serde(rename = "insufficient_margin")]
    InsufficientMargin,
    #[serde(rename = "signed_max_fee_too_low")]
    SignedMaxFeeTooLow,
    #[serde(rename = "cancel_on_disconnect")]
    CancelOnDisconnect,
    #[serde(rename = "ioc_or_market_partial_fill")]
    IocOrMarketPartialFill,
    #[serde(rename = "session_key_deregistered")]
    SessionKeyDeregistered,
    #[serde(rename = "subaccount_withdrawn")]
    SubaccountWithdrawn,
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
            Self::MmpTrigger => "mmp_trigger".to_string(),
            Self::InsufficientMargin => "insufficient_margin".to_string(),
            Self::SignedMaxFeeTooLow => "signed_max_fee_too_low".to_string(),
            Self::CancelOnDisconnect => "cancel_on_disconnect".to_string(),
            Self::IocOrMarketPartialFill => "ioc_or_market_partial_fill".to_string(),
            Self::SessionKeyDeregistered => "session_key_deregistered".to_string(),
            Self::SubaccountWithdrawn => "subaccount_withdrawn".to_string(),
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
            "mmp_trigger" => Ok(Self::MmpTrigger),
            "insufficient_margin" => Ok(Self::InsufficientMargin),
            "signed_max_fee_too_low" => Ok(Self::SignedMaxFeeTooLow),
            "cancel_on_disconnect" => Ok(Self::CancelOnDisconnect),
            "ioc_or_market_partial_fill" => Ok(Self::IocOrMarketPartialFill),
            "session_key_deregistered" => Ok(Self::SessionKeyDeregistered),
            "subaccount_withdrawn" => Ok(Self::SubaccountWithdrawn),
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
///CollateralResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset_name",
    "asset_type",
    "cumulative_interest",
    "currency",
    "initial_margin",
    "maintenance_margin",
    "mark_price",
    "mark_value",
    "pending_interest"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Asset amount of given collateral",
      "type": "string",
      "format": "decimal"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Asset name",
      "type": "string"
    },
    "asset_type": {
      "title": "asset_type",
      "description": "Type of asset collateral (currently always `erc20`)",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "cumulative_interest": {
      "title": "cumulative_interest",
      "description": "Cumulative interest earned on supplying collateral or paid for borrowing",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "initial_margin": {
      "title": "initial_margin",
      "description": "USD value of collateral that contributes to initial margin",
      "type": "string",
      "format": "decimal"
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "USD value of collateral that contributes to maintenance margin",
      "type": "string",
      "format": "decimal"
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Current mark price of the asset",
      "type": "string",
      "format": "decimal"
    },
    "mark_value": {
      "title": "mark_value",
      "description": "USD value of the collateral (amount * mark price)",
      "type": "string",
      "format": "decimal"
    },
    "pending_interest": {
      "title": "pending_interest",
      "description": "Portion of interest that has not yet been settled on-chain. This number is added to the portfolio value for margin calculations purposes.",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct CollateralResponseSchema {
    ///Asset amount of given collateral
    pub amount: bigdecimal::BigDecimal,
    ///Asset name
    pub asset_name: String,
    ///Type of asset collateral (currently always `erc20`)
    pub asset_type: AssetType,
    ///Cumulative interest earned on supplying collateral or paid for borrowing
    pub cumulative_interest: bigdecimal::BigDecimal,
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///USD value of collateral that contributes to initial margin
    pub initial_margin: bigdecimal::BigDecimal,
    ///USD value of collateral that contributes to maintenance margin
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Current mark price of the asset
    pub mark_price: bigdecimal::BigDecimal,
    ///USD value of the collateral (amount * mark price)
    pub mark_value: bigdecimal::BigDecimal,
    ///Portion of interest that has not yet been settled on-chain. This number is added to the portfolio value for margin calculations purposes.
    pub pending_interest: bigdecimal::BigDecimal,
}
impl From<&CollateralResponseSchema> for CollateralResponseSchema {
    fn from(value: &CollateralResponseSchema) -> Self {
        value.clone()
    }
}
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin_type",
  "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
  "type": "string",
  "enum": [
    "PM",
    "SM"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum MarginType {
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "PM2")]
    Pm2,
}
impl From<&MarginType> for MarginType {
    fn from(value: &MarginType) -> Self {
        value.clone()
    }
}
impl ToString for MarginType {
    fn to_string(&self) -> String {
        match *self {
            Self::Pm => "PM".to_string(),
            Self::Sm => "SM".to_string(),
            Self::Pm2 => "PM2".to_string(),
        }
    }
}
impl std::str::FromStr for MarginType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PM" => Ok(Self::Pm),
            "SM" => Ok(Self::Sm),
            "PM2" => Ok(Self::Pm2),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for MarginType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MarginType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MarginType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///OrderResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "average_price",
    "cancel_reason",
    "creation_timestamp",
    "direction",
    "filled_amount",
    "instrument_name",
    "is_transfer",
    "label",
    "last_update_timestamp",
    "limit_price",
    "max_fee",
    "mmp",
    "nonce",
    "order_fee",
    "order_id",
    "order_status",
    "order_type",
    "quote_id",
    "signature",
    "signature_expiry_sec",
    "signer",
    "subaccount_id",
    "time_in_force"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Order amount in units of the base",
      "type": "string",
      "format": "decimal"
    },
    "average_price": {
      "title": "average_price",
      "description": "Average fill price",
      "type": "string",
      "format": "decimal"
    },
    "cancel_reason": {
      "title": "cancel_reason",
      "description": "If cancelled, reason behind order cancellation",
      "type": "string",
      "enum": [
        "",
        "user_request",
        "mmp_trigger",
        "insufficient_margin",
        "signed_max_fee_too_low",
        "cancel_on_disconnect",
        "ioc_or_market_partial_fill",
        "session_key_deregistered",
        "subaccount_withdrawn",
        "compliance"
      ]
    },
    "creation_timestamp": {
      "title": "creation_timestamp",
      "description": "Creation timestamp (in ms since Unix epoch)",
      "type": "integer"
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
    "filled_amount": {
      "title": "filled_amount",
      "description": "Total filled amount for the order",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "is_transfer": {
      "title": "is_transfer",
      "description": "Whether the order was generated through `private/transfer_position`",
      "type": "boolean"
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "type": "string"
    },
    "last_update_timestamp": {
      "title": "last_update_timestamp",
      "description": "Last update timestamp (in ms since Unix epoch)",
      "type": "integer"
    },
    "limit_price": {
      "title": "limit_price",
      "description": "Limit price in quote currency",
      "type": "string",
      "format": "decimal"
    },
    "max_fee": {
      "title": "max_fee",
      "description": "Max fee in units of the quote currency",
      "type": "string",
      "format": "decimal"
    },
    "mmp": {
      "title": "mmp",
      "description": "Whether the order is tagged for market maker protections",
      "type": "boolean"
    },
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_3_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "order_fee": {
      "title": "order_fee",
      "description": "Total order fee paid so far",
      "type": "string",
      "format": "decimal"
    },
    "order_id": {
      "title": "order_id",
      "description": "Order ID",
      "type": "string"
    },
    "order_status": {
      "title": "order_status",
      "description": "Order status",
      "type": "string",
      "enum": [
        "open",
        "filled",
        "rejected",
        "cancelled",
        "expired"
      ]
    },
    "order_type": {
      "title": "order_type",
      "description": "Order type",
      "type": "string",
      "enum": [
        "limit",
        "market"
      ]
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID if the trade was executed via RFQ",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "replaced_order_id": {
      "title": "replaced_order_id",
      "description": "If replaced, ID of the order that was replaced",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "signature": {
      "title": "signature",
      "description": "Ethereum signature of the order",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Signature expiry timestamp",
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
      "description": "Time in force",
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

pub struct OrderResponseSchema {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Average fill price
    pub average_price: bigdecimal::BigDecimal,
    ///If cancelled, reason behind order cancellation
    pub cancel_reason: CancelReason,
    ///Creation timestamp (in ms since Unix epoch)
    pub creation_timestamp: i64,
    ///Order direction
    pub direction: Direction,
    ///Total filled amount for the order
    pub filled_amount: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Whether the order was generated through `private/transfer_position`
    pub is_transfer: bool,
    ///Optional user-defined label for the order
    pub label: String,
    ///Last update timestamp (in ms since Unix epoch)
    pub last_update_timestamp: i64,
    ///Limit price in quote currency
    pub limit_price: bigdecimal::BigDecimal,
    ///Max fee in units of the quote currency
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the order is tagged for market maker protections
    pub mmp: bool,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_3_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Total order fee paid so far
    pub order_fee: bigdecimal::BigDecimal,
    ///Order ID
    pub order_id: String,
    ///Order status
    pub order_status: OrderStatus,
    ///Order type
    pub order_type: OrderType,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///If replaced, ID of the order that was replaced
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<uuid::Uuid>,
    ///Ethereum signature of the order
    pub signature: String,
    ///Signature expiry timestamp
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Time in force
    pub time_in_force: TimeInForce,
}
impl From<&OrderResponseSchema> for OrderResponseSchema {
    fn from(value: &OrderResponseSchema) -> Self {
        value.clone()
    }
}
///Order status
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "order_status",
  "description": "Order status",
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OrderStatus {
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
impl From<&OrderStatus> for OrderStatus {
    fn from(value: &OrderStatus) -> Self {
        value.clone()
    }
}
impl ToString for OrderStatus {
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
impl std::str::FromStr for OrderStatus {
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
impl std::convert::TryFrom<&str> for OrderStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrderStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrderStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Order type
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "order_type",
  "description": "Order type",
  "type": "string",
  "enum": [
    "limit",
    "market"
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "liquidation_price": {
      "title": "liquidation_price",
      "description": "Index price at which position will be liquidated",
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
///Get open orders, active positions, and collaterals of a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_subaccount",
  "description": "Get open orders, active positions, and collaterals of a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetSubaccountJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetSubaccount(pub PrivateGetSubaccountJsonrpcSchema);
impl std::ops::Deref for PrivateGetSubaccount {
    type Target = PrivateGetSubaccountJsonrpcSchema;
    fn deref(&self) -> &PrivateGetSubaccountJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetSubaccount> for PrivateGetSubaccountJsonrpcSchema {
    fn from(value: PrivateGetSubaccount) -> Self {
        value.0
    }
}
impl From<&PrivateGetSubaccount> for PrivateGetSubaccount {
    fn from(value: &PrivateGetSubaccount) -> Self {
        value.clone()
    }
}
impl From<PrivateGetSubaccountJsonrpcSchema> for PrivateGetSubaccount {
    fn from(value: PrivateGetSubaccountJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetSubaccountJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountJsonrpcSchema {
    pub request: PrivateGetSubaccountRequestSchema,
    pub response: PrivateGetSubaccountResponseSchema,
}
impl From<&PrivateGetSubaccountJsonrpcSchema> for PrivateGetSubaccountJsonrpcSchema {
    fn from(value: &PrivateGetSubaccountJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountParamsSchema
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

pub struct PrivateGetSubaccountParamsSchema {
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetSubaccountParamsSchema> for PrivateGetSubaccountParamsSchema {
    fn from(value: &PrivateGetSubaccountParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountRequestSchema
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
      "const": "private/get_subaccount"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetSubaccountRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetSubaccountParamsSchema,
}
impl From<&PrivateGetSubaccountRequestSchema> for PrivateGetSubaccountRequestSchema {
    fn from(value: &PrivateGetSubaccountRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountRequestSchemaId
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
pub enum PrivateGetSubaccountRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountRequestSchemaId> for PrivateGetSubaccountRequestSchemaId {
    fn from(value: &PrivateGetSubaccountRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountResponseSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountResponseSchema {
    pub id: PrivateGetSubaccountResponseSchemaId,
    ///
    pub result: PrivateGetSubaccountResultSchema,
}
impl From<&PrivateGetSubaccountResponseSchema> for PrivateGetSubaccountResponseSchema {
    fn from(value: &PrivateGetSubaccountResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountResponseSchemaId
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
pub enum PrivateGetSubaccountResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountResponseSchemaId> for PrivateGetSubaccountResponseSchemaId {
    fn from(value: &PrivateGetSubaccountResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "collaterals",
    "collaterals_initial_margin",
    "collaterals_maintenance_margin",
    "collaterals_value",
    "currency",
    "initial_margin",
    "is_under_liquidation",
    "label",
    "maintenance_margin",
    "margin_type",
    "open_orders",
    "open_orders_margin",
    "positions",
    "positions_initial_margin",
    "positions_maintenance_margin",
    "positions_value",
    "subaccount_id",
    "subaccount_value"
  ],
  "properties": {
    "collaterals": {
      "title": "collaterals",
      "description": "All collaterals that count towards margin of subaccount",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/CollateralResponseSchema",
        "field_many": true
      }
    },
    "collaterals_initial_margin": {
      "title": "collaterals_initial_margin",
      "description": "Total initial margin credit contributed by collaterals",
      "type": "string",
      "format": "decimal"
    },
    "collaterals_maintenance_margin": {
      "title": "collaterals_maintenance_margin",
      "description": "Total maintenance margin credit contributed by collaterals",
      "type": "string",
      "format": "decimal"
    },
    "collaterals_value": {
      "title": "collaterals_value",
      "description": "Total mark-to-market value of all collaterals",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "Currency of subaccount",
      "type": "string"
    },
    "initial_margin": {
      "title": "initial_margin",
      "description": "Total initial margin requirement of all positions and collaterals.Trades will be rejected if this value falls below zero after the trade.",
      "type": "string",
      "format": "decimal"
    },
    "is_under_liquidation": {
      "title": "is_under_liquidation",
      "description": "Whether the subaccount is undergoing a liquidation auction",
      "type": "boolean"
    },
    "label": {
      "title": "label",
      "description": "User defined label",
      "type": [
        "string",
        "null"
      ]
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.",
      "type": "string",
      "format": "decimal"
    },
    "margin_type": {
      "title": "margin_type",
      "description": "Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))",
      "type": "string",
      "enum": [
        "PM",
        "SM"
      ]
    },
    "open_orders": {
      "title": "open_orders",
      "description": "All open orders of subaccount",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/OrderResponseSchema",
        "field_many": true
      }
    },
    "open_orders_margin": {
      "title": "open_orders_margin",
      "description": "Total margin requirement of all open orders.Orders will be rejected if this value plus initial margin are below zero after the order.",
      "type": "string",
      "format": "decimal"
    },
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
    "positions_initial_margin": {
      "title": "positions_initial_margin",
      "description": "Total initial margin requirement of all positions",
      "type": "string",
      "format": "decimal"
    },
    "positions_maintenance_margin": {
      "title": "positions_maintenance_margin",
      "description": "Total maintenance margin requirement of all positions",
      "type": "string",
      "format": "decimal"
    },
    "positions_value": {
      "title": "positions_value",
      "description": "Total mark-to-market value of all positions",
      "type": "string",
      "format": "decimal"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "subaccount_value": {
      "title": "subaccount_value",
      "description": "Total mark-to-market value of all positions and collaterals",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountResultSchema {
    ///All collaterals that count towards margin of subaccount
    pub collaterals: Vec<CollateralResponseSchema>,
    ///Total initial margin credit contributed by collaterals
    pub collaterals_initial_margin: bigdecimal::BigDecimal,
    ///Total maintenance margin credit contributed by collaterals
    pub collaterals_maintenance_margin: bigdecimal::BigDecimal,
    ///Total mark-to-market value of all collaterals
    pub collaterals_value: bigdecimal::BigDecimal,
    ///Currency of subaccount
    pub currency: String,
    ///Total initial margin requirement of all positions and collaterals.Trades will be rejected if this value falls below zero after the trade.
    pub initial_margin: bigdecimal::BigDecimal,
    ///Whether the subaccount is undergoing a liquidation auction
    pub is_under_liquidation: bool,
    ///User defined label
    pub label: Option<String>,
    ///Total maintenance margin requirement of all positions and collaterals.If this value falls below zero, the subaccount will be flagged for liquidation.
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Margin type of subaccount (`PM` (Portfolio Margin) or `SM` (Standard Margin))
    pub margin_type: MarginType,
    ///All open orders of subaccount
    pub open_orders: Vec<OrderResponseSchema>,
    ///Total margin requirement of all open orders.Orders will be rejected if this value plus initial margin are below zero after the order.
    pub open_orders_margin: bigdecimal::BigDecimal,
    ///All active positions of subaccount
    pub positions: Vec<PositionResponseSchema>,
    ///Total initial margin requirement of all positions
    pub positions_initial_margin: bigdecimal::BigDecimal,
    ///Total maintenance margin requirement of all positions
    pub positions_maintenance_margin: bigdecimal::BigDecimal,
    ///Total mark-to-market value of all positions
    pub positions_value: bigdecimal::BigDecimal,
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Total mark-to-market value of all positions and collaterals
    pub subaccount_value: bigdecimal::BigDecimal,
}
impl From<&PrivateGetSubaccountResultSchema> for PrivateGetSubaccountResultSchema {
    fn from(value: &PrivateGetSubaccountResultSchema) -> Self {
        value.clone()
    }
}
///Time in force
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "time_in_force",
  "description": "Time in force",
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
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
