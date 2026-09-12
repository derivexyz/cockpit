#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::types::orders::enums::{
    AlgoType, CancelReason, Direction, LiquidityRole, OrderStatus, OrderType, TimeInForce,
    TriggerPriceType, TriggerType,
};
use crate::types::shared::{serde_nonce, serde_option_nonce, RPCId};
use bigdecimal;
use serde::{Deserialize, Serialize};
use uuid;

pub fn bool_true() -> bool {
    true
}

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
    /// Unique nonce: UTC nanoseconds as a decimal string
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    ///Order type:<br />- `limit`: limit order (default)<br />- `market`: market order, note that limit_price is still required for market orders, but unfilled order portion will be marked as cancelled
    #[serde(default = "defaults::order_params_order_type")]
    pub order_type: OrderType,
    ///If true, the order will not be able to increase position's size (default false). If the order amount exceeds available position size, the order will be filled up to the position size and the remainder will be cancelled. This flag is only supported for market orders or non-resting limit orders (IOC or FOK)
    #[serde(default)]
    pub reduce_only: bool,
    /// Optional extra fee per unit of volume
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<bigdecimal::BigDecimal>,
    /// Optional referral code for the order
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
    /// UTC timestamp in ms; rejected if `reject_timestamp` < server time
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_timestamp: Option<i64>,
    /// Ethereum signature of the order
    pub signature: String,
    /// Unix timestamp in seconds. Order signature becomes invalid after this time.
    pub signature_expiry_sec: i64,
    /// Owner wallet address or registered session key that signed order
    pub signer: String,
    /// Subaccount ID
    pub subaccount_id: i64,
    /// Time in force
    #[serde(default = "defaults::order_params_time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_atomic_signing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
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
    /// Unique nonce: UTC nanoseconds as a decimal string
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    ///Cancel order by nonce (choose either order_id or nonce).
    #[serde(default, skip_serializing_if = "Option::is_none", with = "serde_option_nonce")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_timestamp: Option<i64>,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    #[serde(default = "defaults::order_params_time_in_force")]
    pub time_in_force: TimeInForce,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_atomic_signing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reject_post_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<TriggerType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price_type: Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_type: Option<AlgoType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_duration_sec: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub algo_num_slices: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RPCId>,
    pub method: String,
    pub params: ReplaceParams,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GetTradesParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instrument_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
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
