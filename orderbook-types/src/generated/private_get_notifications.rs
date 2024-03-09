#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///NotificationResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "event",
    "event_details",
    "id",
    "status",
    "subaccount_id",
    "timestamp"
  ],
  "properties": {
    "event": {
      "title": "event",
      "description": "The specific event leading to the notification.",
      "type": "string"
    },
    "event_details": {
      "title": "event_details",
      "description": "A JSON-structured dictionary containing detailed data or context about the event.",
      "type": "object",
      "additionalProperties": {}
    },
    "id": {
      "title": "id",
      "description": "The unique identifier for the notification.",
      "type": "integer"
    },
    "status": {
      "title": "status",
      "description": "The status of the notification, indicating if it has been read, pending, or processed.",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "The subaccount_id associated with the notification.",
      "type": "integer"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "The timestamp indicating when the notification was created or triggered.",
      "type": "integer"
    },
    "transaction_id": {
      "title": "transaction_id",
      "description": "The transaction id associated with the notification.",
      "type": [
        "integer",
        "null"
      ]
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "The transaction hash associated with the notification.",
      "type": [
        "string",
        "null"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct NotificationResponseSchema {
    ///The specific event leading to the notification.
    pub event: String,
    ///A JSON-structured dictionary containing detailed data or context about the event.
    pub event_details: serde_json::Map<String, serde_json::Value>,
    ///The unique identifier for the notification.
    pub id: i64,
    ///The status of the notification, indicating if it has been read, pending, or processed.
    pub status: String,
    ///The subaccount_id associated with the notification.
    pub subaccount_id: i64,
    ///The timestamp indicating when the notification was created or triggered.
    pub timestamp: i64,
    ///The transaction id associated with the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<i64>,
    ///The transaction hash associated with the notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
}
impl From<&NotificationResponseSchema> for NotificationResponseSchema {
    fn from(value: &NotificationResponseSchema) -> Self {
        value.clone()
    }
}
///Get the notifications related to a subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_notifications",
  "description": "Get the notifications related to a subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetNotificationsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetNotifications(pub PrivateGetNotificationsJsonrpcSchema);
impl std::ops::Deref for PrivateGetNotifications {
    type Target = PrivateGetNotificationsJsonrpcSchema;
    fn deref(&self) -> &PrivateGetNotificationsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetNotifications> for PrivateGetNotificationsJsonrpcSchema {
    fn from(value: PrivateGetNotifications) -> Self {
        value.0
    }
}
impl From<&PrivateGetNotifications> for PrivateGetNotifications {
    fn from(value: &PrivateGetNotifications) -> Self {
        value.clone()
    }
}
impl From<PrivateGetNotificationsJsonrpcSchema> for PrivateGetNotifications {
    fn from(value: PrivateGetNotificationsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetNotificationsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetNotificationsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetNotificationsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetNotificationsJsonrpcSchema {
    pub request: PrivateGetNotificationsRequestSchema,
    pub response: PrivateGetNotificationsResponseSchema,
}
impl From<&PrivateGetNotificationsJsonrpcSchema>
for PrivateGetNotificationsJsonrpcSchema {
    fn from(value: &PrivateGetNotificationsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetNotificationsParamsSchema
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
    "page": {
      "title": "page",
      "description": "Page number of results to return",
      "default": 1,
      "type": "integer"
    },
    "page_size": {
      "title": "page_size",
      "description": "Number of results per page",
      "default": 20,
      "type": "integer"
    },
    "status": {
      "title": "status",
      "description": "Status of the notification",
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "unseen",
        "seen",
        "hidden"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "type": {
      "title": "type",
      "description": "Status of the notification",
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "deposit",
        "withdraw",
        "transfer",
        "trade",
        "settlement",
        "liquidation",
        "types"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetNotificationsParamsSchema {
    ///Page number of results to return
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page
    #[serde(default = "defaults::default_u64::<i64, 20>")]
    pub page_size: i64,
    ///Status of the notification
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Status of the notification
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}
impl From<&PrivateGetNotificationsParamsSchema> for PrivateGetNotificationsParamsSchema {
    fn from(value: &PrivateGetNotificationsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetNotificationsRequestSchema
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
      "const": "private/get_notifications"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetNotificationsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetNotificationsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetNotificationsRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetNotificationsParamsSchema,
}
impl From<&PrivateGetNotificationsRequestSchema>
for PrivateGetNotificationsRequestSchema {
    fn from(value: &PrivateGetNotificationsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetNotificationsRequestSchemaId
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
pub enum PrivateGetNotificationsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetNotificationsRequestSchemaId>
for PrivateGetNotificationsRequestSchemaId {
    fn from(value: &PrivateGetNotificationsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetNotificationsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetNotificationsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetNotificationsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetNotificationsResponseSchema
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
      "$ref": "#/definitions/PrivateGetNotificationsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetNotificationsResponseSchema {
    pub id: PrivateGetNotificationsResponseSchemaId,
    ///
    pub result: PrivateGetNotificationsResultSchema,
}
impl From<&PrivateGetNotificationsResponseSchema>
for PrivateGetNotificationsResponseSchema {
    fn from(value: &PrivateGetNotificationsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetNotificationsResponseSchemaId
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
pub enum PrivateGetNotificationsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetNotificationsResponseSchemaId>
for PrivateGetNotificationsResponseSchemaId {
    fn from(value: &PrivateGetNotificationsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetNotificationsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetNotificationsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetNotificationsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetNotificationsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "notifications",
    "subaccount_id"
  ],
  "properties": {
    "notifications": {
      "title": "notifications",
      "description": "Notification response",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/NotificationResponseSchema",
        "field_many": true
      }
    },
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

pub struct PrivateGetNotificationsResultSchema {
    ///Notification response
    pub notifications: Vec<NotificationResponseSchema>,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetNotificationsResultSchema> for PrivateGetNotificationsResultSchema {
    fn from(value: &PrivateGetNotificationsResultSchema) -> Self {
        value.clone()
    }
}
///Status of the notification
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "Status of the notification",
  "type": "string",
  "enum": [
    "unseen",
    "seen",
    "hidden"
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
    #[serde(rename = "unseen")]
    Unseen,
    #[serde(rename = "seen")]
    Seen,
    #[serde(rename = "hidden")]
    Hidden,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match *self {
            Self::Unseen => "unseen".to_string(),
            Self::Seen => "seen".to_string(),
            Self::Hidden => "hidden".to_string(),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "unseen" => Ok(Self::Unseen),
            "seen" => Ok(Self::Seen),
            "hidden" => Ok(Self::Hidden),
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
///Status of the notification
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "type",
  "description": "Status of the notification",
  "type": "string",
  "enum": [
    "deposit",
    "withdraw",
    "transfer",
    "trade",
    "settlement",
    "liquidation",
    "types"
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
pub enum Type {
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "withdraw")]
    Withdraw,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "settlement")]
    Settlement,
    #[serde(rename = "liquidation")]
    Liquidation,
    #[serde(rename = "types")]
    Custom,
}
impl From<&Type> for Type {
    fn from(value: &Type) -> Self {
        value.clone()
    }
}
impl ToString for Type {
    fn to_string(&self) -> String {
        match *self {
            Self::Deposit => "deposit".to_string(),
            Self::Withdraw => "withdraw".to_string(),
            Self::Transfer => "transfer".to_string(),
            Self::Trade => "trade".to_string(),
            Self::Settlement => "settlement".to_string(),
            Self::Liquidation => "liquidation".to_string(),
            Self::Custom => "types".to_string(),
        }
    }
}
impl std::str::FromStr for Type {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "deposit" => Ok(Self::Deposit),
            "withdraw" => Ok(Self::Withdraw),
            "transfer" => Ok(Self::Transfer),
            "trade" => Ok(Self::Trade),
            "settlement" => Ok(Self::Settlement),
            "liquidation" => Ok(Self::Liquidation),
            "types" => Ok(Self::Custom),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Type {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Type {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Type {
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
