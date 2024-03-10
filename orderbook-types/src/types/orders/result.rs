use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::types::orders::enums::{CancelReason, Direction, OrderStatus, OrderType, TimeInForce, TxStatus, LiquidityRole};
use crate::types::shared::{RPCId, RPCError};
use uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderResponse {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Average fill price
    pub average_price: bigdecimal::BigDecimal,
    ///If cancelled, reason behind order cancellation
    pub cancel_reason: CancelReason,
    ///Creation timestamp (in ms since Unix epoch)
    pub creation_timestamp: i64,
    ///Order direction
    pub direction: Direction,
    ///Total filled amount for the order
    pub filled_amount: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///Whether the order was generated through `private/transfer_position`
    pub is_transfer: bool,
    ///Optional user-defined label for the order
    pub label: String,
    ///Last update timestamp (in ms since Unix epoch)
    pub last_update_timestamp: i64,
    ///Limit price in quote currency
    pub limit_price: bigdecimal::BigDecimal,
    ///Max fee in units of the quote currency
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the order is tagged for market maker protections
    pub mmp: bool,
    ///Unique nonce defined as <UTC_timestamp in ms><random_number_up_to_3_digits> (e.g. 16958360587
    pub nonce: i64,
    ///Total order fee paid so far
    pub order_fee: bigdecimal::BigDecimal,
    ///Order ID
    pub order_id: String,
    ///Order status
    pub order_status: OrderStatus,
    ///Order type
    pub order_type: OrderType,
    ///Quote ID if the trade was executed via RFQ
    pub quote_id: Option<uuid::Uuid>,
    ///If replaced, ID of the order that was replaced
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replaced_order_id: Option<uuid::Uuid>,
    ///Ethereum signature of the order
    pub signature: String,
    ///Signature expiry timestamp
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Time in force
    pub time_in_force: TimeInForce,

    pub trigger_type: Option<Value>, // todo actual enum
    pub trigger_price_type: Option<Value>, // todo actual enum
    pub trigger_price: Option<bigdecimal::BigDecimal>,
    pub trigger_reject_message: Option<String>,
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

pub type OrderNotificationData = Vec<OrderResponse>;