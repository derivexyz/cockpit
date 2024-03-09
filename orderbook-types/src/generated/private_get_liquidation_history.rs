#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///AuctionBidEventSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amounts_liquidated",
    "cash_received",
    "discount_pnl",
    "percent_liquidated",
    "positions_realized_pnl",
    "realized_pnl",
    "timestamp",
    "tx_hash"
  ],
  "properties": {
    "amounts_liquidated": {
      "title": "amounts_liquidated",
      "description": "Amounts of each asset that were closed",
      "type": "object",
      "additionalProperties": {
        "title": "amounts_liquidated",
        "type": "string",
        "format": "decimal"
      }
    },
    "cash_received": {
      "title": "cash_received",
      "description": "Cash received for auctioning off the percentage of the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "discount_pnl": {
      "title": "discount_pnl",
      "description": "Realized PnL due to being liquidated at a discount to mark portfolio value",
      "type": "string",
      "format": "decimal"
    },
    "percent_liquidated": {
      "title": "percent_liquidated",
      "description": "Percent of the subaccount that was liquidated",
      "type": "string",
      "format": "decimal"
    },
    "positions_realized_pnl": {
      "title": "positions_realized_pnl",
      "description": "Realized PnL of each position that was closed",
      "type": "object",
      "additionalProperties": {
        "title": "positions_realized_pnl",
        "type": "string",
        "format": "decimal"
      }
    },
    "realized_pnl": {
      "title": "realized_pnl",
      "description": "Realized PnL of the auction bid, assuming positions are closed at mark price at the time of the liquidation",
      "type": "string",
      "format": "decimal"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Timestamp of the bid (in ms since UNIX epoch)",
      "type": "integer"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Hash of the bid transaction",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AuctionBidEventSchema {
    ///Amounts of each asset that were closed
    pub amounts_liquidated: std::collections::HashMap<String, bigdecimal::BigDecimal>,
    ///Cash received for auctioning off the percentage of the subaccount
    pub cash_received: bigdecimal::BigDecimal,
    ///Realized PnL due to being liquidated at a discount to mark portfolio value
    pub discount_pnl: bigdecimal::BigDecimal,
    ///Percent of the subaccount that was liquidated
    pub percent_liquidated: bigdecimal::BigDecimal,
    ///Realized PnL of each position that was closed
    pub positions_realized_pnl: std::collections::HashMap<
        String,
        bigdecimal::BigDecimal,
    >,
    ///Realized PnL of the auction bid, assuming positions are closed at mark price at the time of the liquidation
    pub realized_pnl: bigdecimal::BigDecimal,
    ///Timestamp of the bid (in ms since UNIX epoch)
    pub timestamp: i64,
    ///Hash of the bid transaction
    pub tx_hash: String,
}
impl From<&AuctionBidEventSchema> for AuctionBidEventSchema {
    fn from(value: &AuctionBidEventSchema) -> Self {
        value.clone()
    }
}
///AuctionResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "auction_id",
    "auction_type",
    "bids",
    "end_timestamp",
    "fee",
    "start_timestamp",
    "tx_hash"
  ],
  "properties": {
    "auction_id": {
      "title": "auction_id",
      "description": "Unique ID of the auction",
      "type": "string"
    },
    "auction_type": {
      "title": "auction_type",
      "description": "Type of auction",
      "type": "string",
      "enum": [
        "solvent",
        "insolvent"
      ]
    },
    "bids": {
      "title": "bids",
      "description": "List of auction bid events",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/AuctionBidEventSchema",
        "field_many": true
      }
    },
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "Timestamp of the auction end (in ms since UNIX epoch), or `null` if not ended",
      "type": [
        "integer",
        "null"
      ]
    },
    "fee": {
      "title": "fee",
      "description": "Fee paid by the subaccount",
      "type": "string",
      "format": "decimal"
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Timestamp of the auction start (in ms since UNIX epoch)",
      "type": "integer"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Hash of the transaction that started the auction",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct AuctionResultSchema {
    ///Unique ID of the auction
    pub auction_id: String,
    ///Type of auction
    pub auction_type: AuctionType,
    ///List of auction bid events
    pub bids: Vec<AuctionBidEventSchema>,
    ///Timestamp of the auction end (in ms since UNIX epoch), or `null` if not ended
    pub end_timestamp: Option<i64>,
    ///Fee paid by the subaccount
    pub fee: bigdecimal::BigDecimal,
    ///Timestamp of the auction start (in ms since UNIX epoch)
    pub start_timestamp: i64,
    ///Hash of the transaction that started the auction
    pub tx_hash: String,
}
impl From<&AuctionResultSchema> for AuctionResultSchema {
    fn from(value: &AuctionResultSchema) -> Self {
        value.clone()
    }
}
///Type of auction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "auction_type",
  "description": "Type of auction",
  "type": "string",
  "enum": [
    "solvent",
    "insolvent"
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
pub enum AuctionType {
    #[serde(rename = "solvent")]
    Solvent,
    #[serde(rename = "insolvent")]
    Insolvent,
}
impl From<&AuctionType> for AuctionType {
    fn from(value: &AuctionType) -> Self {
        value.clone()
    }
}
impl ToString for AuctionType {
    fn to_string(&self) -> String {
        match *self {
            Self::Solvent => "solvent".to_string(),
            Self::Insolvent => "insolvent".to_string(),
        }
    }
}
impl std::str::FromStr for AuctionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "solvent" => Ok(Self::Solvent),
            "insolvent" => Ok(Self::Insolvent),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AuctionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AuctionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AuctionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_liquidation_history",
  "description": "",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetLiquidationHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetLiquidationHistory(pub PrivateGetLiquidationHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetLiquidationHistory {
    type Target = PrivateGetLiquidationHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetLiquidationHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetLiquidationHistory> for PrivateGetLiquidationHistoryJsonrpcSchema {
    fn from(value: PrivateGetLiquidationHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetLiquidationHistory> for PrivateGetLiquidationHistory {
    fn from(value: &PrivateGetLiquidationHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetLiquidationHistoryJsonrpcSchema> for PrivateGetLiquidationHistory {
    fn from(value: PrivateGetLiquidationHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetLiquidationHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetLiquidationHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetLiquidationHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetLiquidationHistoryJsonrpcSchema {
    pub request: PrivateGetLiquidationHistoryRequestSchema,
    pub response: PrivateGetLiquidationHistoryResponseSchema,
}
impl From<&PrivateGetLiquidationHistoryJsonrpcSchema>
for PrivateGetLiquidationHistoryJsonrpcSchema {
    fn from(value: &PrivateGetLiquidationHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetLiquidationHistoryParamsSchema
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
    "end_timestamp": {
      "title": "end_timestamp",
      "description": "End timestamp of the event history (default current time)",
      "default": 9223372036854775807,
      "type": "integer"
    },
    "start_timestamp": {
      "title": "start_timestamp",
      "description": "Start timestamp of the event history (default 0)",
      "default": 0,
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetLiquidationHistoryParamsSchema {
    ///End timestamp of the event history (default current time)
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub end_timestamp: i64,
    ///Start timestamp of the event history (default 0)
    #[serde(default)]
    pub start_timestamp: i64,
    ///Subaccount id
    pub subaccount_id: i64,
}
impl From<&PrivateGetLiquidationHistoryParamsSchema>
for PrivateGetLiquidationHistoryParamsSchema {
    fn from(value: &PrivateGetLiquidationHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetLiquidationHistoryRequestSchema
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
      "const": "private/get_liquidation_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetLiquidationHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetLiquidationHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetLiquidationHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetLiquidationHistoryParamsSchema,
}
impl From<&PrivateGetLiquidationHistoryRequestSchema>
for PrivateGetLiquidationHistoryRequestSchema {
    fn from(value: &PrivateGetLiquidationHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetLiquidationHistoryRequestSchemaId
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
pub enum PrivateGetLiquidationHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetLiquidationHistoryRequestSchemaId>
for PrivateGetLiquidationHistoryRequestSchemaId {
    fn from(value: &PrivateGetLiquidationHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetLiquidationHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetLiquidationHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetLiquidationHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetLiquidationHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetLiquidationHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetLiquidationHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetLiquidationHistoryResponseSchema
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
        "$ref": "#/definitions/AuctionResultSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetLiquidationHistoryResponseSchema {
    pub id: PrivateGetLiquidationHistoryResponseSchemaId,
    ///
    pub result: Vec<AuctionResultSchema>,
}
impl From<&PrivateGetLiquidationHistoryResponseSchema>
for PrivateGetLiquidationHistoryResponseSchema {
    fn from(value: &PrivateGetLiquidationHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetLiquidationHistoryResponseSchemaId
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
pub enum PrivateGetLiquidationHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetLiquidationHistoryResponseSchemaId>
for PrivateGetLiquidationHistoryResponseSchemaId {
    fn from(value: &PrivateGetLiquidationHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetLiquidationHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetLiquidationHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetLiquidationHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetLiquidationHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetLiquidationHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetLiquidationHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
