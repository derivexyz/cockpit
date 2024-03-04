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
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
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
///Get all open orders of a subacccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_open_orders",
  "description": "Get all open orders of a subacccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetOpenOrdersJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetOpenOrders(pub PrivateGetOpenOrdersJsonrpcSchema);
impl std::ops::Deref for PrivateGetOpenOrders {
    type Target = PrivateGetOpenOrdersJsonrpcSchema;
    fn deref(&self) -> &PrivateGetOpenOrdersJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetOpenOrders> for PrivateGetOpenOrdersJsonrpcSchema {
    fn from(value: PrivateGetOpenOrders) -> Self {
        value.0
    }
}
impl From<&PrivateGetOpenOrders> for PrivateGetOpenOrders {
    fn from(value: &PrivateGetOpenOrders) -> Self {
        value.clone()
    }
}
impl From<PrivateGetOpenOrdersJsonrpcSchema> for PrivateGetOpenOrders {
    fn from(value: PrivateGetOpenOrdersJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetOpenOrdersJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetOpenOrdersRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetOpenOrdersResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOpenOrdersJsonrpcSchema {
    pub request: PrivateGetOpenOrdersRequestSchema,
    pub response: PrivateGetOpenOrdersResponseSchema,
}
impl From<&PrivateGetOpenOrdersJsonrpcSchema> for PrivateGetOpenOrdersJsonrpcSchema {
    fn from(value: &PrivateGetOpenOrdersJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOpenOrdersParamsSchema
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
      "description": "Subaccount_id for which to get open orders",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOpenOrdersParamsSchema {
    ///Subaccount_id for which to get open orders
    pub subaccount_id: i64,
}
impl From<&PrivateGetOpenOrdersParamsSchema> for PrivateGetOpenOrdersParamsSchema {
    fn from(value: &PrivateGetOpenOrdersParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOpenOrdersRequestSchema
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
      "const": "private/get_open_orders"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetOpenOrdersParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOpenOrdersRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetOpenOrdersRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetOpenOrdersParamsSchema,
}
impl From<&PrivateGetOpenOrdersRequestSchema> for PrivateGetOpenOrdersRequestSchema {
    fn from(value: &PrivateGetOpenOrdersRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOpenOrdersRequestSchemaId
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
pub enum PrivateGetOpenOrdersRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetOpenOrdersRequestSchemaId> for PrivateGetOpenOrdersRequestSchemaId {
    fn from(value: &PrivateGetOpenOrdersRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetOpenOrdersRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetOpenOrdersRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetOpenOrdersRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetOpenOrdersRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetOpenOrdersRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetOpenOrdersRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetOpenOrdersResponseSchema
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
      "$ref": "#/definitions/PrivateGetOpenOrdersResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOpenOrdersResponseSchema {
    pub id: PrivateGetOpenOrdersResponseSchemaId,
    ///
    pub result: PrivateGetOpenOrdersResultSchema,
}
impl From<&PrivateGetOpenOrdersResponseSchema> for PrivateGetOpenOrdersResponseSchema {
    fn from(value: &PrivateGetOpenOrdersResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetOpenOrdersResponseSchemaId
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
pub enum PrivateGetOpenOrdersResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetOpenOrdersResponseSchemaId>
for PrivateGetOpenOrdersResponseSchemaId {
    fn from(value: &PrivateGetOpenOrdersResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetOpenOrdersResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetOpenOrdersResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetOpenOrdersResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetOpenOrdersResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetOpenOrdersResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetOpenOrdersResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetOpenOrdersResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "orders",
    "subaccount_id"
  ],
  "properties": {
    "orders": {
      "title": "orders",
      "description": "List of open orders",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/OrderResponseSchema",
        "field_many": true
      }
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to get open orders",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetOpenOrdersResultSchema {
    ///List of open orders
    pub orders: Vec<OrderResponseSchema>,
    ///Subaccount_id for which to get open orders
    pub subaccount_id: i64,
}
impl From<&PrivateGetOpenOrdersResultSchema> for PrivateGetOpenOrdersResultSchema {
    fn from(value: &PrivateGetOpenOrdersResultSchema) -> Self {
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
