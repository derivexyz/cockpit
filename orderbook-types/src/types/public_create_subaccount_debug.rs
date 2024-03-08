#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///`PM` (Portfolio Margin) or `SM` (Standard Margin)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "margin_type",
  "description": "`PM` (Portfolio Margin) or `SM` (Standard Margin)",
  "type": "string",
  "enum": [
    "PM",
    "SM"
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
pub enum MarginType {
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SM")]
    Sm,
}
impl From<&MarginType> for MarginType {
    fn from(value: &MarginType) -> Self {
        value.clone()
    }
}
impl ToString for MarginType {
    fn to_string(&self) -> String {
        match *self {
            Self::Pm => "PM".to_string(),
            Self::Sm => "SM".to_string(),
        }
    }
}
impl std::str::FromStr for MarginType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "PM" => Ok(Self::Pm),
            "SM" => Ok(Self::Sm),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for MarginType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MarginType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MarginType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Used for debugging only, do not use in production. Will return the incremental encoded and hashed data.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/create_subaccount_debug",
  "description": "Used for debugging only, do not use in production. Will return the incremental encoded and hashed data.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicCreateSubaccountDebugJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicCreateSubaccountDebug(pub PublicCreateSubaccountDebugJsonrpcSchema);
impl std::ops::Deref for PublicCreateSubaccountDebug {
    type Target = PublicCreateSubaccountDebugJsonrpcSchema;
    fn deref(&self) -> &PublicCreateSubaccountDebugJsonrpcSchema {
        &self.0
    }
}
impl From<PublicCreateSubaccountDebug> for PublicCreateSubaccountDebugJsonrpcSchema {
    fn from(value: PublicCreateSubaccountDebug) -> Self {
        value.0
    }
}
impl From<&PublicCreateSubaccountDebug> for PublicCreateSubaccountDebug {
    fn from(value: &PublicCreateSubaccountDebug) -> Self {
        value.clone()
    }
}
impl From<PublicCreateSubaccountDebugJsonrpcSchema> for PublicCreateSubaccountDebug {
    fn from(value: PublicCreateSubaccountDebugJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicCreateSubaccountDebugJsonrpcSchema
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
      "$ref": "#/definitions/PublicCreateSubaccountDebugRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicCreateSubaccountDebugResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateSubaccountDebugJsonrpcSchema {
    pub request: PublicCreateSubaccountDebugRequestSchema,
    pub response: PublicCreateSubaccountDebugResponseSchema,
}
impl From<&PublicCreateSubaccountDebugJsonrpcSchema>
for PublicCreateSubaccountDebugJsonrpcSchema {
    fn from(value: &PublicCreateSubaccountDebugJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicCreateSubaccountDebugParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset_name",
    "margin_type",
    "nonce",
    "signature_expiry_sec",
    "signer",
    "wallet"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount of the asset to deposit",
      "type": "string",
      "format": "decimal"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Name of asset to deposit",
      "type": "string"
    },
    "currency": {
      "title": "currency",
      "description": "Base currency of the subaccount (only for `PM`)",
      "type": [
        "string",
        "null"
      ]
    },
    "margin_type": {
      "title": "margin_type",
      "description": "`PM` (Portfolio Margin) or `SM` (Standard Margin)",
      "type": "string",
      "enum": [
        "PM",
        "SM"
      ]
    },
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds. Expiry MUST be >5min from now",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Ethereum wallet address that is signing the deposit",
      "type": "string"
    },
    "wallet": {
      "title": "wallet",
      "description": "Ethereum wallet address",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateSubaccountDebugParamsSchema {
    ///Amount of the asset to deposit
    pub amount: bigdecimal::BigDecimal,
    ///Name of asset to deposit
    pub asset_name: String,
    ///Base currency of the subaccount (only for `PM`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///`PM` (Portfolio Margin) or `SM` (Standard Margin)
    pub margin_type: MarginType,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Unix timestamp in seconds. Expiry MUST be >5min from now
    pub signature_expiry_sec: i64,
    ///Ethereum wallet address that is signing the deposit
    pub signer: String,
    ///Ethereum wallet address
    pub wallet: String,
}
impl From<&PublicCreateSubaccountDebugParamsSchema>
for PublicCreateSubaccountDebugParamsSchema {
    fn from(value: &PublicCreateSubaccountDebugParamsSchema) -> Self {
        value.clone()
    }
}
///PublicCreateSubaccountDebugRequestSchema
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
      "const": "public/create_subaccount_debug"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicCreateSubaccountDebugParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateSubaccountDebugRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicCreateSubaccountDebugRequestSchemaId>,
    pub method: String,
    pub params: PublicCreateSubaccountDebugParamsSchema,
}
impl From<&PublicCreateSubaccountDebugRequestSchema>
for PublicCreateSubaccountDebugRequestSchema {
    fn from(value: &PublicCreateSubaccountDebugRequestSchema) -> Self {
        value.clone()
    }
}
///PublicCreateSubaccountDebugRequestSchemaId
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
pub enum PublicCreateSubaccountDebugRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicCreateSubaccountDebugRequestSchemaId>
for PublicCreateSubaccountDebugRequestSchemaId {
    fn from(value: &PublicCreateSubaccountDebugRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicCreateSubaccountDebugRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicCreateSubaccountDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicCreateSubaccountDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicCreateSubaccountDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicCreateSubaccountDebugRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicCreateSubaccountDebugRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicCreateSubaccountDebugResponseSchema
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
      "$ref": "#/definitions/PublicCreateSubaccountDebugResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateSubaccountDebugResponseSchema {
    pub id: PublicCreateSubaccountDebugResponseSchemaId,
    ///
    pub result: PublicCreateSubaccountDebugResultSchema,
}
impl From<&PublicCreateSubaccountDebugResponseSchema>
for PublicCreateSubaccountDebugResponseSchema {
    fn from(value: &PublicCreateSubaccountDebugResponseSchema) -> Self {
        value.clone()
    }
}
///PublicCreateSubaccountDebugResponseSchemaId
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
pub enum PublicCreateSubaccountDebugResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicCreateSubaccountDebugResponseSchemaId>
for PublicCreateSubaccountDebugResponseSchemaId {
    fn from(value: &PublicCreateSubaccountDebugResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicCreateSubaccountDebugResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicCreateSubaccountDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicCreateSubaccountDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicCreateSubaccountDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicCreateSubaccountDebugResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicCreateSubaccountDebugResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicCreateSubaccountDebugResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "action_hash",
    "encoded_data",
    "encoded_data_hashed",
    "typed_data_hash"
  ],
  "properties": {
    "action_hash": {
      "title": "action_hash",
      "description": "Keccak hashed action data",
      "type": "string"
    },
    "encoded_data": {
      "title": "encoded_data",
      "description": "ABI encoded deposit data",
      "type": "string"
    },
    "encoded_data_hashed": {
      "title": "encoded_data_hashed",
      "description": "Keccak hashed encoded_data",
      "type": "string"
    },
    "typed_data_hash": {
      "title": "typed_data_hash",
      "description": "EIP 712 typed data hash",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateSubaccountDebugResultSchema {
    ///Keccak hashed action data
    pub action_hash: String,
    ///ABI encoded deposit data
    pub encoded_data: String,
    ///Keccak hashed encoded_data
    pub encoded_data_hashed: String,
    ///EIP 712 typed data hash
    pub typed_data_hash: String,
}
impl From<&PublicCreateSubaccountDebugResultSchema>
for PublicCreateSubaccountDebugResultSchema {
    fn from(value: &PublicCreateSubaccountDebugResultSchema) -> Self {
        value.clone()
    }
}
