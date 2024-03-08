#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///AssetResponsePublicSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "address",
    "asset_id",
    "asset_name",
    "asset_type",
    "currency",
    "erc20_details",
    "is_collateral",
    "is_position",
    "option_details",
    "perp_details",
    "sub_id"
  ],
  "properties": {
    "address": {
      "title": "address",
      "description": "On-chain address of Asset.sol contract",
      "type": "string"
    },
    "asset_id": {
      "title": "asset_id",
      "description": "Asset ID of the created asset",
      "type": "string",
      "format": "uuid"
    },
    "asset_name": {
      "title": "asset_name",
      "description": "Asset name",
      "type": "string"
    },
    "asset_type": {
      "title": "asset_type",
      "description": "`erc20`, `option`, or `perp`",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "erc20_details": {
      "description": "ERC20-specific details",
      "oneOf": [
        {
          "description": "ERC20-specific details",
          "default": {
            "borrow_index": "1",
            "supply_index": "1"
          },
          "type": "object",
          "$ref": "#/definitions/ERC20PublicDetailsSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "is_collateral": {
      "title": "is_collateral",
      "description": "If `True`: use as collateral in margin calculations",
      "type": "boolean"
    },
    "is_position": {
      "title": "is_position",
      "description": "If `True`: treat as position in margin calculations",
      "type": "boolean"
    },
    "option_details": {
      "description": "Option-specific details",
      "oneOf": [
        {
          "description": "Option-specific details",
          "default": {
            "settlement_price": null
          },
          "type": "object",
          "$ref": "#/definitions/OptionPublicDetailsSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "perp_details": {
      "description": "Perp-specific details",
      "oneOf": [
        {
          "description": "Perp-specific details",
          "default": {},
          "type": "object",
          "$ref": "#/definitions/PerpPublicDetailsSchema",
          "field_many": false
        },
        {
          "title": "",
          "type": "null"
        }
      ]
    },
    "sub_id": {
      "title": "sub_id",
      "description": "SubId identifier used in Asset.sol",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AssetResponsePublicSchema {
    ///On-chain address of Asset.sol contract
    pub address: String,
    ///Asset ID of the created asset
    pub asset_id: uuid::Uuid,
    ///Asset name
    pub asset_name: String,
    ///`erc20`, `option`, or `perp`
    pub asset_type: AssetType,
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///ERC20-specific details
    pub erc20_details: Option<Erc20PublicDetailsSchema>,
    ///If `True`: use as collateral in margin calculations
    pub is_collateral: bool,
    ///If `True`: treat as position in margin calculations
    pub is_position: bool,
    ///Option-specific details
    pub option_details: Option<OptionPublicDetailsSchema>,
    ///Perp-specific details
    pub perp_details: Option<PerpPublicDetailsSchema>,
    ///SubId identifier used in Asset.sol
    pub sub_id: String,
}
impl From<&AssetResponsePublicSchema> for AssetResponsePublicSchema {
    fn from(value: &AssetResponsePublicSchema) -> Self {
        value.clone()
    }
}
///`erc20`, `option`, or `perp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "asset_type",
  "description": "`erc20`, `option`, or `perp`",
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
///Erc20PublicDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "decimals"
  ],
  "properties": {
    "borrow_index": {
      "title": "borrow_index",
      "description": "Latest borrow index as per `CashAsset.sol` implementation",
      "default": "1",
      "type": "string",
      "format": "decimal"
    },
    "decimals": {
      "title": "decimals",
      "description": "Number of decimals of the underlying on-chain ERC20 token",
      "type": "integer"
    },
    "supply_index": {
      "title": "supply_index",
      "description": "Latest supply index as per `CashAsset.sol` implementation",
      "default": "1",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct Erc20PublicDetailsSchema {
    ///Latest borrow index as per `CashAsset.sol` implementation
    #[serde(default = "defaults::erc20_public_details_schema_borrow_index")]
    pub borrow_index: bigdecimal::BigDecimal,
    ///Number of decimals of the underlying on-chain ERC20 token
    pub decimals: i64,
    ///Latest supply index as per `CashAsset.sol` implementation
    #[serde(default = "defaults::erc20_public_details_schema_supply_index")]
    pub supply_index: bigdecimal::BigDecimal,
}
impl From<&Erc20PublicDetailsSchema> for Erc20PublicDetailsSchema {
    fn from(value: &Erc20PublicDetailsSchema) -> Self {
        value.clone()
    }
}
///OptionPublicDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "expiry",
    "index",
    "option_type",
    "strike"
  ],
  "properties": {
    "expiry": {
      "title": "expiry",
      "description": "Unix timestamp of expiry date (in seconds)",
      "type": "integer"
    },
    "index": {
      "title": "index",
      "description": "Underlying settlement price index",
      "type": "string"
    },
    "option_type": {
      "title": "option_type",
      "type": "string",
      "enum": [
        "C",
        "P"
      ]
    },
    "settlement_price": {
      "title": "settlement_price",
      "description": "Settlement price of the option",
      "type": [
        "string",
        "null"
      ],
      "format": "decimal"
    },
    "strike": {
      "title": "strike",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct OptionPublicDetailsSchema {
    ///Unix timestamp of expiry date (in seconds)
    pub expiry: i64,
    ///Underlying settlement price index
    pub index: String,
    pub option_type: OptionType,
    ///Settlement price of the option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<bigdecimal::BigDecimal>,
    pub strike: bigdecimal::BigDecimal,
}
impl From<&OptionPublicDetailsSchema> for OptionPublicDetailsSchema {
    fn from(value: &OptionPublicDetailsSchema) -> Self {
        value.clone()
    }
}
///OptionType
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "option_type",
  "type": "string",
  "enum": [
    "C",
    "P"
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
pub enum OptionType {
    C,
    P,
}
impl From<&OptionType> for OptionType {
    fn from(value: &OptionType) -> Self {
        value.clone()
    }
}
impl ToString for OptionType {
    fn to_string(&self) -> String {
        match *self {
            Self::C => "C".to_string(),
            Self::P => "P".to_string(),
        }
    }
}
impl std::str::FromStr for OptionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "C" => Ok(Self::C),
            "P" => Ok(Self::P),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OptionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OptionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OptionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///PerpPublicDetailsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "aggregate_funding",
    "funding_rate",
    "index",
    "max_rate_per_hour",
    "min_rate_per_hour",
    "static_interest_rate"
  ],
  "properties": {
    "aggregate_funding": {
      "title": "aggregate_funding",
      "description": "Latest aggregated funding as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "funding_rate": {
      "title": "funding_rate",
      "description": "Current hourly funding rate as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "index": {
      "title": "index",
      "description": "Underlying spot price index for funding rate",
      "type": "string"
    },
    "max_rate_per_hour": {
      "title": "max_rate_per_hour",
      "description": "Max rate per hour as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "min_rate_per_hour": {
      "title": "min_rate_per_hour",
      "description": "Min rate per hour as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    },
    "static_interest_rate": {
      "title": "static_interest_rate",
      "description": "Static interest rate as per `PerpAsset.sol`",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PerpPublicDetailsSchema {
    ///Latest aggregated funding as per `PerpAsset.sol`
    pub aggregate_funding: bigdecimal::BigDecimal,
    ///Current hourly funding rate as per `PerpAsset.sol`
    pub funding_rate: bigdecimal::BigDecimal,
    ///Underlying spot price index for funding rate
    pub index: String,
    ///Max rate per hour as per `PerpAsset.sol`
    pub max_rate_per_hour: bigdecimal::BigDecimal,
    ///Min rate per hour as per `PerpAsset.sol`
    pub min_rate_per_hour: bigdecimal::BigDecimal,
    ///Static interest rate as per `PerpAsset.sol`
    pub static_interest_rate: bigdecimal::BigDecimal,
}
impl From<&PerpPublicDetailsSchema> for PerpPublicDetailsSchema {
    fn from(value: &PerpPublicDetailsSchema) -> Self {
        value.clone()
    }
}
///Get all assets for a given `currency` and `type`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "public/get_assets",
  "description": "Get all assets for a given `currency` and `type`",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetAssetsJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetAssets(pub PublicGetAssetsJsonrpcSchema);
impl std::ops::Deref for PublicGetAssets {
    type Target = PublicGetAssetsJsonrpcSchema;
    fn deref(&self) -> &PublicGetAssetsJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetAssets> for PublicGetAssetsJsonrpcSchema {
    fn from(value: PublicGetAssets) -> Self {
        value.0
    }
}
impl From<&PublicGetAssets> for PublicGetAssets {
    fn from(value: &PublicGetAssets) -> Self {
        value.clone()
    }
}
impl From<PublicGetAssetsJsonrpcSchema> for PublicGetAssets {
    fn from(value: PublicGetAssetsJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetAssetsJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetAssetsRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetAssetsResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAssetsJsonrpcSchema {
    pub request: PublicGetAssetsRequestSchema,
    pub response: PublicGetAssetsResponseSchema,
}
impl From<&PublicGetAssetsJsonrpcSchema> for PublicGetAssetsJsonrpcSchema {
    fn from(value: &PublicGetAssetsJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetAssetsParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "asset_type",
    "currency",
    "expired"
  ],
  "properties": {
    "asset_type": {
      "title": "asset_type",
      "description": "Asset type",
      "type": "string",
      "enum": [
        "erc20",
        "option",
        "perp"
      ]
    },
    "currency": {
      "title": "currency",
      "description": "Underlying currency of asset (`ETH`, `BTC`, etc)",
      "type": "string"
    },
    "expired": {
      "title": "expired",
      "description": "If `True`: include expired assets",
      "type": "boolean"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAssetsParamsSchema {
    ///Asset type
    pub asset_type: AssetType,
    ///Underlying currency of asset (`ETH`, `BTC`, etc)
    pub currency: String,
    ///If `True`: include expired assets
    pub expired: bool,
}
impl From<&PublicGetAssetsParamsSchema> for PublicGetAssetsParamsSchema {
    fn from(value: &PublicGetAssetsParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetAssetsRequestSchema
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
      "const": "public/get_assets"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetAssetsParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAssetsRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetAssetsRequestSchemaId>,
    pub method: String,
    pub params: PublicGetAssetsParamsSchema,
}
impl From<&PublicGetAssetsRequestSchema> for PublicGetAssetsRequestSchema {
    fn from(value: &PublicGetAssetsRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetAssetsRequestSchemaId
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
pub enum PublicGetAssetsRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetAssetsRequestSchemaId> for PublicGetAssetsRequestSchemaId {
    fn from(value: &PublicGetAssetsRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetAssetsRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetAssetsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetAssetsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetAssetsRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetAssetsRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetAssetsRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetAssetsResponseSchema
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
      "title": "result",
      "description": "",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/AssetResponsePublicSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetAssetsResponseSchema {
    pub id: PublicGetAssetsResponseSchemaId,
    ///
    pub result: Vec<AssetResponsePublicSchema>,
}
impl From<&PublicGetAssetsResponseSchema> for PublicGetAssetsResponseSchema {
    fn from(value: &PublicGetAssetsResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetAssetsResponseSchemaId
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
pub enum PublicGetAssetsResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetAssetsResponseSchemaId> for PublicGetAssetsResponseSchemaId {
    fn from(value: &PublicGetAssetsResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetAssetsResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetAssetsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetAssetsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetAssetsResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetAssetsResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetAssetsResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
pub mod defaults {
    pub(super) fn erc20_public_details_schema_borrow_index() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"1\"").unwrap()
    }
    pub(super) fn erc20_public_details_schema_supply_index() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"1\"").unwrap()
    }
}
