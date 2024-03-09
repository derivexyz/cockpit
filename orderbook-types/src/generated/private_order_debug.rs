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
///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "order_type",
  "description": "Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled",
  "default": "limit",
  "type": "string",
  "enum": [
    "limit",
    "market"
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
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}
impl From<&OrderType> for OrderType {
    fn from(value: &OrderType) -> Self {
        value.clone()
    }
}
impl ToString for OrderType {
    fn to_string(&self) -> String {
        match *self {
            Self::Limit => "limit".to_string(),
            Self::Market => "market".to_string(),
        }
    }
}
impl std::str::FromStr for OrderType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "limit" => Ok(Self::Limit),
            "market" => Ok(Self::Market),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for OrderType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OrderType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OrderType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for OrderType {
    fn default() -> Self {
        OrderType::Limit
    }
}
///Debug a new order
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/order_debug",
  "description": "Debug a new order",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateOrderDebugJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateOrderDebug(pub PrivateOrderDebugJsonrpcSchema);
impl std::ops::Deref for PrivateOrderDebug {
    type Target = PrivateOrderDebugJsonrpcSchema;
    fn deref(&self) -> &PrivateOrderDebugJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateOrderDebug> for PrivateOrderDebugJsonrpcSchema {
    fn from(value: PrivateOrderDebug) -> Self {
        value.0
    }
}
impl From<&PrivateOrderDebug> for PrivateOrderDebug {
    fn from(value: &PrivateOrderDebug) -> Self {
        value.clone()
    }
}
impl From<PrivateOrderDebugJsonrpcSchema> for PrivateOrderDebug {
    fn from(value: PrivateOrderDebugJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateOrderDebugJsonrpcSchema
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
      "$ref": "#/definitions/PrivateOrderDebugRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateOrderDebugResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderDebugJsonrpcSchema {
    pub request: PrivateOrderDebugRequestSchema,
    pub response: PrivateOrderDebugResponseSchema,
}
impl From<&PrivateOrderDebugJsonrpcSchema> for PrivateOrderDebugJsonrpcSchema {
    fn from(value: &PrivateOrderDebugJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderDebugParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "amount",
    "direction",
    "instrument_name",
    "limit_price",
    "max_fee",
    "nonce",
    "signature",
    "signature_expiry_sec",
    "signer",
    "subaccount_id"
  ],
  "properties": {
    "amount": {
      "title": "amount",
      "description": "Order amount in units of the base",
      "type": "string",
      "format": "decimal"
    },
    "direction": {
      "title": "direction",
      "description": "Order direction",
      "type": "string",
      "enum": [
        "buy",
        "sell"
      ]
    },
    "instrument_name": {
      "title": "instrument_name",
      "description": "Instrument name",
      "type": "string"
    },
    "label": {
      "title": "label",
      "description": "Optional user-defined label for the order",
      "default": "",
      "type": "string"
    },
    "limit_price": {
      "title": "limit_price",
      "description": "Limit price in quote currency.<br />This field is still required for market orders because it is a component of the signature. However, market orders will not leave a resting order in the book in case of a partial fill.",
      "type": "string",
      "format": "decimal"
    },
    "max_fee": {
      "title": "max_fee",
      "description": "Max fee in units of the quote currency. Order will be rejected if the supplied max fee is below the estimated fee for this order.",
      "type": "string",
      "format": "decimal"
    },
    "mmp": {
      "title": "mmp",
      "description": "Whether the order is tagged for market maker protections (default false)",
      "default": false,
      "type": "boolean"
    },
    "nonce": {
      "title": "nonce",
      "description": "Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)",
      "type": "integer"
    },
    "order_type": {
      "title": "order_type",
      "description": "Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled",
      "default": "limit",
      "type": "string",
      "enum": [
        "limit",
        "market"
      ]
    },
    "reduce_only": {
      "title": "reduce_only",
      "description": "If true, the order will not be able to increase position's size (default false). If the order amount exceeds available position size, the order will be filled up to the position size and the remainder will be cancelled. This flag is only supported for market orders or non-resting limit orders (IOC or FOK)",
      "default": false,
      "type": "boolean"
    },
    "referral_code": {
      "title": "referral_code",
      "description": "Optional referral code for the order",
      "default": "",
      "type": "string"
    },
    "reject_timestamp": {
      "title": "reject_timestamp",
      "description": "UTC timestamp in ms, if provided the matching engine will reject the order with an error if `reject_timestamp` < `server_time`. Note that the timestamp must be consistent with the server time: use `public/get_time` method to obtain current server time.",
      "default": 9223372036854775807,
      "type": "integer"
    },
    "replaced_order_id": {
      "title": "replaced_order_id",
      "description": "If replaced, ID of the order that was replaced",
      "type": [
        "string",
        "null"
      ],
      "format": "uuid"
    },
    "signature": {
      "title": "signature",
      "description": "Etherium signature of the order",
      "type": "string"
    },
    "signature_expiry_sec": {
      "title": "signature_expiry_sec",
      "description": "Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.",
      "type": "integer"
    },
    "signer": {
      "title": "signer",
      "description": "Owner wallet address or registered session key that signed order",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    },
    "time_in_force": {
      "title": "time_in_force",
      "description": "Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.",
      "default": "gtc",
      "type": "string",
      "enum": [
        "gtc",
        "post_only",
        "fok",
        "ioc"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderDebugParamsSchema {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Order direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
    ///Optional user-defined label for the order
    #[serde(default)]
    pub label: String,
    ///Limit price in quote currency.<br />This field is still required for market orders because it is a component of the signature. However, market orders will not leave a resting order in the book in case of a partial fill.
    pub limit_price: bigdecimal::BigDecimal,
    ///Max fee in units of the quote currency. Order will be rejected if the supplied max fee is below the estimated fee for this order.
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the order is tagged for market maker protections (default false)
    #[serde(default)]
    pub mmp: bool,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_6_digits> (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
    #[serde(default = "defaults::private_order_debug_params_schema_order_type")]
    pub order_type: OrderType,
    ///If true, the order will not be able to increase position's size (default false). If the order amount exceeds available position size, the order will be filled up to the position size and the remainder will be cancelled. This flag is only supported for market orders or non-resting limit orders (IOC or FOK)
    #[serde(default)]
    pub reduce_only: bool,
    ///Optional referral code for the order
    #[serde(default)]
    pub referral_code: String,
    ///UTC timestamp in ms, if provided the matching engine will reject the order with an error if `reject_timestamp` < `server_time`. Note that the timestamp must be consistent with the server time: use `public/get_time` method to obtain current server time.
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub reject_timestamp: i64,
    ///If replaced, ID of the order that was replaced
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<uuid::Uuid>,
    ///Etherium signature of the order
    pub signature: String,
    ///Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.
    #[serde(default = "defaults::private_order_debug_params_schema_time_in_force")]
    pub time_in_force: TimeInForce,
}
impl From<&PrivateOrderDebugParamsSchema> for PrivateOrderDebugParamsSchema {
    fn from(value: &PrivateOrderDebugParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderDebugRequestSchema
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
      "const": "private/order_debug"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateOrderDebugParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderDebugRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateOrderDebugRequestSchemaId>,
    pub method: String,
    pub params: PrivateOrderDebugParamsSchema,
}
impl From<&PrivateOrderDebugRequestSchema> for PrivateOrderDebugRequestSchema {
    fn from(value: &PrivateOrderDebugRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderDebugRequestSchemaId
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
pub enum PrivateOrderDebugRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateOrderDebugRequestSchemaId> for PrivateOrderDebugRequestSchemaId {
    fn from(value: &PrivateOrderDebugRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateOrderDebugRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateOrderDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateOrderDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateOrderDebugRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateOrderDebugRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateOrderDebugRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateOrderDebugResponseSchema
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
      "$ref": "#/definitions/PrivateOrderDebugResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderDebugResponseSchema {
    pub id: PrivateOrderDebugResponseSchemaId,
    ///
    pub result: PrivateOrderDebugResultSchema,
}
impl From<&PrivateOrderDebugResponseSchema> for PrivateOrderDebugResponseSchema {
    fn from(value: &PrivateOrderDebugResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateOrderDebugResponseSchemaId
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
pub enum PrivateOrderDebugResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateOrderDebugResponseSchemaId> for PrivateOrderDebugResponseSchemaId {
    fn from(value: &PrivateOrderDebugResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateOrderDebugResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateOrderDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateOrderDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateOrderDebugResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateOrderDebugResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateOrderDebugResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateOrderDebugResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "action_hash",
    "encoded_data",
    "encoded_data_hashed",
    "raw_data",
    "typed_data_hash"
  ],
  "properties": {
    "action_hash": {
      "title": "action_hash",
      "description": "Keccak hashed action data",
      "type": "string"
    },
    "encoded_data": {
      "title": "encoded_data",
      "description": "ABI encoded order data",
      "type": "string"
    },
    "encoded_data_hashed": {
      "title": "encoded_data_hashed",
      "description": "Keccak hashed encoded_data",
      "type": "string"
    },
    "raw_data": {
      "description": "Raw order data",
      "type": "object",
      "$ref": "#/definitions/SignedTradeOrderSchema",
      "field_many": false
    },
    "typed_data_hash": {
      "title": "typed_data_hash",
      "description": "EIP 712 typed data hash",
      "type": "string"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateOrderDebugResultSchema {
    ///Keccak hashed action data
    pub action_hash: String,
    ///ABI encoded order data
    pub encoded_data: String,
    ///Keccak hashed encoded_data
    pub encoded_data_hashed: String,
    ///Raw order data
    pub raw_data: SignedTradeOrderSchema,
    ///EIP 712 typed data hash
    pub typed_data_hash: String,
}
impl From<&PrivateOrderDebugResultSchema> for PrivateOrderDebugResultSchema {
    fn from(value: &PrivateOrderDebugResultSchema) -> Self {
        value.clone()
    }
}
///SignedTradeOrderSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "data",
    "expiry",
    "module",
    "nonce",
    "owner",
    "signature",
    "signer",
    "subaccount_id"
  ],
  "properties": {
    "data": {
      "type": "object",
      "$ref": "#/definitions/TradeModuleDataSchema",
      "field_many": false
    },
    "expiry": {
      "title": "expiry",
      "type": "integer"
    },
    "module": {
      "title": "module",
      "type": "string"
    },
    "nonce": {
      "title": "nonce",
      "type": "integer"
    },
    "owner": {
      "title": "owner",
      "type": "string"
    },
    "signature": {
      "title": "signature",
      "type": "string"
    },
    "signer": {
      "title": "signer",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SignedTradeOrderSchema {
    pub data: TradeModuleDataSchema,
    pub expiry: i64,
    pub module: String,
    pub nonce: i64,
    pub owner: String,
    pub signature: String,
    pub signer: String,
    pub subaccount_id: i64,
}
impl From<&SignedTradeOrderSchema> for SignedTradeOrderSchema {
    fn from(value: &SignedTradeOrderSchema) -> Self {
        value.clone()
    }
}
///Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "time_in_force",
  "description": "Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.",
  "default": "gtc",
  "type": "string",
  "enum": [
    "gtc",
    "post_only",
    "fok",
    "ioc"
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
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    Fok,
    #[serde(rename = "ioc")]
    Ioc,
}
impl From<&TimeInForce> for TimeInForce {
    fn from(value: &TimeInForce) -> Self {
        value.clone()
    }
}
impl ToString for TimeInForce {
    fn to_string(&self) -> String {
        match *self {
            Self::Gtc => "gtc".to_string(),
            Self::PostOnly => "post_only".to_string(),
            Self::Fok => "fok".to_string(),
            Self::Ioc => "ioc".to_string(),
        }
    }
}
impl std::str::FromStr for TimeInForce {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "gtc" => Ok(Self::Gtc),
            "post_only" => Ok(Self::PostOnly),
            "fok" => Ok(Self::Fok),
            "ioc" => Ok(Self::Ioc),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TimeInForce {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::Gtc
    }
}
///TradeModuleDataSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "asset",
    "desired_amount",
    "is_bid",
    "limit_price",
    "recipient_id",
    "sub_id",
    "trade_id",
    "worst_fee"
  ],
  "properties": {
    "asset": {
      "title": "asset",
      "type": "string"
    },
    "desired_amount": {
      "title": "desired_amount",
      "type": "string",
      "format": "decimal"
    },
    "is_bid": {
      "title": "is_bid",
      "type": "boolean"
    },
    "limit_price": {
      "title": "limit_price",
      "type": "string",
      "format": "decimal"
    },
    "recipient_id": {
      "title": "recipient_id",
      "type": "integer"
    },
    "sub_id": {
      "title": "sub_id",
      "type": "integer"
    },
    "trade_id": {
      "title": "trade_id",
      "type": "string"
    },
    "worst_fee": {
      "title": "worst_fee",
      "type": "string",
      "format": "decimal"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct TradeModuleDataSchema {
    pub asset: String,
    pub desired_amount: bigdecimal::BigDecimal,
    pub is_bid: bool,
    pub limit_price: bigdecimal::BigDecimal,
    pub recipient_id: i64,
    pub sub_id: i64,
    pub trade_id: String,
    pub worst_fee: bigdecimal::BigDecimal,
}
impl From<&TradeModuleDataSchema> for TradeModuleDataSchema {
    fn from(value: &TradeModuleDataSchema) -> Self {
        value.clone()
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
    pub(super) fn private_order_debug_params_schema_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn private_order_debug_params_schema_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
}
