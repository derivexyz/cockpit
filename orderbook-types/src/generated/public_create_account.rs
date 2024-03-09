#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Create a new account
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/create_account",
  "description": "Create a new account",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicCreateAccountJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicCreateAccount(pub PublicCreateAccountJsonrpcSchema);
impl std::ops::Deref for PublicCreateAccount {
    type Target = PublicCreateAccountJsonrpcSchema;
    fn deref(&self) -> &PublicCreateAccountJsonrpcSchema {
        &self.0
    }
}
impl From<PublicCreateAccount> for PublicCreateAccountJsonrpcSchema {
    fn from(value: PublicCreateAccount) -> Self {
        value.0
    }
}
impl From<&PublicCreateAccount> for PublicCreateAccount {
    fn from(value: &PublicCreateAccount) -> Self {
        value.clone()
    }
}
impl From<PublicCreateAccountJsonrpcSchema> for PublicCreateAccount {
    fn from(value: PublicCreateAccountJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicCreateAccountJsonrpcSchema
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
      "$ref": "#/definitions/PublicCreateAccountRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicCreateAccountResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateAccountJsonrpcSchema {
    pub request: PublicCreateAccountRequestSchema,
    pub response: PublicCreateAccountResponseSchema,
}
impl From<&PublicCreateAccountJsonrpcSchema> for PublicCreateAccountJsonrpcSchema {
    fn from(value: &PublicCreateAccountJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicCreateAccountParamsSchema
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
      "description": "Ethereum wallet address",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateAccountParamsSchema {
    ///Ethereum wallet address
    pub wallet: String,
}
impl From<&PublicCreateAccountParamsSchema> for PublicCreateAccountParamsSchema {
    fn from(value: &PublicCreateAccountParamsSchema) -> Self {
        value.clone()
    }
}
///PublicCreateAccountRequestSchema
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
      "const": "public/create_account"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicCreateAccountParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateAccountRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicCreateAccountRequestSchemaId>,
    pub method: String,
    pub params: PublicCreateAccountParamsSchema,
}
impl From<&PublicCreateAccountRequestSchema> for PublicCreateAccountRequestSchema {
    fn from(value: &PublicCreateAccountRequestSchema) -> Self {
        value.clone()
    }
}
///PublicCreateAccountRequestSchemaId
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
pub enum PublicCreateAccountRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicCreateAccountRequestSchemaId> for PublicCreateAccountRequestSchemaId {
    fn from(value: &PublicCreateAccountRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicCreateAccountRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicCreateAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicCreateAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicCreateAccountRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicCreateAccountRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicCreateAccountRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicCreateAccountResponseSchema
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
      "$ref": "#/definitions/PublicCreateAccountResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicCreateAccountResponseSchema {
    pub id: PublicCreateAccountResponseSchemaId,
    ///
    pub result: PublicCreateAccountResultSchema,
}
impl From<&PublicCreateAccountResponseSchema> for PublicCreateAccountResponseSchema {
    fn from(value: &PublicCreateAccountResponseSchema) -> Self {
        value.clone()
    }
}
///PublicCreateAccountResponseSchemaId
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
pub enum PublicCreateAccountResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicCreateAccountResponseSchemaId> for PublicCreateAccountResponseSchemaId {
    fn from(value: &PublicCreateAccountResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicCreateAccountResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicCreateAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicCreateAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicCreateAccountResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicCreateAccountResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicCreateAccountResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicCreateAccountResultSchema
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
      "description": "`created` or `exists`",
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

pub struct PublicCreateAccountResultSchema {
    ///`created` or `exists`
    pub status: String,
    ///Ethereum wallet address
    pub wallet: String,
}
impl From<&PublicCreateAccountResultSchema> for PublicCreateAccountResultSchema {
    fn from(value: &PublicCreateAccountResultSchema) -> Self {
        value.clone()
    }
}
