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
///Instrument type to filter by (defaults to all)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "instrument_type",
  "description": "Instrument type to filter by (defaults to all)",
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
pub enum InstrumentType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}
impl From<&InstrumentType> for InstrumentType {
    fn from(value: &InstrumentType) -> Self {
        value.clone()
    }
}
impl ToString for InstrumentType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}
impl std::str::FromStr for InstrumentType {
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
impl std::convert::TryFrom<&str> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for InstrumentType {
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
  "title": "public/get_trade_history",
  "description": "Get trade history for a subaccount, with filter parameters.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PublicGetTradeHistoryJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicGetTradeHistory(pub PublicGetTradeHistoryJsonrpcSchema);
impl std::ops::Deref for PublicGetTradeHistory {
    type Target = PublicGetTradeHistoryJsonrpcSchema;
    fn deref(&self) -> &PublicGetTradeHistoryJsonrpcSchema {
        &self.0
    }
}
impl From<PublicGetTradeHistory> for PublicGetTradeHistoryJsonrpcSchema {
    fn from(value: PublicGetTradeHistory) -> Self {
        value.0
    }
}
impl From<&PublicGetTradeHistory> for PublicGetTradeHistory {
    fn from(value: &PublicGetTradeHistory) -> Self {
        value.clone()
    }
}
impl From<PublicGetTradeHistoryJsonrpcSchema> for PublicGetTradeHistory {
    fn from(value: PublicGetTradeHistoryJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PublicGetTradeHistoryJsonrpcSchema
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
      "$ref": "#/definitions/PublicGetTradeHistoryRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTradeHistoryResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTradeHistoryJsonrpcSchema {
    pub request: PublicGetTradeHistoryRequestSchema,
    pub response: PublicGetTradeHistoryResponseSchema,
}
impl From<&PublicGetTradeHistoryJsonrpcSchema> for PublicGetTradeHistoryJsonrpcSchema {
    fn from(value: &PublicGetTradeHistoryJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PublicGetTradeHistoryParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency to filter by (defaults to all)",
      "type": [
        "string",
        "null"
      ]
    },
    "from_timestamp": {
      "title": "from_timestamp",
      "description": "Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.",
      "default": 0,
      "type": "integer"
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name to filter by (defaults to all)",
      "type": [
        "string",
        "null"
      ]
    },
    "instrument_type": {
      "title": "instrument_type",
      "description": "Instrument type to filter by (defaults to all)",
      "type": [
        "string",
        "null"
      ],
      "enum": [
        "erc20",
        "option",
        "perp"
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
    "to_timestamp": {
      "title": "to_timestamp",
      "description": "Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.",
      "default": 18446744073709551615,
      "type": "integer"
    },
    "trade_id": {
      "title": "trade_id",
      "description": "Trade ID to filter by. If set, all other filters are ignored",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "tx_hash": {
      "title": "tx_hash",
      "description": "On-chain tx hash to filter by. If set, all other filters are ignored",
      "type": [
        "string",
        "null"
      ]
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Transaction status to filter by (default `settled`).",
      "default": "settled",
      "type": "string",
      "enum": [
        "settled",
        "reverted"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTradeHistoryParamsSchema {
    ///Currency to filter by (defaults to all)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Earliest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to 0.
    #[serde(default)]
    pub from_timestamp: i64,
    ///Instrument name to filter by (defaults to all)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    ///Instrument type to filter by (defaults to all)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<InstrumentType>,
    ///Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    ///Number of results per page (default 100, max 1000)
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
    ///Latest timestamp to filter by (in ms since Unix epoch). If not provied, defaults to returning all data up to current time.
    #[serde(default = "defaults::default_u64::<i64, 18446744073709551615>")]
    pub to_timestamp: i64,
    ///Trade ID to filter by. If set, all other filters are ignored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_id: Option<uuid::Uuid>,
    ///On-chain tx hash to filter by. If set, all other filters are ignored
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    ///Transaction status to filter by (default `settled`).
    #[serde(default = "defaults::public_get_trade_history_params_schema_tx_status")]
    pub tx_status: TxStatus,
}
impl From<&PublicGetTradeHistoryParamsSchema> for PublicGetTradeHistoryParamsSchema {
    fn from(value: &PublicGetTradeHistoryParamsSchema) -> Self {
        value.clone()
    }
}
///PublicGetTradeHistoryRequestSchema
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
      "const": "public/get_trade_history"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PublicGetTradeHistoryParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTradeHistoryRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PublicGetTradeHistoryRequestSchemaId>,
    pub method: String,
    pub params: PublicGetTradeHistoryParamsSchema,
}
impl From<&PublicGetTradeHistoryRequestSchema> for PublicGetTradeHistoryRequestSchema {
    fn from(value: &PublicGetTradeHistoryRequestSchema) -> Self {
        value.clone()
    }
}
///PublicGetTradeHistoryRequestSchemaId
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
pub enum PublicGetTradeHistoryRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTradeHistoryRequestSchemaId>
for PublicGetTradeHistoryRequestSchemaId {
    fn from(value: &PublicGetTradeHistoryRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTradeHistoryRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTradeHistoryRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTradeHistoryRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTradeHistoryRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetTradeHistoryResponseSchema
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
      "$ref": "#/definitions/PublicGetTradeHistoryResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTradeHistoryResponseSchema {
    pub id: PublicGetTradeHistoryResponseSchemaId,
    ///
    pub result: PublicGetTradeHistoryResultSchema,
}
impl From<&PublicGetTradeHistoryResponseSchema> for PublicGetTradeHistoryResponseSchema {
    fn from(value: &PublicGetTradeHistoryResponseSchema) -> Self {
        value.clone()
    }
}
///PublicGetTradeHistoryResponseSchemaId
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
pub enum PublicGetTradeHistoryResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PublicGetTradeHistoryResponseSchemaId>
for PublicGetTradeHistoryResponseSchemaId {
    fn from(value: &PublicGetTradeHistoryResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PublicGetTradeHistoryResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PublicGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PublicGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PublicGetTradeHistoryResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PublicGetTradeHistoryResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PublicGetTradeHistoryResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PublicGetTradeHistoryResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "pagination",
    "trades"
  ],
  "properties": {
    "pagination": {
      "description": "Pagination info",
      "type": "object",
      "$ref": "#/definitions/PaginationInfoSchema",
      "field_many": false
    },
    "trades": {
      "title": "trades",
      "description": "List of trades",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/TradeSettledPublicResponseSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PublicGetTradeHistoryResultSchema {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///List of trades
    pub trades: Vec<TradeSettledPublicResponseSchema>,
}
impl From<&PublicGetTradeHistoryResultSchema> for PublicGetTradeHistoryResultSchema {
    fn from(value: &PublicGetTradeHistoryResultSchema) -> Self {
        value.clone()
    }
}
///TradeSettledPublicResponseSchema
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
    "liquidity_role",
    "mark_price",
    "quote_id",
    "realized_pnl",
    "subaccount_id",
    "timestamp",
    "trade_amount",
    "trade_fee",
    "trade_id",
    "trade_price",
    "tx_hash",
    "tx_status",
    "wallet"
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
    "quote_id": {
      "title": "quote_id",
      "description": "Quote ID if the trade was executed via RFQ",
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
      "type": "string"
    },
    "tx_status": {
      "title": "tx_status",
      "description": "Blockchain transaction status",
      "type": "string",
      "enum": [
        "settled",
        "reverted"
      ]
    },
    "wallet": {
      "title": "wallet",
      "description": "Wallet address (owner) of the subaccount",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeSettledPublicResponseSchema {
    ///Order direction
    pub direction: Direction,
    ///Index price of the underlying at the time of the trade
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Role of the user in the trade
    pub liquidity_role: LiquidityRole,
    ///Mark price of the instrument at the time of the trade
    pub mark_price: bigdecimal::BigDecimal,
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
    pub tx_hash: String,
    ///Blockchain transaction status
    pub tx_status: TxStatus,
    ///Wallet address (owner) of the subaccount
    pub wallet: String,
}
impl From<&TradeSettledPublicResponseSchema> for TradeSettledPublicResponseSchema {
    fn from(value: &TradeSettledPublicResponseSchema) -> Self {
        value.clone()
    }
}
///Transaction status to filter by (default `settled`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "tx_status",
  "description": "Transaction status to filter by (default `settled`).",
  "default": "settled",
  "type": "string",
  "enum": [
    "settled",
    "reverted"
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
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
}
impl From<&TxStatus> for TxStatus {
    fn from(value: &TxStatus) -> Self {
        value.clone()
    }
}
impl ToString for TxStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Settled => "settled".to_string(),
            Self::Reverted => "reverted".to_string(),
        }
    }
}
impl std::str::FromStr for TxStatus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
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
impl Default for TxStatus {
    fn default() -> Self {
        TxStatus::Settled
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
    pub(super) fn public_get_trade_history_params_schema_tx_status() -> super::TxStatus {
        super::TxStatus::Settled
    }
}
