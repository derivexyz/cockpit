use ethers::prelude::{LocalWallet, Signer, EthAbiCodec, Address, U256, I256, EthAbiType, Signature};
use bigdecimal::BigDecimal;
use anyhow::{Result};
use ethers::abi::AbiEncode;
use ethers::utils::hex;
use uuid::Uuid;
pub use orderbook_types::types::orders::{OrderParams, OrderResponse, ReplaceParams, OrderType, OrderStatus, TimeInForce, Direction, LiquidityRole};

use std::str::FromStr;
use orderbook_types::generated::channel_ticker_instrument_name_interval::InstrumentTickerSchema;
use orderbook_types::generated::public_get_ticker::PublicGetTickerResultSchema;
use orderbook_types::generated::channel_subaccount_id_orders;
use crate::utils::{decimal_to_u256, decimal_to_i256};

/// Subset of ticker info required for order signing.
pub trait OrderTicker {
    fn get_name(&self) -> String;
    fn get_max_fee(&self) -> BigDecimal;
    fn get_sub_id(&self) -> Result<U256>;
    fn get_address(&self) -> Result<Address>;
    fn get_unit_fee(&self, role: LiquidityRole) -> BigDecimal;
}

impl OrderTicker for &InstrumentTickerSchema {
    fn get_name(&self) -> String {
        self.instrument_name.clone()
    }
    fn get_max_fee(&self) -> BigDecimal {
        BigDecimal::from(3) * &self.taker_fee_rate * &self.index_price
    }
    fn get_sub_id(&self) -> Result<U256> {
        Ok(self.base_asset_sub_id.parse::<u128>()?.into())
    }
    fn get_address(&self) -> Result<Address> {
        Ok(self.base_asset_address.parse()?)
    }
    fn get_unit_fee(&self, role: LiquidityRole) -> BigDecimal {
        match role {
            LiquidityRole::Maker => self.maker_fee_rate.clone() * &self.index_price,
            LiquidityRole::Taker => self.taker_fee_rate.clone() * &self.index_price,
        }
    }
}

// todo can impl for the other ticker structs like get_ticker endpoint result

pub fn opposite(direction: Direction) -> Direction {
    match direction {
        Direction::Buy => Direction::Sell,
        Direction::Sell => Direction::Buy
    }
}

pub struct OrderArgs {
    pub amount: BigDecimal,
    pub limit_price: BigDecimal,
    pub direction: Direction,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub label: String,
    pub mmp: bool,
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct TradeData {
    address: Address,
    sub_id: U256,
    limit_price: I256,
    amount: I256,
    max_fee: U256,
    subaccount_id: U256,
    is_bid: bool,
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct ActionData {
    action_typehash: [u8; 32],
    subaccount_id: U256,
    nonce: U256,
    module: Address,
    data: [u8; 32],
    expiry: U256,
    owner: Address,
    signer: Address,
}

fn get_order_signature(
    subaccount_id: i64,
    nonce: i64,
    signature_expiry_sec: i64,
    limit_price: BigDecimal,
    amount: BigDecimal,
    is_bid: bool,
    max_fee: BigDecimal, signer: &LocalWallet, ticker: impl OrderTicker) -> Result<Signature>
{
    let trade_data = TradeData {
        address: ticker.get_address()?,
        sub_id: ticker.get_sub_id()?,
        limit_price: decimal_to_i256(limit_price)?,
        amount: decimal_to_i256(amount)?,
        max_fee: decimal_to_u256(max_fee)?,
        subaccount_id: subaccount_id.into(),
        is_bid
    };
    let encoded_data = trade_data.encode();
    let hashed_data = ethers::utils::keccak256(&encoded_data);
    // env var
    let action_typehash = std::env::var("ACTION_TYPEHASH").expect("ACTION_TYPEHASH must be set");
    let action_typehash = hex::const_decode_to_array::<32>(action_typehash.as_bytes())?;
    let domain_sep = std::env::var("DOMAIN_SEPARATOR").expect("DOMAIN_SEPARATOR must be set");
    let domain_sep = hex::decode(domain_sep)?;
    let prefix = hex::decode("1901")?;
    let trade_module = std::env::var("TRADE_ADDRESS").expect("TRADE_ADDRESS must be set");
    let action_data = ActionData {
        action_typehash,
        subaccount_id: subaccount_id.into(),
        nonce: nonce.into(),
        module: trade_module.parse()?,
        data: hashed_data,
        expiry: signature_expiry_sec.into(),
        owner: signer.address(),
        signer: signer.address(),
    };
    let action_hash = ethers::utils::keccak256(&action_data.encode());
    let typed_data_hash = ethers::utils::keccak256(&[prefix, domain_sep, action_hash.into()].concat());
    let signature = signer.sign_hash(typed_data_hash.into())?;
    Ok(signature)
}

pub fn new_order_params(
    signer: &LocalWallet,
    ticker: impl OrderTicker,
    subaccount_id: i64,
    args: OrderArgs,
) -> Result<OrderParams>
{
    let max_fee = ticker.get_max_fee();
    let now = chrono::Utc::now();
    let nonce = now.timestamp_nanos_opt().unwrap();
    let reject_timestamp = (now + chrono::Duration::seconds(5)).timestamp_millis();
    let signature_expiry_sec = (now + chrono::Duration::seconds(345)).timestamp();
    let mut params = OrderParams {
        instrument_name: ticker.get_name(),
        subaccount_id,
        amount: args.amount,
        limit_price: args.limit_price,
        direction: args.direction,
        time_in_force: args.time_in_force,
        order_type: args.order_type,
        mmp: args.mmp,
        max_fee,
        label: args.label,
        nonce,
        reject_timestamp,
        signature_expiry_sec,
        signer: hex::encode_prefixed(signer.address()),
        reduce_only: false,
        replaced_order_id: None,
        referral_code: "".to_string(),
        signature: "".to_string(),
    };
    let signature = get_order_signature(
        params.subaccount_id,
        params.nonce,
        params.signature_expiry_sec,
        params.limit_price.clone(),
        params.amount.clone(),
        params.direction == Direction::Buy,
        params.max_fee.clone(),
        signer,
        ticker,
    );
    params.signature = signature?.to_string();
    Ok(params)
}

pub fn new_replace_params(
    signer: &LocalWallet,
    ticker: impl OrderTicker,
    subaccount_id: i64,
    order_id_to_cancel: Uuid,
    args: OrderArgs,
) -> Result<ReplaceParams>
{
    let max_fee = ticker.get_max_fee();
    let now = chrono::Utc::now();
    let nonce = now.timestamp_nanos_opt().unwrap();
    let reject_timestamp = (now + chrono::Duration::seconds(5)).timestamp_millis();
    let signature_expiry_sec = (now + chrono::Duration::seconds(345)).timestamp();
    let mut params = ReplaceParams {
        instrument_name: ticker.get_name(),
        subaccount_id,
        amount: args.amount,
        limit_price: args.limit_price,
        direction: args.direction,
        time_in_force: args.time_in_force,
        order_type: args.order_type,
        mmp: args.mmp,
        max_fee,
        label: args.label,
        nonce,
        reject_timestamp,
        signature_expiry_sec,
        signer: hex::encode_prefixed(signer.address()),
        reduce_only: false,
        replaced_order_id: None,
        referral_code: "".to_string(),
        signature: "".to_string(),
        expected_filled_amount: None,
        nonce_to_cancel: None,
        order_id_to_cancel: Some(order_id_to_cancel),
    };
    let signature = get_order_signature(
        params.subaccount_id,
        params.nonce,
        params.signature_expiry_sec,
        params.limit_price.clone(),
        params.amount.clone(),
        params.direction == Direction::Buy,
        params.max_fee.clone(),
        signer,
        ticker,
    );
    params.signature = signature?.to_string();
    Ok(params)
}
