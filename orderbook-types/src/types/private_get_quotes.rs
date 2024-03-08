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
///PaginationInfoSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "count",
    "num_pages"
  ],
  "properties": {
    "count": {
      "title": "count",
      "description": "Total number of items, across all pages",
      "type": "integer"
    },
    "num_pages": {
      "title": "num_pages",
      "description": "Number of pages",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PaginationInfoSchema {
    ///Total number of items, across all pages
    pub count: i64,
    ///Number of pages
    pub num_pages: i64,
}
impl From<&PaginationInfoSchema> for PaginationInfoSchema {
    fn from(value: &PaginationInfoSchema) -> Self {
        value.clone()
    }
}
/**Retrieves a list of quotes matching filter criteria.
Market makers can use this to get their open quotes, quote history, etc.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_quotes",
  "description": "Retrieves a list of quotes matching filter criteria.\nMarket makers can use this to get their open quotes, quote history, etc.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetQuotesJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetQuotes(pub PrivateGetQuotesJsonrpcSchema);
impl std::ops::Deref for PrivateGetQuotes {
    type Target = PrivateGetQuotesJsonrpcSchema;
    fn deref(&self) -> &PrivateGetQuotesJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetQuotes> for PrivateGetQuotesJsonrpcSchema {
    fn from(value: PrivateGetQuotes) -> Self {
        value.0
    }
}
impl From<&PrivateGetQuotes> for PrivateGetQuotes {
    fn from(value: &PrivateGetQuotes) -> Self {
        value.clone()
    }
}
impl From<PrivateGetQuotesJsonrpcSchema> for PrivateGetQuotes {
    fn from(value: PrivateGetQuotesJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetQuotesJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetQuotesRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetQuotesResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetQuotesJsonrpcSchema {
    pub request: PrivateGetQuotesRequestSchema,
    pub response: PrivateGetQuotesResponseSchema,
}
impl From<&PrivateGetQuotesJsonrpcSchema> for PrivateGetQuotesJsonrpcSchema {
    fn from(value: &PrivateGetQuotesJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetQuotesParamsSchema
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
    "from_timestamp": {
      "title": "from_timestamp",
      "description": "Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.",
      "default": 0,
      "type": "integer"
    },
    "page": {
      "title": "page",
      "description": "Page number of results to return (default 1, returns last if above `num_pages`)",
      "default": 1,
      "type": "integer"
    },
    "page_size": {
      "title": "page_size",
      "description": "Number of results per page (default 100, max 1000)",
      "default": 100,
      "type": "integer"
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID filter, if applicable",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "rfq_id": {
      "title": "rfq_id",
      "description": "RFQ ID filter, if applicable",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "status": {
      "title": "status",
      "description": "Quote status filter, if applicable",
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "open",
        "filled",
        "cancelled",
        "expired"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID for auth purposes, returned data will be scoped to this subaccount.",
      "type": "integer"
    },
    "to_timestamp": {
      "title": "to_timestamp",
      "description": "Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.",
      "default": 18446744073709551615,
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetQuotesParamsSchema {
    ///Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.
    #[serde(default)]
    pub from_timestamp: i64,
    ///Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page (default 100, max 1000)
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
    ///Quote ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    ///RFQ ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    ///Quote status filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    ///Subaccount ID for auth purposes, returned data will be scoped to this subaccount.
    pub subaccount_id: i64,
    ///Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.
    #[serde(default = "defaults::default_u64::<i64, 18446744073709551615>")]
    pub to_timestamp: i64,
}
impl From<&PrivateGetQuotesParamsSchema> for PrivateGetQuotesParamsSchema {
    fn from(value: &PrivateGetQuotesParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetQuotesRequestSchema
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
      "const": "private/get_quotes"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetQuotesParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetQuotesRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetQuotesRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetQuotesParamsSchema,
}
impl From<&PrivateGetQuotesRequestSchema> for PrivateGetQuotesRequestSchema {
    fn from(value: &PrivateGetQuotesRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetQuotesRequestSchemaId
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
pub enum PrivateGetQuotesRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetQuotesRequestSchemaId> for PrivateGetQuotesRequestSchemaId {
    fn from(value: &PrivateGetQuotesRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetQuotesRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetQuotesRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetQuotesRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetQuotesRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetQuotesResponseSchema
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
      "$ref": "#/definitions/PrivateGetQuotesResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetQuotesResponseSchema {
    pub id: PrivateGetQuotesResponseSchemaId,
    ///
    pub result: PrivateGetQuotesResultSchema,
}
impl From<&PrivateGetQuotesResponseSchema> for PrivateGetQuotesResponseSchema {
    fn from(value: &PrivateGetQuotesResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetQuotesResponseSchemaId
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
pub enum PrivateGetQuotesResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetQuotesResponseSchemaId> for PrivateGetQuotesResponseSchemaId {
    fn from(value: &PrivateGetQuotesResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetQuotesResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetQuotesResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetQuotesResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetQuotesResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetQuotesResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "pagination",
    "quotes"
  ],
  "properties": {
    "pagination": {
      "description": "Pagination info",
      "type": "object",
      "$ref": "#/definitions/PaginationInfoSchema",
      "field_many": false
    },
    "quotes": {
      "title": "quotes",
      "description": "Quotes matching filter criteria",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/QuoteResultSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetQuotesResultSchema {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///Quotes matching filter criteria
    pub quotes: Vec<QuoteResultSchema>,
}
impl From<&PrivateGetQuotesResultSchema> for PrivateGetQuotesResultSchema {
    fn from(value: &PrivateGetQuotesResultSchema) -> Self {
        value.clone()
    }
}
///QuoteResultSchema
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

pub struct QuoteResultSchema {
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
impl From<&QuoteResultSchema> for QuoteResultSchema {
    fn from(value: &QuoteResultSchema) -> Self {
        value.clone()
    }
}
///Quote status filter, if applicable
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "Quote status filter, if applicable",
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
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
