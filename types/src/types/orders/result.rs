use crate::types::orders::enums::{
    AlgoType, BatchStatus, CancelReason, Direction, LiquidityRole, OrderStatus, OrderType,
    TimeInForce, TriggerPriceType, TriggerType,
};
use crate::types::shared::{serde_nonce, PaginationInfoSchema, RPCError, RPCId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderResponse {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Average fill price
    pub average_price: bigdecimal::BigDecimal,
    /// If cancelled, reason behind order cancellation
    #[serde(default)]
    pub cancel_reason: CancelReason,
    /// Creation timestamp (in ms since Unix epoch)
    pub creation_timestamp: i64,
    /// Order direction
    pub direction: Direction,
    /// Extra fee per unit of volume
    #[serde(default)]
    pub extra_fee: bigdecimal::BigDecimal,
    /// Total filled amount for the order
    pub filled_amount: bigdecimal::BigDecimal,
    /// Instrument name
    pub instrument_name: String,
    /// Whether the order was generated through `private/transfer_position`
    pub is_transfer: bool,
    /// Optional user-defined label for the order
    #[serde(default)]
    pub label: String,
    /// Last update timestamp (in ms since Unix epoch)
    pub last_update_timestamp: i64,
    /// Limit price in quote currency
    pub limit_price: bigdecimal::BigDecimal,
    /// Max fee in units of the quote currency
    pub max_fee: bigdecimal::BigDecimal,
    /// Whether the order is tagged for market maker protections
    pub mmp: bool,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    /// Total order fee paid so far
    pub order_fee: bigdecimal::BigDecimal,
    /// Order ID
    pub order_id: String,
    /// Order status
    pub order_status: OrderStatus,
    /// Order type
    pub order_type: OrderType,
    /// Quote ID if the trade was executed via RFQ
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    /// If replaced, ID of the order that was replaced
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_limit_price: Option<bigdecimal::BigDecimal>,
    /// Ethereum signature of the order
    pub signature: String,
    /// Signature expiry timestamp
    pub signature_expiry_sec: i64,
    /// Owner wallet address or registered session key that signed order
    pub signer: String,
    /// Subaccount ID
    pub subaccount_id: i64,
    /// Time in force
    pub time_in_force: TimeInForce,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_reject_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_slices_completed: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TradeResponse {
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
    /// Quote ID if the trade was executed via RFQ
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    /// Realized PnL for this trade
    pub realized_pnl: bigdecimal::BigDecimal,
    #[serde(default)]
    pub realized_pnl_excl_fees: bigdecimal::BigDecimal,
    /// Subaccount ID
    pub subaccount_id: i64,
    /// Trade timestamp (in ms since Unix epoch)
    pub timestamp: i64,
    /// Amount filled in this trade
    pub trade_amount: bigdecimal::BigDecimal,
    /// Fee for this trade
    pub trade_fee: bigdecimal::BigDecimal,
    pub expected_rebate: bigdecimal::BigDecimal,
    #[serde(default)]
    pub extra_fee: bigdecimal::BigDecimal,
    /// Trade ID
    pub trade_id: String,
    /// Price at which the trade was filled
    pub trade_price: bigdecimal::BigDecimal,
    /// Settling operation UUID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op_uuid: Option<String>,
    /// Blockchain transaction hash
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_hash: Option<String>,
    /// Batch lifecycle status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<BatchStatus>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendOrderResult {
    pub order: OrderResponse,
    pub trades: Vec<TradeResponse>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendOrderResponse {
    pub id: RPCId,
    pub result: SendOrderResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceResult {
    ///Order that was cancelled
    pub cancelled_order: OrderResponse,
    ///Optional. Returns error during new order creation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_order_error: Option<RPCError>,
    ///New order that was created
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<OrderResponse>,
    ///List of trades executed by the created order
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeResponse>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceResponse {
    pub id: RPCId,
    pub result: ReplaceResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTradesResult {
    pub subaccount_id: Option<i64>,
    pub trades: Vec<TradeResponse>,
    pub pagination: PaginationInfoSchema,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTradesResponse {
    pub id: RPCId,
    pub result: GetTradesResult,
}

pub type OrderNotificationData = Vec<OrderResponse>;
pub type TradeNotificationData = Vec<TradeResponse>;
