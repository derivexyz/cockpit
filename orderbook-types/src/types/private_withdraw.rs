#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Withdraw an asset to wallet.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/withdraw",
  "description": "Withdraw an asset to wallet.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateWithdrawJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateWithdraw(pub PrivateWithdrawJsonrpcSchema);
impl std::ops::Deref for PrivateWithdraw {
    type Target = PrivateWithdrawJsonrpcSchema;
    fn deref(&self) -> &PrivateWithdrawJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateWithdraw> for PrivateWithdrawJsonrpcSchema {
    fn from(value: PrivateWithdraw) -> Self {
        value.0
    }
}
impl From<&PrivateWithdraw> for PrivateWithdraw {
    fn from(value: &PrivateWithdraw) -> Self {
        value.clone()
    }
}
impl From<PrivateWithdrawJsonrpcSchema> for PrivateWithdraw {
    fn from(value: PrivateWithdrawJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateWithdrawJsonrpcSchema
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
      "$ref": "#/definitions/PrivateWithdrawRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateWithdrawResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateWithdrawJsonrpcSchema {
    pub request: PrivateWithdrawRequestSchema,
    pub response: PrivateWithdrawResponseSchema,
}
impl From<&PrivateWithdrawJsonrpcSchema> for PrivateWithdrawJsonrpcSchema {
    fn from(value: &PrivateWithdrawJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateWithdrawParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset_name",
    "nonce",
    "signature",
    "signature_expiry_sec",
    "signer",
    "subaccount_id"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Amount of the asset to withdraw",
      "type": "string",
      "format": "decimal"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Name of asset to withdraw",
      "type": "string"
    },
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "signature": {
      "title": "signature",
      "description": "Ethereum signature of the withdraw",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds. Expiry MUST be >5min from now",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Ethereum wallet address that is signing the withdraw",
      "type": "string"
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

pub struct PrivateWithdrawParamsSchema {
    ///Amount of the asset to withdraw
    pub amount: bigdecimal::BigDecimal,
    ///Name of asset to withdraw
    pub asset_name: String,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Ethereum signature of the withdraw
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be >5min from now
    pub signature_expiry_sec: i64,
    ///Ethereum wallet address that is signing the withdraw
    pub signer: String,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateWithdrawParamsSchema> for PrivateWithdrawParamsSchema {
    fn from(value: &PrivateWithdrawParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateWithdrawRequestSchema
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
      "const": "private/withdraw"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateWithdrawParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateWithdrawRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateWithdrawRequestSchemaId>,
    pub method: String,
    pub params: PrivateWithdrawParamsSchema,
}
impl From<&PrivateWithdrawRequestSchema> for PrivateWithdrawRequestSchema {
    fn from(value: &PrivateWithdrawRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateWithdrawRequestSchemaId
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
pub enum PrivateWithdrawRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateWithdrawRequestSchemaId> for PrivateWithdrawRequestSchemaId {
    fn from(value: &PrivateWithdrawRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateWithdrawRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateWithdrawRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateWithdrawRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateWithdrawRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateWithdrawRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateWithdrawRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateWithdrawResponseSchema
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
      "$ref": "#/definitions/PrivateWithdrawResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateWithdrawResponseSchema {
    pub id: PrivateWithdrawResponseSchemaId,
    ///
    pub result: PrivateWithdrawResultSchema,
}
impl From<&PrivateWithdrawResponseSchema> for PrivateWithdrawResponseSchema {
    fn from(value: &PrivateWithdrawResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateWithdrawResponseSchemaId
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
pub enum PrivateWithdrawResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateWithdrawResponseSchemaId> for PrivateWithdrawResponseSchemaId {
    fn from(value: &PrivateWithdrawResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateWithdrawResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateWithdrawResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateWithdrawResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateWithdrawResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateWithdrawResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateWithdrawResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateWithdrawResultSchema
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
      "description": "Transaction id of the withdrawal",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateWithdrawResultSchema {
    ///`requested`
    pub status: String,
    ///Transaction id of the withdrawal
    pub transaction_id: uuid::Uuid,
}
impl From<&PrivateWithdrawResultSchema> for PrivateWithdrawResultSchema {
    fn from(value: &PrivateWithdrawResultSchema) -> Self {
        value.clone()
    }
}
