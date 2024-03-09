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
///Cancels an open quote.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_quote",
  "description": "Cancels an open quote.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelQuoteJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelQuote(pub PrivateCancelQuoteJsonrpcSchema);
impl std::ops::Deref for PrivateCancelQuote {
    type Target = PrivateCancelQuoteJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelQuoteJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelQuote> for PrivateCancelQuoteJsonrpcSchema {
    fn from(value: PrivateCancelQuote) -> Self {
        value.0
    }
}
impl From<&PrivateCancelQuote> for PrivateCancelQuote {
    fn from(value: &PrivateCancelQuote) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelQuoteJsonrpcSchema> for PrivateCancelQuote {
    fn from(value: PrivateCancelQuoteJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelQuoteJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelQuoteRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelQuoteResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelQuoteJsonrpcSchema {
    pub request: PrivateCancelQuoteRequestSchema,
    pub response: PrivateCancelQuoteResponseSchema,
}
impl From<&PrivateCancelQuoteJsonrpcSchema> for PrivateCancelQuoteJsonrpcSchema {
    fn from(value: &PrivateCancelQuoteJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelQuoteParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "quote_id",
    "subaccount_id"
  ],
  "properties": {
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID to cancel",
      "type": "string",
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

pub struct PrivateCancelQuoteParamsSchema {
    ///Quote ID to cancel
    pub quote_id: uuid::Uuid,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelQuoteParamsSchema> for PrivateCancelQuoteParamsSchema {
    fn from(value: &PrivateCancelQuoteParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelQuoteRequestSchema
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
      "const": "private/cancel_quote"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelQuoteParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelQuoteRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelQuoteRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelQuoteParamsSchema,
}
impl From<&PrivateCancelQuoteRequestSchema> for PrivateCancelQuoteRequestSchema {
    fn from(value: &PrivateCancelQuoteRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelQuoteRequestSchemaId
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
pub enum PrivateCancelQuoteRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelQuoteRequestSchemaId> for PrivateCancelQuoteRequestSchemaId {
    fn from(value: &PrivateCancelQuoteRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelQuoteRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelQuoteRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelQuoteRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelQuoteRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelQuoteResponseSchema
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
      "$ref": "#/definitions/PrivateCancelQuoteResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelQuoteResponseSchema {
    pub id: PrivateCancelQuoteResponseSchemaId,
    ///
    pub result: PrivateCancelQuoteResultSchema,
}
impl From<&PrivateCancelQuoteResponseSchema> for PrivateCancelQuoteResponseSchema {
    fn from(value: &PrivateCancelQuoteResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelQuoteResponseSchemaId
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
pub enum PrivateCancelQuoteResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelQuoteResponseSchemaId> for PrivateCancelQuoteResponseSchemaId {
    fn from(value: &PrivateCancelQuoteResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelQuoteResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelQuoteResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelQuoteResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelQuoteResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelQuoteResultSchema
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
    "fee",
    "label",
    "last_update_timestamp",
    "legs",
    "legs_hash",
    "liquidity_role",
    "max_fee",
    "nonce",
    "quote_id",
    "rfq_id",
    "signature",
    "signature_expiry_sec",
    "signer",
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
    "fee": {
      "title": "fee",
      "description": "Fee paid for this quote (if executed)",
      "type": "string",
      "format": "decimal"
    },
    "label": {
      "title": "label",
      "description": "User-defined label, if any",
      "type": "string"
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
    "max_fee": {
      "title": "max_fee",
      "description": "Signed max fee",
      "type": "string",
      "format": "decimal"
    },
    "mmp": {
      "title": "mmp",
      "description": "Whether the quote is tagged for market maker protections (default false)",
      "default": false,
      "type": "boolean"
    },
    "nonce": {
      "title": "nonce",
      "description": "Nonce",
      "type": "integer"
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
    "signature": {
      "title": "signature",
      "description": "Etherium signature of the quote",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Owner wallet address or registered session key that signed the quote",
      "type": "string"
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
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status (only for executed quotes)",
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

pub struct PrivateCancelQuoteResultSchema {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///Quote direction
    pub direction: Direction,
    ///Fee paid for this quote (if executed)
    pub fee: bigdecimal::BigDecimal,
    ///User-defined label, if any
    pub label: String,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///Quote legs
    pub legs: Vec<LegPricedSchema>,
    ///Hash of the legs of the best quote to be signed by the taker.
    pub legs_hash: String,
    ///Liquidity role
    pub liquidity_role: LiquidityRole,
    ///Signed max fee
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the quote is tagged for market maker protections (default false)
    #[serde(default)]
    pub mmp: bool,
    ///Nonce
    pub nonce: i64,
    ///Quote ID
    pub quote_id: uuid::Uuid,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Etherium signature of the quote
    pub signature: String,
    ///Unix timestamp in seconds
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed the quote
    pub signer: String,
    ///Status
    pub status: Status,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Blockchain transaction hash (only for executed quotes)
    pub tx_hash: Option<String>,
    ///Blockchain transaction status (only for executed quotes)
    pub tx_status: Option<TxStatus>,
}
impl From<&PrivateCancelQuoteResultSchema> for PrivateCancelQuoteResultSchema {
    fn from(value: &PrivateCancelQuoteResultSchema) -> Self {
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
