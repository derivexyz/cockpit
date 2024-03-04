#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Order direction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "direction",
  "description": "Order direction",
  "type": "string",
  "enum": [
    "buy",
    "sell"
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
pub enum Direction {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl From<&Direction> for Direction {
    fn from(value: &Direction) -> Self {
        value.clone()
    }
}
impl ToString for Direction {
    fn to_string(&self) -> String {
        match *self {
            Self::Buy => "buy".to_string(),
            Self::Sell => "sell".to_string(),
        }
    }
}
impl std::str::FromStr for Direction {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Direction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Direction {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Direction {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///Role of the user in the trade
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "liquidity_role",
  "description": "Role of the user in the trade",
  "type": "string",
  "enum": [
    "maker",
    "taker"
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
pub enum LiquidityRole {
    #[serde(rename = "maker")]
    Maker,
    #[serde(rename = "taker")]
    Taker,
}
impl From<&LiquidityRole> for LiquidityRole {
    fn from(value: &LiquidityRole) -> Self {
        value.clone()
    }
}
impl ToString for LiquidityRole {
    fn to_string(&self) -> String {
        match *self {
            Self::Maker => "maker".to_string(),
            Self::Taker => "taker".to_string(),
        }
    }
}
impl std::str::FromStr for LiquidityRole {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "maker" => Ok(Self::Maker),
            "taker" => Ok(Self::Taker),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for LiquidityRole {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
///PaginationInfoSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "count",
    "num_pages"
  ],
  "properties": {
    "count": {
      "title": "count",
      "description": "Total number of items, across all pages",
      "type": "integer"
    },
    "num_pages": {
      "title": "num_pages",
      "description": "Number of pages",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PaginationInfoSchema {
    ///Total number of items, across all pages
    pub count: i64,
    ///Number of pages
    pub num_pages: i64,
}
impl From<&PaginationInfoSchema> for PaginationInfoSchema {
    fn from(value: &PaginationInfoSchema) -> Self {
        value.clone()
    }
}
///Get trade history for a subaccount, with filter parameters.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_trade_history",
  "description": "Get trade history for a subaccount, with filter parameters.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetTradeHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetTradeHistory(pub PrivateGetTradeHistoryJsonrpcSchema);
impl std::ops::Deref for PrivateGetTradeHistory {
    type Target = PrivateGetTradeHistoryJsonrpcSchema;
    fn deref(&self) -> &PrivateGetTradeHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetTradeHistory> for PrivateGetTradeHistoryJsonrpcSchema {
    fn from(value: PrivateGetTradeHistory) -> Self {
        value.0
    }
}
impl From<&PrivateGetTradeHistory> for PrivateGetTradeHistory {
    fn from(value: &PrivateGetTradeHistory) -> Self {
        value.clone()
    }
}
impl From<PrivateGetTradeHistoryJsonrpcSchema> for PrivateGetTradeHistory {
    fn from(value: PrivateGetTradeHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetTradeHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetTradeHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetTradeHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetTradeHistoryJsonrpcSchema {
    pub request: PrivateGetTradeHistoryRequestSchema,
    pub response: PrivateGetTradeHistoryResponseSchema,
}
impl From<&PrivateGetTradeHistoryJsonrpcSchema> for PrivateGetTradeHistoryJsonrpcSchema {
    fn from(value: &PrivateGetTradeHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetTradeHistoryParamsSchema
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
    "from_timestamp": {
      "title": "from_timestamp",
      "description": "Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.",
      "default": 0,
      "type": "integer"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name to filter by",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "order_id": {
      "title": "order_id",
      "description": "Order id to filter by",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "page": {
      "title": "page",
      "description": "Page number of results to return (default 1, returns last if above `num_pages`)",
      "default": 1,
      "type": "integer"
    },
    "page_size": {
      "title": "page_size",
      "description": "Number of results per page (default 100, max 1000)",
      "default": 100,
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to get trades",
      "type": "integer"
    },
    "to_timestamp": {
      "title": "to_timestamp",
      "description": "Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.",
      "default": 18446744073709551615,
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetTradeHistoryParamsSchema {
    ///Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.
    #[serde(default)]
    pub from_timestamp: i64,
    ///Instrument name to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    ///Order id to filter by
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    ///Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page (default 100, max 1000)
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
    ///Subaccount_id for which to get trades
    pub subaccount_id: i64,
    ///Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.
    #[serde(default = "defaults::default_u64::<i64, 18446744073709551615>")]
    pub to_timestamp: i64,
}
impl From<&PrivateGetTradeHistoryParamsSchema> for PrivateGetTradeHistoryParamsSchema {
    fn from(value: &PrivateGetTradeHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetTradeHistoryRequestSchema
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
      "const": "private/get_trade_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetTradeHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetTradeHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetTradeHistoryRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetTradeHistoryParamsSchema,
}
impl From<&PrivateGetTradeHistoryRequestSchema> for PrivateGetTradeHistoryRequestSchema {
    fn from(value: &PrivateGetTradeHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetTradeHistoryRequestSchemaId
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
pub enum PrivateGetTradeHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetTradeHistoryRequestSchemaId>
for PrivateGetTradeHistoryRequestSchemaId {
    fn from(value: &PrivateGetTradeHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetTradeHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetTradeHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetTradeHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetTradeHistoryResponseSchema
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
      "$ref": "#/definitions/PrivateGetTradeHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetTradeHistoryResponseSchema {
    pub id: PrivateGetTradeHistoryResponseSchemaId,
    ///
    pub result: PrivateGetTradeHistoryResultSchema,
}
impl From<&PrivateGetTradeHistoryResponseSchema>
for PrivateGetTradeHistoryResponseSchema {
    fn from(value: &PrivateGetTradeHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetTradeHistoryResponseSchemaId
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
pub enum PrivateGetTradeHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetTradeHistoryResponseSchemaId>
for PrivateGetTradeHistoryResponseSchemaId {
    fn from(value: &PrivateGetTradeHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetTradeHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetTradeHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetTradeHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetTradeHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "pagination",
    "subaccount_id",
    "trades"
  ],
  "properties": {
    "pagination": {
      "description": "Pagination info",
      "type": "object",
      "$ref": "#/definitions/PaginationInfoSchema",
      "field_many": false
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to get the trade history",
      "type": "integer"
    },
    "trades": {
      "title": "trades",
      "description": "List of trades",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/TradeResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetTradeHistoryResultSchema {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///Subaccount_id for which to get the trade history
    pub subaccount_id: i64,
    ///List of trades
    pub trades: Vec<TradeResponseSchema>,
}
impl From<&PrivateGetTradeHistoryResultSchema> for PrivateGetTradeHistoryResultSchema {
    fn from(value: &PrivateGetTradeHistoryResultSchema) -> Self {
        value.clone()
    }
}
///TradeResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "direction",
    "index_price",
    "instrument_name",
    "is_transfer",
    "label",
    "liquidity_role",
    "mark_price",
    "order_id",
    "quote_id",
    "realized_pnl",
    "subaccount_id",
    "timestamp",
    "trade_amount",
    "trade_fee",
    "trade_id",
    "trade_price",
    "tx_hash",
    "tx_status"
  ],
  "properties": {
    "direction": {
      "title": "direction",
      "description": "Order direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "index_price": {
      "title": "index_price",
      "description": "Index price of the underlying at the time of the trade",
      "type": "string",
      "format": "decimal"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "is_transfer": {
      "title": "is_transfer",
      "description": "Whether the trade was generated through `private/transfer_position`",
      "type": "boolean"
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "type": "string"
    },
    "liquidity_role": {
      "title": "liquidity_role",
      "description": "Role of the user in the trade",
      "type": "string",
      "enum": [
        "maker",
        "taker"
      ]
    },
    "mark_price": {
      "title": "mark_price",
      "description": "Mark price of the instrument at the time of the trade",
      "type": "string",
      "format": "decimal"
    },
    "order_id": {
      "title": "order_id",
      "description": "Order ID",
      "type": "string"
    },
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID if the trade was executed via RFQ",
      "default": null,
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "realized_pnl": {
      "title": "realized_pnl",
      "description": "Realized PnL for this trade",
      "type": "string",
      "format": "decimal"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "timestamp": {
      "title": "timestamp",
      "description": "Trade timestamp (in ms since Unix epoch)",
      "type": "integer"
    },
    "trade_amount": {
      "title": "trade_amount",
      "description": "Amount filled in this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_fee": {
      "title": "trade_fee",
      "description": "Fee for this trade",
      "type": "string",
      "format": "decimal"
    },
    "trade_id": {
      "title": "trade_id",
      "description": "Trade ID",
      "type": "string"
    },
    "trade_price": {
      "title": "trade_price",
      "description": "Price at which the trade was filled",
      "type": "string",
      "format": "decimal"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "Blockchain transaction hash",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status",
      "type": "string",
      "enum": [
        "requested",
        "pending",
        "settled",
        "reverted",
        "ignored"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeResponseSchema {
    ///Order direction
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Whether the trade was generated through `private/transfer_position`
    pub is_transfer: bool,
    ///Optional user-defined label for the order
    pub label: String,
    ///Role of the user in the trade
    pub liquidity_role: LiquidityRole,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
    ///Order ID
    pub order_id: String,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///Realized PnL for this trade
    pub realized_pnl: bigdecimal::BigDecimal,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Trade timestamp (in ms since Unix epoch)
    pub timestamp: i64,
    ///Amount filled in this trade
    pub trade_amount: bigdecimal::BigDecimal,
    ///Fee for this trade
    pub trade_fee: bigdecimal::BigDecimal,
    ///Trade ID
    pub trade_id: String,
    ///Price at which the trade was filled
    pub trade_price: bigdecimal::BigDecimal,
    ///Blockchain transaction hash
    pub tx_hash: Option<String>,
    ///Blockchain transaction status
    pub tx_status: TxStatus,
}
impl From<&TradeResponseSchema> for TradeResponseSchema {
    fn from(value: &TradeResponseSchema) -> Self {
        value.clone()
    }
}
///Blockchain transaction status
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Blockchain transaction status",
  "type": "string",
  "enum": [
    "requested",
    "pending",
    "settled",
    "reverted",
    "ignored"
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
pub enum TxStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
    #[serde(rename = "ignored")]
    Ignored,
}
impl From<&TxStatus> for TxStatus {
    fn from(value: &TxStatus) -> Self {
        value.clone()
    }
}
impl ToString for TxStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Requested => "requested".to_string(),
            Self::Pending => "pending".to_string(),
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
            Self::Ignored => "ignored".to_string(),
        }
    }
}
impl std::str::FromStr for TxStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "requested" => Ok(Self::Requested),
            "pending" => Ok(Self::Pending),
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            "ignored" => Ok(Self::Ignored),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TxStatus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
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
