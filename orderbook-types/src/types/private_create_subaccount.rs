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
///Create a new subaccount under a given wallet, and deposit an asset into that subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/create_subaccount",
  "description": "Create a new subaccount under a given wallet, and deposit an asset into that subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCreateSubaccountJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCreateSubaccount(pub PrivateCreateSubaccountJsonrpcSchema);
impl std::ops::Deref for PrivateCreateSubaccount {
    type Target = PrivateCreateSubaccountJsonrpcSchema;
    fn deref(&self) -> &PrivateCreateSubaccountJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCreateSubaccount> for PrivateCreateSubaccountJsonrpcSchema {
    fn from(value: PrivateCreateSubaccount) -> Self {
        value.0
    }
}
impl From<&PrivateCreateSubaccount> for PrivateCreateSubaccount {
    fn from(value: &PrivateCreateSubaccount) -> Self {
        value.clone()
    }
}
impl From<PrivateCreateSubaccountJsonrpcSchema> for PrivateCreateSubaccount {
    fn from(value: PrivateCreateSubaccountJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCreateSubaccountJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCreateSubaccountRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCreateSubaccountResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCreateSubaccountJsonrpcSchema {
    pub request: PrivateCreateSubaccountRequestSchema,
    pub response: PrivateCreateSubaccountResponseSchema,
}
impl From<&PrivateCreateSubaccountJsonrpcSchema>
for PrivateCreateSubaccountJsonrpcSchema {
    fn from(value: &PrivateCreateSubaccountJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCreateSubaccountParamsSchema
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
    "signature",
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
      "default": null,
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
    "signature": {
      "title": "signature",
      "description": "Ethereum signature of the deposit",
      "type": "string"
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

pub struct PrivateCreateSubaccountParamsSchema {
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
    ///Ethereum signature of the deposit
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be >5min from now
    pub signature_expiry_sec: i64,
    ///Ethereum wallet address that is signing the deposit
    pub signer: String,
    ///Ethereum wallet address
    pub wallet: String,
}
impl From<&PrivateCreateSubaccountParamsSchema> for PrivateCreateSubaccountParamsSchema {
    fn from(value: &PrivateCreateSubaccountParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCreateSubaccountRequestSchema
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
      "const": "private/create_subaccount"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCreateSubaccountParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCreateSubaccountRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCreateSubaccountRequestSchemaId>,
    pub method: String,
    pub params: PrivateCreateSubaccountParamsSchema,
}
impl From<&PrivateCreateSubaccountRequestSchema>
for PrivateCreateSubaccountRequestSchema {
    fn from(value: &PrivateCreateSubaccountRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCreateSubaccountRequestSchemaId
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
pub enum PrivateCreateSubaccountRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCreateSubaccountRequestSchemaId>
for PrivateCreateSubaccountRequestSchemaId {
    fn from(value: &PrivateCreateSubaccountRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCreateSubaccountRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCreateSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCreateSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCreateSubaccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCreateSubaccountRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCreateSubaccountRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCreateSubaccountResponseSchema
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
      "$ref": "#/definitions/PrivateCreateSubaccountResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCreateSubaccountResponseSchema {
    pub id: PrivateCreateSubaccountResponseSchemaId,
    ///
    pub result: PrivateCreateSubaccountResultSchema,
}
impl From<&PrivateCreateSubaccountResponseSchema>
for PrivateCreateSubaccountResponseSchema {
    fn from(value: &PrivateCreateSubaccountResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCreateSubaccountResponseSchemaId
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
pub enum PrivateCreateSubaccountResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCreateSubaccountResponseSchemaId>
for PrivateCreateSubaccountResponseSchemaId {
    fn from(value: &PrivateCreateSubaccountResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCreateSubaccountResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCreateSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCreateSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCreateSubaccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCreateSubaccountResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCreateSubaccountResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCreateSubaccountResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "status",
    "transaction_id"
  ],
  "properties": {
    "status": {
      "title": "status",
      "description": "`requested`",
      "type": "string"
    },
    "transaction_id": {
      "title": "transaction_id",
      "description": "Transaction id of the request",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCreateSubaccountResultSchema {
    ///`requested`
    pub status: String,
    ///Transaction id of the request
    pub transaction_id: uuid::Uuid,
}
impl From<&PrivateCreateSubaccountResultSchema> for PrivateCreateSubaccountResultSchema {
    fn from(value: &PrivateCreateSubaccountResultSchema) -> Self {
        value.clone()
    }
}
