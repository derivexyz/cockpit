#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::types::orders::enums::{
    CancelReason, Direction, LiquidityRole, OrderStatus, OrderType, TimeInForce,
};
use crate::types::shared::RPCId;
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderParams {
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
    #[serde(default = "defaults::order_params_order_type")]
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
    ///Ethereum signature of the order
    pub signature: String,
    ///Unix timestamp in seconds. Order signature becomes invalid after this time, and the system will cancel the order.Expiry MUST be at least 5 min from now.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed order
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Time in force behaviour:<br />- `gtc`: good til cancelled (default)<br />- `post_only`: a limit order that will be rejected if it crosses any order in the book, i.e. acts as a taker order<br />- `fok`: fill or kill, will be rejected if it is not fully filled<br />- `ioc`: immediate or cancel, fill at best bid/ask (market) or at limit price (limit), the unfilled portion is cancelled<br />Note that the order will still expire on the `signature_expiry_sec` timestamp.
    #[serde(default = "defaults::order_params_time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(default)]
    pub is_atomic_signing: bool,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RPCId>,
    pub method: String,
    pub params: OrderParams,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceParams {
    ///Order amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Order direction
    pub direction: Direction,
    ///Optional check to only create new order if old order filled_amount is equal to this value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_filled_amount: Option<bigdecimal::BigDecimal>,
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
    ///Cancel order by nonce (choose either order_id or nonce).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce_to_cancel: Option<i64>,
    ///Cancel order by order_id (choose either order_id or nonce).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id_to_cancel: Option<uuid::Uuid>,
    ///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
    #[serde(default = "defaults::order_params_order_type")]
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
    #[serde(default = "defaults::order_params_time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(default)]
    pub is_atomic_signing: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RPCId>,
    pub method: String,
    pub params: ReplaceParams,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTradesParams {
    #[serde(default)]
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(default = "defaults::default_u64::<i64, 0>")]
    pub from_timestamp: i64,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub to_timestamp: i64,
    #[serde(default = "defaults::default_u64::<i64, 1>")]
    pub page: i64,
    #[serde(default = "defaults::default_u64::<i64, 100>")]
    pub page_size: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTradesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RPCId>,
    pub method: String,
    pub params: GetTradesParams,
}

pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: std::convert::TryFrom<u64>,
        <T as std::convert::TryFrom<u64>>::Error: std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn order_params_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn order_params_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
}
