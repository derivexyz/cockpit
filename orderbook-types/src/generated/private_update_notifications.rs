#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///RPC to mark specified notifications as seen for a given subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/update_notifications",
  "description": "RPC to mark specified notifications as seen for a given subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateUpdateNotificationsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateUpdateNotifications(pub PrivateUpdateNotificationsJsonrpcSchema);
impl std::ops::Deref for PrivateUpdateNotifications {
    type Target = PrivateUpdateNotificationsJsonrpcSchema;
    fn deref(&self) -> &PrivateUpdateNotificationsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateUpdateNotifications> for PrivateUpdateNotificationsJsonrpcSchema {
    fn from(value: PrivateUpdateNotifications) -> Self {
        value.0
    }
}
impl From<&PrivateUpdateNotifications> for PrivateUpdateNotifications {
    fn from(value: &PrivateUpdateNotifications) -> Self {
        value.clone()
    }
}
impl From<PrivateUpdateNotificationsJsonrpcSchema> for PrivateUpdateNotifications {
    fn from(value: PrivateUpdateNotificationsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateUpdateNotificationsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateUpdateNotificationsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateUpdateNotificationsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateUpdateNotificationsJsonrpcSchema {
    pub request: PrivateUpdateNotificationsRequestSchema,
    pub response: PrivateUpdateNotificationsResponseSchema,
}
impl From<&PrivateUpdateNotificationsJsonrpcSchema>
for PrivateUpdateNotificationsJsonrpcSchema {
    fn from(value: &PrivateUpdateNotificationsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateUpdateNotificationsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "notification_ids",
    "subaccount_id"
  ],
  "properties": {
    "notification_ids": {
      "title": "notification_ids",
      "description": "List of notification IDs to be marked as seen",
      "type": "array",
      "items": {
        "title": "notification_ids",
        "type": "integer"
      }
    },
    "status": {
      "title": "status",
      "description": "Status of the notification",
      "default": "seen",
      "type": "string",
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
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateUpdateNotificationsParamsSchema {
    ///List of notification IDs to be marked as seen
    pub notification_ids: Vec<i64>,
    ///Status of the notification
    #[serde(default = "defaults::private_update_notifications_params_schema_status")]
    pub status: Status,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateUpdateNotificationsParamsSchema>
for PrivateUpdateNotificationsParamsSchema {
    fn from(value: &PrivateUpdateNotificationsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateUpdateNotificationsRequestSchema
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
      "const": "private/update_notifications"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateUpdateNotificationsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateUpdateNotificationsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateUpdateNotificationsRequestSchemaId>,
    pub method: String,
    pub params: PrivateUpdateNotificationsParamsSchema,
}
impl From<&PrivateUpdateNotificationsRequestSchema>
for PrivateUpdateNotificationsRequestSchema {
    fn from(value: &PrivateUpdateNotificationsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateUpdateNotificationsRequestSchemaId
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
pub enum PrivateUpdateNotificationsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateUpdateNotificationsRequestSchemaId>
for PrivateUpdateNotificationsRequestSchemaId {
    fn from(value: &PrivateUpdateNotificationsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateUpdateNotificationsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateUpdateNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateUpdateNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateUpdateNotificationsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateUpdateNotificationsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateUpdateNotificationsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateUpdateNotificationsResponseSchema
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
      "$ref": "#/definitions/PrivateUpdateNotificationsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateUpdateNotificationsResponseSchema {
    pub id: PrivateUpdateNotificationsResponseSchemaId,
    ///
    pub result: PrivateUpdateNotificationsResultSchema,
}
impl From<&PrivateUpdateNotificationsResponseSchema>
for PrivateUpdateNotificationsResponseSchema {
    fn from(value: &PrivateUpdateNotificationsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateUpdateNotificationsResponseSchemaId
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
pub enum PrivateUpdateNotificationsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateUpdateNotificationsResponseSchemaId>
for PrivateUpdateNotificationsResponseSchemaId {
    fn from(value: &PrivateUpdateNotificationsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateUpdateNotificationsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateUpdateNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateUpdateNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateUpdateNotificationsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateUpdateNotificationsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateUpdateNotificationsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateUpdateNotificationsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "updated_count"
  ],
  "properties": {
    "updated_count": {
      "title": "updated_count",
      "description": "Number of notifications marked as seen",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateUpdateNotificationsResultSchema {
    ///Number of notifications marked as seen
    pub updated_count: i64,
}
impl From<&PrivateUpdateNotificationsResultSchema>
for PrivateUpdateNotificationsResultSchema {
    fn from(value: &PrivateUpdateNotificationsResultSchema) -> Self {
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
  "default": "seen",
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
impl Default for Status {
    fn default() -> Self {
        Status::Seen
    }
}
pub mod defaults {
    pub(super) fn private_update_notifications_params_schema_status() -> super::Status {
        super::Status::Seen
    }
}
