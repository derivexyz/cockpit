#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Deposit an asset to a subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/deposit",
  "description": "Deposit an asset to a subaccount.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateDepositJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateDeposit(pub PrivateDepositJsonrpcSchema);
impl std::ops::Deref for PrivateDeposit {
    type Target = PrivateDepositJsonrpcSchema;
    fn deref(&self) -> &PrivateDepositJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateDeposit> for PrivateDepositJsonrpcSchema {
    fn from(value: PrivateDeposit) -> Self {
        value.0
    }
}
impl From<&PrivateDeposit> for PrivateDeposit {
    fn from(value: &PrivateDeposit) -> Self {
        value.clone()
    }
}
impl From<PrivateDepositJsonrpcSchema> for PrivateDeposit {
    fn from(value: PrivateDepositJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateDepositJsonrpcSchema
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
      "$ref": "#/definitions/PrivateDepositRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateDepositResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateDepositJsonrpcSchema {
    pub request: PrivateDepositRequestSchema,
    pub response: PrivateDepositResponseSchema,
}
impl From<&PrivateDepositJsonrpcSchema> for PrivateDepositJsonrpcSchema {
    fn from(value: &PrivateDepositJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateDepositParamsSchema
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
      "description": "Amount of the asset to deposit",
      "type": "string",
      "format": "decimal"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Name of asset to deposit",
      "type": "string"
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

pub struct PrivateDepositParamsSchema {
    ///Amount of the asset to deposit
    pub amount: bigdecimal::BigDecimal,
    ///Name of asset to deposit
    pub asset_name: String,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Ethereum signature of the deposit
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be >5min from now
    pub signature_expiry_sec: i64,
    ///Ethereum wallet address that is signing the deposit
    pub signer: String,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateDepositParamsSchema> for PrivateDepositParamsSchema {
    fn from(value: &PrivateDepositParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateDepositRequestSchema
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
      "const": "private/deposit"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateDepositParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateDepositRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateDepositRequestSchemaId>,
    pub method: String,
    pub params: PrivateDepositParamsSchema,
}
impl From<&PrivateDepositRequestSchema> for PrivateDepositRequestSchema {
    fn from(value: &PrivateDepositRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateDepositRequestSchemaId
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
pub enum PrivateDepositRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateDepositRequestSchemaId> for PrivateDepositRequestSchemaId {
    fn from(value: &PrivateDepositRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateDepositRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateDepositRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateDepositRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateDepositRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateDepositRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateDepositRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateDepositResponseSchema
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
      "$ref": "#/definitions/PrivateDepositResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateDepositResponseSchema {
    pub id: PrivateDepositResponseSchemaId,
    ///
    pub result: PrivateDepositResultSchema,
}
impl From<&PrivateDepositResponseSchema> for PrivateDepositResponseSchema {
    fn from(value: &PrivateDepositResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateDepositResponseSchemaId
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
pub enum PrivateDepositResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateDepositResponseSchemaId> for PrivateDepositResponseSchemaId {
    fn from(value: &PrivateDepositResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateDepositResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateDepositResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateDepositResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateDepositResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateDepositResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateDepositResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateDepositResultSchema
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
      "description": "Transaction id of the deposit",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateDepositResultSchema {
    ///`requested`
    pub status: String,
    ///Transaction id of the deposit
    pub transaction_id: uuid::Uuid,
}
impl From<&PrivateDepositResultSchema> for PrivateDepositResultSchema {
    fn from(value: &PrivateDepositResultSchema) -> Self {
        value.clone()
    }
}
