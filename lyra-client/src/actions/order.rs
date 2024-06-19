use crate::actions::helpers::ModuleData;
use crate::actions::ActionData;
use crate::utils::{decimal_to_i256, decimal_to_u256, decimal_to_u256_with_prec};
use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, EthAbiCodec, EthAbiType, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;
pub use orderbook_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
use orderbook_types::types::tickers::InstrumentTicker;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
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
pub struct TradeData {
    asset_address: Address,
    sub_id: U256,
    limit_price: I256,
    amount: I256,
    max_fee: U256,
    subaccount_id: U256,
    is_bid: bool,
}

impl TradeData {
    pub fn new(
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        limit_price: BigDecimal,
        amount: BigDecimal,
        is_bid: bool,
    ) -> Result<Self> {
        Ok(Self {
            asset_address: ticker.base_asset_address.parse()?,
            sub_id: ticker.base_asset_sub_id.parse::<u128>()?.into(),
            limit_price: decimal_to_i256(limit_price)?,
            amount: decimal_to_i256(amount)?,
            max_fee: decimal_to_u256(ticker.get_max_fee())?,
            subaccount_id: subaccount_id.into(),
            is_bid,
        })
    }
}

impl ModuleData for TradeData {
    fn address(&self) -> Address {
        let addr = std::env::var("TRADE_ADDRESS").expect("TRADE_ADDRESS must be set");
        addr.parse().expect("TRADE_ADDRESS must be a valid module address")
    }
}

pub fn new_order_params(
    signer: &LocalWallet,
    ticker: &InstrumentTicker,
    subaccount_id: i64,
    args: OrderArgs,
) -> Result<OrderParams> {
    let trade_data = TradeData::new(
        ticker,
        subaccount_id,
        args.limit_price.clone(),
        args.amount.clone(),
        args.direction.is_bid(),
    )?;
    let order_action = ActionData::new(trade_data, subaccount_id, signer.address())?;
    order_action.to_order_params(signer, ticker, subaccount_id, args)
}

pub fn new_replace_params(
    signer: &LocalWallet,
    ticker: &InstrumentTicker,
    subaccount_id: i64,
    order_id_to_cancel: Uuid,
    args: OrderArgs,
) -> Result<ReplaceParams> {
    let trade_data = TradeData::new(
        ticker,
        subaccount_id,
        args.limit_price.clone(),
        args.amount.clone(),
        args.direction.is_bid(),
    )?;
    let order_action = ActionData::new(trade_data, subaccount_id, signer.address())?;
    order_action.to_replace_params(signer, ticker, subaccount_id, order_id_to_cancel, args)
}

impl ActionData {
    pub fn to_order_params(
        self,
        signer: &LocalWallet,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        args: OrderArgs,
    ) -> Result<OrderParams> {
        Ok(OrderParams {
            instrument_name: ticker.instrument_name.clone(),
            subaccount_id,
            amount: args.amount,
            limit_price: args.limit_price,
            direction: args.direction,
            time_in_force: args.time_in_force,
            order_type: args.order_type,
            mmp: args.mmp,
            max_fee: ticker.get_max_fee(),
            label: args.label,
            nonce: self.nonce.as_u64() as i64,
            reject_timestamp: (chrono::Utc::now() + chrono::Duration::seconds(5))
                .timestamp_millis(),
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            reduce_only: false,
            replaced_order_id: None,
            referral_code: "".to_string(),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }

    pub fn to_replace_params(
        self,
        signer: &LocalWallet,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        order_id_to_cancel: Uuid,
        args: OrderArgs,
    ) -> Result<ReplaceParams> {
        Ok(ReplaceParams {
            instrument_name: ticker.instrument_name.clone(),
            subaccount_id,
            amount: args.amount,
            limit_price: args.limit_price,
            direction: args.direction,
            time_in_force: args.time_in_force,
            order_type: args.order_type,
            mmp: args.mmp,
            max_fee: ticker.get_max_fee(),
            label: args.label,
            nonce: self.nonce.as_u64() as i64,
            reject_timestamp: (chrono::Utc::now() + chrono::Duration::seconds(5))
                .timestamp_millis(),
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            reduce_only: false,
            replaced_order_id: None,
            referral_code: "".to_string(),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
            expected_filled_amount: None,
            nonce_to_cancel: None,
            order_id_to_cancel: Some(order_id_to_cancel),
        })
    }
}
