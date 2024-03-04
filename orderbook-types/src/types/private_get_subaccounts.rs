#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Get all subaccounts of an account / wallet
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_subaccounts",
  "description": "Get all subaccounts of an account / wallet",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetSubaccountsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetSubaccounts(pub PrivateGetSubaccountsJsonrpcSchema);
impl std::ops::Deref for PrivateGetSubaccounts {
    type Target = PrivateGetSubaccountsJsonrpcSchema;
    fn deref(&self) -> &PrivateGetSubaccountsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetSubaccounts> for PrivateGetSubaccountsJsonrpcSchema {
    fn from(value: PrivateGetSubaccounts) -> Self {
        value.0
    }
}
impl From<&PrivateGetSubaccounts> for PrivateGetSubaccounts {
    fn from(value: &PrivateGetSubaccounts) -> Self {
        value.clone()
    }
}
impl From<PrivateGetSubaccountsJsonrpcSchema> for PrivateGetSubaccounts {
    fn from(value: PrivateGetSubaccountsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetSubaccountsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountsJsonrpcSchema {
    pub request: PrivateGetSubaccountsRequestSchema,
    pub response: PrivateGetSubaccountsResponseSchema,
}
impl From<&PrivateGetSubaccountsJsonrpcSchema> for PrivateGetSubaccountsJsonrpcSchema {
    fn from(value: &PrivateGetSubaccountsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "wallet"
  ],
  "properties": {
    "wallet": {
      "title": "wallet",
      "description": "Ethereum wallet address of account",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountsParamsSchema {
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PrivateGetSubaccountsParamsSchema> for PrivateGetSubaccountsParamsSchema {
    fn from(value: &PrivateGetSubaccountsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountsRequestSchema
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
      "const": "private/get_subaccounts"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetSubaccountsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetSubaccountsRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetSubaccountsParamsSchema,
}
impl From<&PrivateGetSubaccountsRequestSchema> for PrivateGetSubaccountsRequestSchema {
    fn from(value: &PrivateGetSubaccountsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountsRequestSchemaId
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
pub enum PrivateGetSubaccountsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountsRequestSchemaId>
for PrivateGetSubaccountsRequestSchemaId {
    fn from(value: &PrivateGetSubaccountsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetSubaccountsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountsResponseSchema
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
      "$ref": "#/definitions/PrivateGetSubaccountsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetSubaccountsResponseSchema {
    pub id: PrivateGetSubaccountsResponseSchemaId,
    ///
    pub result: PrivateGetSubaccountsResultSchema,
}
impl From<&PrivateGetSubaccountsResponseSchema> for PrivateGetSubaccountsResponseSchema {
    fn from(value: &PrivateGetSubaccountsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetSubaccountsResponseSchemaId
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
pub enum PrivateGetSubaccountsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetSubaccountsResponseSchemaId>
for PrivateGetSubaccountsResponseSchemaId {
    fn from(value: &PrivateGetSubaccountsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetSubaccountsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetSubaccountsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetSubaccountsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetSubaccountsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetSubaccountsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetSubaccountsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetSubaccountsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_ids",
    "wallet"
  ],
  "properties": {
    "subaccount_ids": {
      "title": "subaccount_ids",
      "description": "List of subaccount_ids owned by the wallet in `SubAccounts.sol`",
      "type": "array",
      "items": {
        "title": "subaccount_ids",
        "type": "integer"
      }
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

pub struct PrivateGetSubaccountsResultSchema {
    ///List of subaccount_ids owned by the wallet in `SubAccounts.sol`
    pub subaccount_ids: Vec<i64>,
    ///Ethereum wallet address
    pub wallet: String,
}
impl From<&PrivateGetSubaccountsResultSchema> for PrivateGetSubaccountsResultSchema {
    fn from(value: &PrivateGetSubaccountsResultSchema) -> Self {
        value.clone()
    }
}
