#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Type of asset collateral (currently always `erc20`)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "asset_type",
  "description": "Type of asset collateral (currently always `erc20`)",
  "type": "string",
  "enum": [
    "erc20",
    "option",
    "perp"
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
pub enum AssetType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}
impl From<&AssetType> for AssetType {
    fn from(value: &AssetType) -> Self {
        value.clone()
    }
}
impl ToString for AssetType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}
impl std::str::FromStr for AssetType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "erc20" => Ok(Self::Erc20),
            "option" => Ok(Self::Option),
            "perp" => Ok(Self::Perp),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AssetType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AssetType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AssetType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///CollateralResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "asset_name",
    "asset_type",
    "cumulative_interest",
    "currency",
    "initial_margin",
    "maintenance_margin",
    "mark_price",
    "mark_value",
    "pending_interest"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Asset amount of given collateral",
      "type": "string",
      "format": "decimal"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Asset name",
      "type": "string"
    },
    "asset_type": {
      "title": "asset_type",
      "description": "Type of asset collateral (currently always `erc20`)",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "cumulative_interest": {
      "title": "cumulative_interest",
      "description": "Cumulative interest earned on supplying collateral or paid for borrowing",
      "type": "string",
      "format": "decimal"
    },
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "initial_margin": {
      "title": "initial_margin",
      "description": "USD value of collateral that contributes to initial margin",
      "type": "string",
      "format": "decimal"
    },
    "maintenance_margin": {
      "title": "maintenance_margin",
      "description": "USD value of collateral that contributes to maintenance margin",
      "type": "string",
      "format": "decimal"
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Current mark price of the asset",
      "type": "string",
      "format": "decimal"
    },
    "mark_value": {
      "title": "mark_value",
      "description": "USD value of the collateral (amount * mark price)",
      "type": "string",
      "format": "decimal"
    },
    "pending_interest": {
      "title": "pending_interest",
      "description": "Portion of interest that has not yet been settled on-chain. This number is added to the portfolio value for margin calculations purposes.",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct CollateralResponseSchema {
    ///Asset amount of given collateral
    pub amount: bigdecimal::BigDecimal,
    ///Asset name
    pub asset_name: String,
    ///Type of asset collateral (currently always `erc20`)
    pub asset_type: AssetType,
    ///Cumulative interest earned on supplying collateral or paid for borrowing
    pub cumulative_interest: bigdecimal::BigDecimal,
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///USD value of collateral that contributes to initial margin
    pub initial_margin: bigdecimal::BigDecimal,
    ///USD value of collateral that contributes to maintenance margin
    pub maintenance_margin: bigdecimal::BigDecimal,
    ///Current mark price of the asset
    pub mark_price: bigdecimal::BigDecimal,
    ///USD value of the collateral (amount * mark price)
    pub mark_value: bigdecimal::BigDecimal,
    ///Portion of interest that has not yet been settled on-chain. This number is added to the portfolio value for margin calculations purposes.
    pub pending_interest: bigdecimal::BigDecimal,
}
impl From<&CollateralResponseSchema> for CollateralResponseSchema {
    fn from(value: &CollateralResponseSchema) -> Self {
        value.clone()
    }
}
///Get collaterals of a subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_collaterals",
  "description": "Get collaterals of a subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetCollateralsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetCollaterals(pub PrivateGetCollateralsJsonrpcSchema);
impl std::ops::Deref for PrivateGetCollaterals {
    type Target = PrivateGetCollateralsJsonrpcSchema;
    fn deref(&self) -> &PrivateGetCollateralsJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetCollaterals> for PrivateGetCollateralsJsonrpcSchema {
    fn from(value: PrivateGetCollaterals) -> Self {
        value.0
    }
}
impl From<&PrivateGetCollaterals> for PrivateGetCollaterals {
    fn from(value: &PrivateGetCollaterals) -> Self {
        value.clone()
    }
}
impl From<PrivateGetCollateralsJsonrpcSchema> for PrivateGetCollaterals {
    fn from(value: PrivateGetCollateralsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetCollateralsJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetCollateralsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetCollateralsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetCollateralsJsonrpcSchema {
    pub request: PrivateGetCollateralsRequestSchema,
    pub response: PrivateGetCollateralsResponseSchema,
}
impl From<&PrivateGetCollateralsJsonrpcSchema> for PrivateGetCollateralsJsonrpcSchema {
    fn from(value: &PrivateGetCollateralsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetCollateralsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id"
  ],
  "properties": {
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

pub struct PrivateGetCollateralsParamsSchema {
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetCollateralsParamsSchema> for PrivateGetCollateralsParamsSchema {
    fn from(value: &PrivateGetCollateralsParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetCollateralsRequestSchema
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
      "const": "private/get_collaterals"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetCollateralsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetCollateralsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetCollateralsRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetCollateralsParamsSchema,
}
impl From<&PrivateGetCollateralsRequestSchema> for PrivateGetCollateralsRequestSchema {
    fn from(value: &PrivateGetCollateralsRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetCollateralsRequestSchemaId
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
pub enum PrivateGetCollateralsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetCollateralsRequestSchemaId>
for PrivateGetCollateralsRequestSchemaId {
    fn from(value: &PrivateGetCollateralsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetCollateralsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetCollateralsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetCollateralsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetCollateralsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetCollateralsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetCollateralsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetCollateralsResponseSchema
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
      "$ref": "#/definitions/PrivateGetCollateralsResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetCollateralsResponseSchema {
    pub id: PrivateGetCollateralsResponseSchemaId,
    ///
    pub result: PrivateGetCollateralsResultSchema,
}
impl From<&PrivateGetCollateralsResponseSchema> for PrivateGetCollateralsResponseSchema {
    fn from(value: &PrivateGetCollateralsResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetCollateralsResponseSchemaId
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
pub enum PrivateGetCollateralsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetCollateralsResponseSchemaId>
for PrivateGetCollateralsResponseSchemaId {
    fn from(value: &PrivateGetCollateralsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetCollateralsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetCollateralsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetCollateralsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetCollateralsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetCollateralsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetCollateralsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetCollateralsResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "collaterals",
    "subaccount_id"
  ],
  "properties": {
    "collaterals": {
      "title": "collaterals",
      "description": "All collaterals that count towards margin of subaccount",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/CollateralResponseSchema",
        "field_many": true
      }
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

pub struct PrivateGetCollateralsResultSchema {
    ///All collaterals that count towards margin of subaccount
    pub collaterals: Vec<CollateralResponseSchema>,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateGetCollateralsResultSchema> for PrivateGetCollateralsResultSchema {
    fn from(value: &PrivateGetCollateralsResultSchema) -> Self {
        value.clone()
    }
}
