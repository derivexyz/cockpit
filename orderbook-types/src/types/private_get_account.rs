#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Account details getter
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_account",
  "description": "Account details getter",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetAccountJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetAccount(pub PrivateGetAccountJsonrpcSchema);
impl std::ops::Deref for PrivateGetAccount {
    type Target = PrivateGetAccountJsonrpcSchema;
    fn deref(&self) -> &PrivateGetAccountJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetAccount> for PrivateGetAccountJsonrpcSchema {
    fn from(value: PrivateGetAccount) -> Self {
        value.0
    }
}
impl From<&PrivateGetAccount> for PrivateGetAccount {
    fn from(value: &PrivateGetAccount) -> Self {
        value.clone()
    }
}
impl From<PrivateGetAccountJsonrpcSchema> for PrivateGetAccount {
    fn from(value: PrivateGetAccountJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetAccountJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetAccountRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetAccountResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetAccountJsonrpcSchema {
    pub request: PrivateGetAccountRequestSchema,
    pub response: PrivateGetAccountResponseSchema,
}
impl From<&PrivateGetAccountJsonrpcSchema> for PrivateGetAccountJsonrpcSchema {
    fn from(value: &PrivateGetAccountJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetAccountParamsSchema
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

pub struct PrivateGetAccountParamsSchema {
    ///Ethereum wallet address of account
    pub wallet: String,
}
impl From<&PrivateGetAccountParamsSchema> for PrivateGetAccountParamsSchema {
    fn from(value: &PrivateGetAccountParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetAccountRequestSchema
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
      "const": "private/get_account"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetAccountParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetAccountRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetAccountRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetAccountParamsSchema,
}
impl From<&PrivateGetAccountRequestSchema> for PrivateGetAccountRequestSchema {
    fn from(value: &PrivateGetAccountRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetAccountRequestSchemaId
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
pub enum PrivateGetAccountRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetAccountRequestSchemaId> for PrivateGetAccountRequestSchemaId {
    fn from(value: &PrivateGetAccountRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetAccountRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetAccountRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetAccountRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetAccountResponseSchema
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
      "$ref": "#/definitions/PrivateGetAccountResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetAccountResponseSchema {
    pub id: PrivateGetAccountResponseSchemaId,
    ///
    pub result: PrivateGetAccountResultSchema,
}
impl From<&PrivateGetAccountResponseSchema> for PrivateGetAccountResponseSchema {
    fn from(value: &PrivateGetAccountResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetAccountResponseSchemaId
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
pub enum PrivateGetAccountResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetAccountResponseSchemaId> for PrivateGetAccountResponseSchemaId {
    fn from(value: &PrivateGetAccountResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetAccountResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetAccountResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetAccountResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetAccountResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancel_on_disconnect",
    "subaccount_ids",
    "wallet",
    "websocket_matching_tps",
    "websocket_non_matching_tps",
    "websocket_option_tps",
    "websocket_perp_tps"
  ],
  "properties": {
    "cancel_on_disconnect": {
      "title": "cancel_on_disconnect",
      "description": "Whether cancel on disconnect is enabled for the account",
      "type": "boolean"
    },
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
    },
    "websocket_matching_tps": {
      "title": "websocket_matching_tps",
      "description": "Max transactions per second for matching requests over websocket (see `Rate Limiting` in docs)",
      "type": "integer"
    },
    "websocket_non_matching_tps": {
      "title": "websocket_non_matching_tps",
      "description": "Max transactions per second for non-matching requests over websocket (see `Rate Limiting` in docs)",
      "type": "integer"
    },
    "websocket_option_tps": {
      "title": "websocket_option_tps",
      "description": "Max transactions per second for EACH option instrument over websocket (see `Rate Limiting` in docs)",
      "type": "integer"
    },
    "websocket_perp_tps": {
      "title": "websocket_perp_tps",
      "description": "Max transactions per second for EACH perp instrument over websocket (see `Rate Limiting` in docs)",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetAccountResultSchema {
    ///Whether cancel on disconnect is enabled for the account
    pub cancel_on_disconnect: bool,
    ///List of subaccount_ids owned by the wallet in `SubAccounts.sol`
    pub subaccount_ids: Vec<i64>,
    ///Ethereum wallet address
    pub wallet: String,
    ///Max transactions per second for matching requests over websocket (see `Rate Limiting` in docs)
    pub websocket_matching_tps: i64,
    ///Max transactions per second for non-matching requests over websocket (see `Rate Limiting` in docs)
    pub websocket_non_matching_tps: i64,
    ///Max transactions per second for EACH option instrument over websocket (see `Rate Limiting` in docs)
    pub websocket_option_tps: i64,
    ///Max transactions per second for EACH perp instrument over websocket (see `Rate Limiting` in docs)
    pub websocket_perp_tps: i64,
}
impl From<&PrivateGetAccountResultSchema> for PrivateGetAccountResultSchema {
    fn from(value: &PrivateGetAccountResultSchema) -> Self {
        value.clone()
    }
}
