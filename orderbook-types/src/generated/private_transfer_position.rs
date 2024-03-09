#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
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
///Role of the user in the trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "liquidity_role",
  "description": "Role of the user in the trade",
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
/**Transfers a positions from one subaccount to another, owned by the same wallet.

The transfer is executed as a pair of orders crossing each other.
The maker order is created first, followed by a taker order crossing it.
The order amounts, limit prices and instrument name must be the same for both orders.
Fee is not charged and a zero `max_fee` must be signed.
The maker order is forcibly considered to be `reduce_only`, meaning it can only reduce the position size.

History: For position transfer history, use the `private/get_trade_history` RPC (not `private/get_erc20_transfer_history`).*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/transfer_position",
  "description": "Transfers a positions from one subaccount to another, owned by the same wallet.\n\nThe transfer is executed as a pair of orders crossing each other.\nThe maker order is created first, followed by a taker order crossing it.\nThe order amounts, limit prices and instrument name must be the same for both orders.\nFee is not charged and a zero `max_fee` must be signed.\nThe maker order is forcibly considered to be `reduce_only`, meaning it can only reduce the position size.\n\nHistory: For position transfer history, use the `private/get_trade_history` RPC (not `private/get_erc20_transfer_history`).",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateTransferPositionJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateTransferPosition(pub PrivateTransferPositionJsonrpcSchema);
impl std::ops::Deref for PrivateTransferPosition {
    type Target = PrivateTransferPositionJsonrpcSchema;
    fn deref(&self) -> &PrivateTransferPositionJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateTransferPosition> for PrivateTransferPositionJsonrpcSchema {
    fn from(value: PrivateTransferPosition) -> Self {
        value.0
    }
}
impl From<&PrivateTransferPosition> for PrivateTransferPosition {
    fn from(value: &PrivateTransferPosition) -> Self {
        value.clone()
    }
}
impl From<PrivateTransferPositionJsonrpcSchema> for PrivateTransferPosition {
    fn from(value: PrivateTransferPositionJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateTransferPositionJsonrpcSchema
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
      "$ref": "#/definitions/PrivateTransferPositionRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateTransferPositionResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferPositionJsonrpcSchema {
    pub request: PrivateTransferPositionRequestSchema,
    pub response: PrivateTransferPositionResponseSchema,
}
impl From<&PrivateTransferPositionJsonrpcSchema>
for PrivateTransferPositionJsonrpcSchema {
    fn from(value: &PrivateTransferPositionJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferPositionParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "maker_params",
    "taker_params",
    "wallet"
  ],
  "properties": {
    "maker_params": {
      "description": "Maker order parameters and signature. Maximum transfer amount is limited by the size of the maker position. Transfers that increase the maker's position size are not allowed.",
      "type": "object",
      "$ref": "#/definitions/TradeModuleParamsSchema",
      "field_many": false
    },
    "taker_params": {
      "description": "Taker order parameters and signature",
      "type": "object",
      "$ref": "#/definitions/TradeModuleParamsSchema",
      "field_many": false
    },
    "wallet": {
      "title": "wallet",
      "description": "Public key (wallet) of the account",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferPositionParamsSchema {
    ///Maker order parameters and signature. Maximum transfer amount is limited by the size of the maker position. Transfers that increase the maker's position size are not allowed.
    pub maker_params: TradeModuleParamsSchema,
    ///Taker order parameters and signature
    pub taker_params: TradeModuleParamsSchema,
    ///Public key (wallet) of the account
    pub wallet: String,
}
impl From<&PrivateTransferPositionParamsSchema> for PrivateTransferPositionParamsSchema {
    fn from(value: &PrivateTransferPositionParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferPositionRequestSchema
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
      "const": "private/transfer_position"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateTransferPositionParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferPositionRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateTransferPositionRequestSchemaId>,
    pub method: String,
    pub params: PrivateTransferPositionParamsSchema,
}
impl From<&PrivateTransferPositionRequestSchema>
for PrivateTransferPositionRequestSchema {
    fn from(value: &PrivateTransferPositionRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferPositionRequestSchemaId
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
pub enum PrivateTransferPositionRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateTransferPositionRequestSchemaId>
for PrivateTransferPositionRequestSchemaId {
    fn from(value: &PrivateTransferPositionRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateTransferPositionRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateTransferPositionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateTransferPositionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateTransferPositionRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateTransferPositionRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateTransferPositionRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateTransferPositionResponseSchema
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
      "$ref": "#/definitions/PrivateTransferPositionResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferPositionResponseSchema {
    pub id: PrivateTransferPositionResponseSchemaId,
    ///
    pub result: PrivateTransferPositionResultSchema,
}
impl From<&PrivateTransferPositionResponseSchema>
for PrivateTransferPositionResponseSchema {
    fn from(value: &PrivateTransferPositionResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferPositionResponseSchemaId
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
pub enum PrivateTransferPositionResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateTransferPositionResponseSchemaId>
for PrivateTransferPositionResponseSchemaId {
    fn from(value: &PrivateTransferPositionResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateTransferPositionResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateTransferPositionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateTransferPositionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateTransferPositionResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateTransferPositionResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateTransferPositionResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateTransferPositionResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "maker_order",
    "maker_trade",
    "taker_order",
    "taker_trade"
  ],
  "properties": {
    "maker_order": {
      "type": "object",
      "$ref": "#/definitions/OrderResponseSchema",
      "field_many": false
    },
    "maker_trade": {
      "type": "object",
      "$ref": "#/definitions/TradeResponseSchema",
      "field_many": false
    },
    "taker_order": {
      "type": "object",
      "$ref": "#/definitions/OrderResponseSchema",
      "field_many": false
    },
    "taker_trade": {
      "type": "object",
      "$ref": "#/definitions/TradeResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferPositionResultSchema {
    pub maker_order: OrderResponseSchema,
    pub maker_trade: TradeResponseSchema,
    pub taker_order: OrderResponseSchema,
    pub taker_trade: TradeResponseSchema,
}
impl From<&PrivateTransferPositionResultSchema> for PrivateTransferPositionResultSchema {
    fn from(value: &PrivateTransferPositionResultSchema) -> Self {
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
///TradeModuleParamsSchema
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
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
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
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeModuleParamsSchema {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Order direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
    ///Limit price in quote currency.<br />This field is still required for market orders because it is a component of the signature. However, market orders will not leave a resting order in the book in case of a partial fill.
    pub limit_price: bigdecimal::BigDecimal,
    ///Max fee in units of the quote currency. Order will be rejected if the supplied max fee is below the estimated fee for this order.
    pub max_fee: bigdecimal::BigDecimal,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Etherium signature of the order
    pub signature: String,
    ///Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&TradeModuleParamsSchema> for TradeModuleParamsSchema {
    fn from(value: &TradeModuleParamsSchema) -> Self {
        value.clone()
    }
}
///TradeResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "direction",
    "index_price",
    "instrument_name",
    "is_transfer",
    "label",
    "liquidity_role",
    "mark_price",
    "order_id",
    "quote_id",
    "realized_pnl",
    "subaccount_id",
    "timestamp",
    "trade_amount",
    "trade_fee",
    "trade_id",
    "trade_price",
    "tx_hash",
    "tx_status"
  ],
  "properties": {
    "direction": {
      "title": "direction",
      "description": "Order direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "index_price": {
      "title": "index_price",
      "description": "Index price of the underlying at the time of the trade",
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
      "description": "Whether the trade was generated through `private/transfer_position`",
      "type": "boolean"
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "type": "string"
    },
    "liquidity_role": {
      "title": "liquidity_role",
      "description": "Role of the user in the trade",
      "type": "string",
      "enum": [
        "maker",
        "taker"
      ]
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Mark price of the instrument at the time of the trade",
      "type": "string",
      "format": "decimal"
    },
    "order_id": {
      "title": "order_id",
      "description": "Order ID",
      "type": "string"
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
    "realized_pnl": {
      "title": "realized_pnl",
      "description": "Realized PnL for this trade",
      "type": "string",
      "format": "decimal"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Trade timestamp (in ms since Unix epoch)",
      "type": "integer"
    },
    "trade_amount": {
      "title": "trade_amount",
      "description": "Amount filled in this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_fee": {
      "title": "trade_fee",
      "description": "Fee for this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_id": {
      "title": "trade_id",
      "description": "Trade ID",
      "type": "string"
    },
    "trade_price": {
      "title": "trade_price",
      "description": "Price at which the trade was filled",
      "type": "string",
      "format": "decimal"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Blockchain transaction hash",
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status",
      "type": "string",
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

pub struct TradeResponseSchema {
    ///Order direction
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Whether the trade was generated through `private/transfer_position`
    pub is_transfer: bool,
    ///Optional user-defined label for the order
    pub label: String,
    ///Role of the user in the trade
    pub liquidity_role: LiquidityRole,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
    ///Order ID
    pub order_id: String,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///Realized PnL for this trade
    pub realized_pnl: bigdecimal::BigDecimal,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Trade timestamp (in ms since Unix epoch)
    pub timestamp: i64,
    ///Amount filled in this trade
    pub trade_amount: bigdecimal::BigDecimal,
    ///Fee for this trade
    pub trade_fee: bigdecimal::BigDecimal,
    ///Trade ID
    pub trade_id: String,
    ///Price at which the trade was filled
    pub trade_price: bigdecimal::BigDecimal,
    ///Blockchain transaction hash
    pub tx_hash: Option<String>,
    ///Blockchain transaction status
    pub tx_status: TxStatus,
}
impl From<&TradeResponseSchema> for TradeResponseSchema {
    fn from(value: &TradeResponseSchema) -> Self {
        value.clone()
    }
}
///Blockchain transaction status
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Blockchain transaction status",
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
