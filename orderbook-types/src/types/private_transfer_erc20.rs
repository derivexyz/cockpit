#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
/**Transfer ERC20 assets from one subaccount to another (e.g. USDC or ETH).

For transfering positions (e.g. options or perps), use `private/transfer_position` instead.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/transfer_erc20",
  "description": "Transfer ERC20 assets from one subaccount to another (e.g. USDC or ETH).\n\nFor transfering positions (e.g. options or perps), use `private/transfer_position` instead.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateTransferErc20JSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateTransferErc20(pub PrivateTransferErc20JsonrpcSchema);
impl std::ops::Deref for PrivateTransferErc20 {
    type Target = PrivateTransferErc20JsonrpcSchema;
    fn deref(&self) -> &PrivateTransferErc20JsonrpcSchema {
        &self.0
    }
}
impl From<PrivateTransferErc20> for PrivateTransferErc20JsonrpcSchema {
    fn from(value: PrivateTransferErc20) -> Self {
        value.0
    }
}
impl From<&PrivateTransferErc20> for PrivateTransferErc20 {
    fn from(value: &PrivateTransferErc20) -> Self {
        value.clone()
    }
}
impl From<PrivateTransferErc20JsonrpcSchema> for PrivateTransferErc20 {
    fn from(value: PrivateTransferErc20JsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateTransferErc20JsonrpcSchema
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
      "$ref": "#/definitions/PrivateTransferErc20RequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateTransferErc20ResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferErc20JsonrpcSchema {
    pub request: PrivateTransferErc20RequestSchema,
    pub response: PrivateTransferErc20ResponseSchema,
}
impl From<&PrivateTransferErc20JsonrpcSchema> for PrivateTransferErc20JsonrpcSchema {
    fn from(value: &PrivateTransferErc20JsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferErc20ParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "recipient_details",
    "recipient_subaccount_id",
    "sender_details",
    "subaccount_id",
    "transfer"
  ],
  "properties": {
    "recipient_details": {
      "description": "Details of the recipient",
      "type": "object",
      "$ref": "#/definitions/SignatureDetailsSchema",
      "field_many": false
    },
    "recipient_subaccount_id": {
      "title": "recipient_subaccount_id",
      "description": "Subaccount_id of the recipient",
      "type": "integer"
    },
    "sender_details": {
      "description": "Details of the sender",
      "type": "object",
      "$ref": "#/definitions/SignatureDetailsSchema",
      "field_many": false
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    },
    "transfer": {
      "description": "Transfer details",
      "type": "object",
      "$ref": "#/definitions/TransferDetailsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferErc20ParamsSchema {
    ///Details of the recipient
    pub recipient_details: SignatureDetailsSchema,
    ///Subaccount_id of the recipient
    pub recipient_subaccount_id: i64,
    ///Details of the sender
    pub sender_details: SignatureDetailsSchema,
    ///Subaccount_id
    pub subaccount_id: i64,
    ///Transfer details
    pub transfer: TransferDetailsSchema,
}
impl From<&PrivateTransferErc20ParamsSchema> for PrivateTransferErc20ParamsSchema {
    fn from(value: &PrivateTransferErc20ParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferErc20RequestSchema
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
      "const": "private/transfer_erc20"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateTransferErc20ParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferErc20RequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateTransferErc20RequestSchemaId>,
    pub method: String,
    pub params: PrivateTransferErc20ParamsSchema,
}
impl From<&PrivateTransferErc20RequestSchema> for PrivateTransferErc20RequestSchema {
    fn from(value: &PrivateTransferErc20RequestSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferErc20RequestSchemaId
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
pub enum PrivateTransferErc20RequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateTransferErc20RequestSchemaId> for PrivateTransferErc20RequestSchemaId {
    fn from(value: &PrivateTransferErc20RequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateTransferErc20RequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateTransferErc20RequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateTransferErc20RequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateTransferErc20RequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateTransferErc20RequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateTransferErc20RequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateTransferErc20ResponseSchema
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
      "$ref": "#/definitions/PrivateTransferErc20ResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferErc20ResponseSchema {
    pub id: PrivateTransferErc20ResponseSchemaId,
    ///
    pub result: PrivateTransferErc20ResultSchema,
}
impl From<&PrivateTransferErc20ResponseSchema> for PrivateTransferErc20ResponseSchema {
    fn from(value: &PrivateTransferErc20ResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateTransferErc20ResponseSchemaId
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
pub enum PrivateTransferErc20ResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateTransferErc20ResponseSchemaId>
for PrivateTransferErc20ResponseSchemaId {
    fn from(value: &PrivateTransferErc20ResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateTransferErc20ResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateTransferErc20ResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateTransferErc20ResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateTransferErc20ResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateTransferErc20ResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateTransferErc20ResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateTransferErc20ResultSchema
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
      "description": "Transaction id of the transfer",
      "type": "string",
      "format": "uuid"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateTransferErc20ResultSchema {
    ///`requested`
    pub status: String,
    ///Transaction id of the transfer
    pub transaction_id: uuid::Uuid,
}
impl From<&PrivateTransferErc20ResultSchema> for PrivateTransferErc20ResultSchema {
    fn from(value: &PrivateTransferErc20ResultSchema) -> Self {
        value.clone()
    }
}
///SignatureDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "nonce",
    "signature",
    "signature_expiry_sec",
    "signer"
  ],
  "properties": {
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "signature": {
      "title": "signature",
      "description": "Ethereum signature of the transfer",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds. Expiry MUST be >5min from now",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Ethereum wallet address that is signing the transfer",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SignatureDetailsSchema {
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Ethereum signature of the transfer
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be >5min from now
    pub signature_expiry_sec: i64,
    ///Ethereum wallet address that is signing the transfer
    pub signer: String,
}
impl From<&SignatureDetailsSchema> for SignatureDetailsSchema {
    fn from(value: &SignatureDetailsSchema) -> Self {
        value.clone()
    }
}
///TransferDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "address",
    "amount",
    "sub_id"
  ],
  "properties": {
    "address": {
      "title": "address",
      "description": "Ethereum address of the asset being transferred",
      "type": "string"
    },
    "amount": {
      "title": "amount",
      "description": "Amount to transfer",
      "type": "string",
      "format": "decimal"
    },
    "sub_id": {
      "title": "sub_id",
      "description": "Sub ID of the asset being transferred",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TransferDetailsSchema {
    ///Ethereum address of the asset being transferred
    pub address: String,
    ///Amount to transfer
    pub amount: bigdecimal::BigDecimal,
    ///Sub ID of the asset being transferred
    pub sub_id: i64,
}
impl From<&TransferDetailsSchema> for TransferDetailsSchema {
    fn from(value: &TransferDetailsSchema) -> Self {
        value.clone()
    }
}
