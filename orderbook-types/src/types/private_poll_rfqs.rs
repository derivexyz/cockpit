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
///Retrieves a list of RFQs matching filter criteria. Market makers can use this to poll RFQs directed to them.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/poll_rfqs",
  "description": "Retrieves a list of RFQs matching filter criteria. Market makers can use this to poll RFQs directed to them.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivatePollRfqsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivatePollRfqs(pub PrivatePollRfqsJsonrpcSchema);
impl std::ops::Deref for PrivatePollRfqs {
    type Target = PrivatePollRfqsJsonrpcSchema;
    fn deref(&self) -> &PrivatePollRfqsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivatePollRfqs> for PrivatePollRfqsJsonrpcSchema {
    fn from(value: PrivatePollRfqs) -> Self {
        value.0
    }
}
impl From<&PrivatePollRfqs> for PrivatePollRfqs {
    fn from(value: &PrivatePollRfqs) -> Self {
        value.clone()
    }
}
impl From<PrivatePollRfqsJsonrpcSchema> for PrivatePollRfqs {
    fn from(value: PrivatePollRfqsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivatePollRfqsJsonrpcSchema
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
      "$ref": "#/definitions/PrivatePollRfqsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivatePollRfqsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivatePollRfqsJsonrpcSchema {
    pub request: PrivatePollRfqsRequestSchema,
    pub response: PrivatePollRfqsResponseSchema,
}
impl From<&PrivatePollRfqsJsonrpcSchema> for PrivatePollRfqsJsonrpcSchema {
    fn from(value: &PrivatePollRfqsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivatePollRfqsParamsSchema
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
      "description": "Earliest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provied, defaults to 0.",
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
      "description": "RFQ status filter, if applicable",
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
      "description": "Latest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.",
      "default": 18446744073709551615,
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivatePollRfqsParamsSchema {
    ///Earliest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provied, defaults to 0.
    #[serde(default)]
    pub from_timestamp: i64,
    ///Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page (default 100, max 1000)
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
    ///RFQ ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    ///RFQ status filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    ///Subaccount ID for auth purposes, returned data will be scoped to this subaccount.
    pub subaccount_id: i64,
    ///Latest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.
    #[serde(default = "defaults::default_u64::<i64, 18446744073709551615>")]
    pub to_timestamp: i64,
}
impl From<&PrivatePollRfqsParamsSchema> for PrivatePollRfqsParamsSchema {
    fn from(value: &PrivatePollRfqsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivatePollRfqsRequestSchema
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
      "const": "private/poll_rfqs"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivatePollRfqsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivatePollRfqsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivatePollRfqsRequestSchemaId>,
    pub method: String,
    pub params: PrivatePollRfqsParamsSchema,
}
impl From<&PrivatePollRfqsRequestSchema> for PrivatePollRfqsRequestSchema {
    fn from(value: &PrivatePollRfqsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivatePollRfqsRequestSchemaId
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
pub enum PrivatePollRfqsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivatePollRfqsRequestSchemaId> for PrivatePollRfqsRequestSchemaId {
    fn from(value: &PrivatePollRfqsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivatePollRfqsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivatePollRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivatePollRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivatePollRfqsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivatePollRfqsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivatePollRfqsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivatePollRfqsResponseSchema
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
      "$ref": "#/definitions/PrivatePollRfqsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivatePollRfqsResponseSchema {
    pub id: PrivatePollRfqsResponseSchemaId,
    ///
    pub result: PrivatePollRfqsResultSchema,
}
impl From<&PrivatePollRfqsResponseSchema> for PrivatePollRfqsResponseSchema {
    fn from(value: &PrivatePollRfqsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivatePollRfqsResponseSchemaId
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
pub enum PrivatePollRfqsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivatePollRfqsResponseSchemaId> for PrivatePollRfqsResponseSchemaId {
    fn from(value: &PrivatePollRfqsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivatePollRfqsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivatePollRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivatePollRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivatePollRfqsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivatePollRfqsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivatePollRfqsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivatePollRfqsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "pagination",
    "rfqs"
  ],
  "properties": {
    "pagination": {
      "description": "Pagination info",
      "type": "object",
      "$ref": "#/definitions/PaginationInfoSchema",
      "field_many": false
    },
    "rfqs": {
      "title": "rfqs",
      "description": "RFQs matching filter criteria",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/RFQResultPublicSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivatePollRfqsResultSchema {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///RFQs matching filter criteria
    pub rfqs: Vec<RfqResultPublicSchema>,
}
impl From<&PrivatePollRfqsResultSchema> for PrivatePollRfqsResultSchema {
    fn from(value: &PrivatePollRfqsResultSchema) -> Self {
        value.clone()
    }
}
///RfqResultPublicSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancel_reason",
    "creation_timestamp",
    "last_update_timestamp",
    "legs",
    "rfq_id",
    "status",
    "subaccount_id",
    "valid_until"
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
    "last_update_timestamp": {
      "title": "last_update_timestamp",
      "description": "Last update timestamp in ms since Unix epoch",
      "type": "integer"
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
    "valid_until": {
      "title": "valid_until",
      "description": "RFQ expiry timestamp in ms since Unix epoch",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct RfqResultPublicSchema {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///RFQ legs
    pub legs: Vec<LegUnpricedSchema>,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Status
    pub status: Status,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///RFQ expiry timestamp in ms since Unix epoch
    pub valid_until: i64,
}
impl From<&RfqResultPublicSchema> for RfqResultPublicSchema {
    fn from(value: &RfqResultPublicSchema) -> Self {
        value.clone()
    }
}
///RFQ status filter, if applicable
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "RFQ status filter, if applicable",
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
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
