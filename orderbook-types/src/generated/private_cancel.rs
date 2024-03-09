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
///Cancel a single order.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel",
  "description": "Cancel a single order.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancel(pub PrivateCancelJsonrpcSchema);
impl std::ops::Deref for PrivateCancel {
    type Target = PrivateCancelJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancel> for PrivateCancelJsonrpcSchema {
    fn from(value: PrivateCancel) -> Self {
        value.0
    }
}
impl From<&PrivateCancel> for PrivateCancel {
    fn from(value: &PrivateCancel) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelJsonrpcSchema> for PrivateCancel {
    fn from(value: PrivateCancelJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelJsonrpcSchema {
    pub request: PrivateCancelRequestSchema,
    pub response: PrivateCancelResponseSchema,
}
impl From<&PrivateCancelJsonrpcSchema> for PrivateCancelJsonrpcSchema {
    fn from(value: &PrivateCancelJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_name",
    "order_id",
    "subaccount_id"
  ],
  "properties": {
    "instrument_name": {
      "title": "instrument_name",
      "type": "string"
    },
    "order_id": {
      "title": "order_id",
      "type": "string",
      "format": "uuid"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelParamsSchema {
    pub instrument_name: String,
    pub order_id: uuid::Uuid,
    pub subaccount_id: i64,
}
impl From<&PrivateCancelParamsSchema> for PrivateCancelParamsSchema {
    fn from(value: &PrivateCancelParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRequestSchema
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
      "const": "private/cancel"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelParamsSchema,
}
impl From<&PrivateCancelRequestSchema> for PrivateCancelRequestSchema {
    fn from(value: &PrivateCancelRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelRequestSchemaId
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
pub enum PrivateCancelRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelRequestSchemaId> for PrivateCancelRequestSchemaId {
    fn from(value: &PrivateCancelRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelResponseSchema
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
      "$ref": "#/definitions/PrivateCancelResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelResponseSchema {
    pub id: PrivateCancelResponseSchemaId,
    ///
    pub result: PrivateCancelResultSchema,
}
impl From<&PrivateCancelResponseSchema> for PrivateCancelResponseSchema {
    fn from(value: &PrivateCancelResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelResponseSchemaId
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
pub enum PrivateCancelResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelResponseSchemaId> for PrivateCancelResponseSchemaId {
    fn from(value: &PrivateCancelResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelResultSchema
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

pub struct PrivateCancelResultSchema {
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
impl From<&PrivateCancelResultSchema> for PrivateCancelResultSchema {
    fn from(value: &PrivateCancelResultSchema) -> Self {
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
