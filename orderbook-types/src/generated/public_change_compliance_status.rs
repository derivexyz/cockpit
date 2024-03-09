#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get the value history of a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/change_compliance_status",
  "description": "Get the value history of a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicChangeComplianceStatusJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicChangeComplianceStatus(pub PublicChangeComplianceStatusJsonrpcSchema);
impl std::ops::Deref for PublicChangeComplianceStatus {
    type Target = PublicChangeComplianceStatusJsonrpcSchema;
    fn deref(&self) -> &PublicChangeComplianceStatusJsonrpcSchema {
        &self.0
    }
}
impl From<PublicChangeComplianceStatus> for PublicChangeComplianceStatusJsonrpcSchema {
    fn from(value: PublicChangeComplianceStatus) -> Self {
        value.0
    }
}
impl From<&PublicChangeComplianceStatus> for PublicChangeComplianceStatus {
    fn from(value: &PublicChangeComplianceStatus) -> Self {
        value.clone()
    }
}
impl From<PublicChangeComplianceStatusJsonrpcSchema> for PublicChangeComplianceStatus {
    fn from(value: PublicChangeComplianceStatusJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicChangeComplianceStatusJsonrpcSchema
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
      "$ref": "#/definitions/PublicChangeComplianceStatusRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicChangeComplianceStatusResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicChangeComplianceStatusJsonrpcSchema {
    pub request: PublicChangeComplianceStatusRequestSchema,
    pub response: PublicChangeComplianceStatusResponseSchema,
}
impl From<&PublicChangeComplianceStatusJsonrpcSchema>
for PublicChangeComplianceStatusJsonrpcSchema {
    fn from(value: &PublicChangeComplianceStatusJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicChangeComplianceStatusParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "status",
    "wallet"
  ],
  "properties": {
    "status": {
      "title": "status",
      "description": "Compliance status enum: enabled, disabled, force_enabled, force_disabled",
      "type": "string",
      "enum": [
        "enabled",
        "disabled"
      ]
    },
    "wallet": {
      "title": "wallet",
      "description": "Wallet address of the account owner",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicChangeComplianceStatusParamsSchema {
    ///Compliance status enum: enabled, disabled, force_enabled, force_disabled
    pub status: Status,
    ///Wallet address of the account owner
    pub wallet: String,
}
impl From<&PublicChangeComplianceStatusParamsSchema>
for PublicChangeComplianceStatusParamsSchema {
    fn from(value: &PublicChangeComplianceStatusParamsSchema) -> Self {
        value.clone()
    }
}
///PublicChangeComplianceStatusRequestSchema
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
      "const": "public/change_compliance_status"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicChangeComplianceStatusParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicChangeComplianceStatusRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicChangeComplianceStatusRequestSchemaId>,
    pub method: String,
    pub params: PublicChangeComplianceStatusParamsSchema,
}
impl From<&PublicChangeComplianceStatusRequestSchema>
for PublicChangeComplianceStatusRequestSchema {
    fn from(value: &PublicChangeComplianceStatusRequestSchema) -> Self {
        value.clone()
    }
}
///PublicChangeComplianceStatusRequestSchemaId
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
pub enum PublicChangeComplianceStatusRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicChangeComplianceStatusRequestSchemaId>
for PublicChangeComplianceStatusRequestSchemaId {
    fn from(value: &PublicChangeComplianceStatusRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicChangeComplianceStatusRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicChangeComplianceStatusRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicChangeComplianceStatusRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicChangeComplianceStatusRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicChangeComplianceStatusRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicChangeComplianceStatusRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicChangeComplianceStatusResponseSchema
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
      "$ref": "#/definitions/PublicChangeComplianceStatusResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicChangeComplianceStatusResponseSchema {
    pub id: PublicChangeComplianceStatusResponseSchemaId,
    ///
    pub result: PublicChangeComplianceStatusResultSchema,
}
impl From<&PublicChangeComplianceStatusResponseSchema>
for PublicChangeComplianceStatusResponseSchema {
    fn from(value: &PublicChangeComplianceStatusResponseSchema) -> Self {
        value.clone()
    }
}
///PublicChangeComplianceStatusResponseSchemaId
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
pub enum PublicChangeComplianceStatusResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicChangeComplianceStatusResponseSchemaId>
for PublicChangeComplianceStatusResponseSchemaId {
    fn from(value: &PublicChangeComplianceStatusResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicChangeComplianceStatusResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicChangeComplianceStatusResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicChangeComplianceStatusResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicChangeComplianceStatusResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicChangeComplianceStatusResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicChangeComplianceStatusResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicChangeComplianceStatusResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "result"
  ],
  "properties": {
    "orders_cancelled": {
      "title": "orders_cancelled",
      "description": "Number of orders cancelled",
      "default": 0,
      "type": "integer"
    },
    "result": {
      "title": "result",
      "description": "Result of the operation",
      "type": "string"
    },
    "subaccounts_affected": {
      "title": "subaccounts_affected",
      "description": "Number of subaccounts affected",
      "default": 0,
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicChangeComplianceStatusResultSchema {
    ///Number of orders cancelled
    #[serde(default)]
    pub orders_cancelled: i64,
    ///Result of the operation
    pub result: String,
    ///Number of subaccounts affected
    #[serde(default)]
    pub subaccounts_affected: i64,
}
impl From<&PublicChangeComplianceStatusResultSchema>
for PublicChangeComplianceStatusResultSchema {
    fn from(value: &PublicChangeComplianceStatusResultSchema) -> Self {
        value.clone()
    }
}
///Compliance status enum: enabled, disabled, force_enabled, force_disabled
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "status",
  "description": "Compliance status enum: enabled, disabled, force_enabled, force_disabled",
  "type": "string",
  "enum": [
    "enabled",
    "disabled"
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
    #[serde(rename = "enabled")]
    Enabled,
    #[serde(rename = "disabled")]
    Disabled,
}
impl From<&Status> for Status {
    fn from(value: &Status) -> Self {
        value.clone()
    }
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match *self {
            Self::Enabled => "enabled".to_string(),
            Self::Disabled => "disabled".to_string(),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "enabled" => Ok(Self::Enabled),
            "disabled" => Ok(Self::Disabled),
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
