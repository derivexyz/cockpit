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
///Requests two-sided quotes from participating market makers.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/send_rfq",
  "description": "Requests two-sided quotes from participating market makers.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateSendRfqJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateSendRfq(pub PrivateSendRfqJsonrpcSchema);
impl std::ops::Deref for PrivateSendRfq {
    type Target = PrivateSendRfqJsonrpcSchema;
    fn deref(&self) -> &PrivateSendRfqJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateSendRfq> for PrivateSendRfqJsonrpcSchema {
    fn from(value: PrivateSendRfq) -> Self {
        value.0
    }
}
impl From<&PrivateSendRfq> for PrivateSendRfq {
    fn from(value: &PrivateSendRfq) -> Self {
        value.clone()
    }
}
impl From<PrivateSendRfqJsonrpcSchema> for PrivateSendRfq {
    fn from(value: PrivateSendRfqJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateSendRfqJsonrpcSchema
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
      "$ref": "#/definitions/PrivateSendRfqRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateSendRfqResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSendRfqJsonrpcSchema {
    pub request: PrivateSendRfqRequestSchema,
    pub response: PrivateSendRfqResponseSchema,
}
impl From<&PrivateSendRfqJsonrpcSchema> for PrivateSendRfqJsonrpcSchema {
    fn from(value: &PrivateSendRfqJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateSendRfqParamsSchema
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
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "min_total_cost": {
      "title": "min_total_cost",
      "description": "An optional min total cost for the RFQ. Only used when the RFQ sender executes as seller. Polling endpoints and channels will ignore quotes where the total cost across all legs is below this value. Positive values mean the RFQ sender expects to receive $, negative mean the RFQ sender expects to pay $.This field is not disclosed to the market makers.",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
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

pub struct PrivateSendRfqParamsSchema {
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
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateSendRfqParamsSchema> for PrivateSendRfqParamsSchema {
    fn from(value: &PrivateSendRfqParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateSendRfqRequestSchema
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
      "const": "private/send_rfq"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateSendRfqParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSendRfqRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateSendRfqRequestSchemaId>,
    pub method: String,
    pub params: PrivateSendRfqParamsSchema,
}
impl From<&PrivateSendRfqRequestSchema> for PrivateSendRfqRequestSchema {
    fn from(value: &PrivateSendRfqRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateSendRfqRequestSchemaId
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
pub enum PrivateSendRfqRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSendRfqRequestSchemaId> for PrivateSendRfqRequestSchemaId {
    fn from(value: &PrivateSendRfqRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSendRfqRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSendRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSendRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSendRfqRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSendRfqRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSendRfqRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSendRfqResponseSchema
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
      "$ref": "#/definitions/PrivateSendRfqResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSendRfqResponseSchema {
    pub id: PrivateSendRfqResponseSchemaId,
    ///
    pub result: PrivateSendRfqResultSchema,
}
impl From<&PrivateSendRfqResponseSchema> for PrivateSendRfqResponseSchema {
    fn from(value: &PrivateSendRfqResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateSendRfqResponseSchemaId
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
pub enum PrivateSendRfqResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSendRfqResponseSchemaId> for PrivateSendRfqResponseSchemaId {
    fn from(value: &PrivateSendRfqResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSendRfqResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateSendRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSendRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSendRfqResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSendRfqResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSendRfqResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSendRfqResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancel_reason",
    "creation_timestamp",
    "label",
    "last_update_timestamp",
    "legs",
    "max_total_cost",
    "min_total_cost",
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
      "description": "Max total cost for the RFQ",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "min_total_cost": {
      "title": "min_total_cost",
      "description": "Min total cost for the RFQ",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
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

pub struct PrivateSendRfqResultSchema {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///User-defined label, if any
    pub label: String,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///RFQ legs
    pub legs: Vec<LegUnpricedSchema>,
    ///Max total cost for the RFQ
    pub max_total_cost: Option<bigdecimal::BigDecimal>,
    ///Min total cost for the RFQ
    pub min_total_cost: Option<bigdecimal::BigDecimal>,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Status
    pub status: Status,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///RFQ expiry timestamp in ms since Unix epoch
    pub valid_until: i64,
}
impl From<&PrivateSendRfqResultSchema> for PrivateSendRfqResultSchema {
    fn from(value: &PrivateSendRfqResultSchema) -> Self {
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
