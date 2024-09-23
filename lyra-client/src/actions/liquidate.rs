use crate::utils::{decimal_to_i256, decimal_to_u256, decimal_to_u256_with_prec};
use std::str::FromStr;

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

use crate::actions::helpers::ModuleData;
use crate::actions::ActionData;
use orderbook_types::types::liquidations::{AuctionDetailsSchema, LiquidationParams};

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct LiquidateData {
    liquidated_account_id: i64,
    cash_transfer: U256,
    percent_of_acc: U256,
    price_limit: I256,
    last_seen_trade_id: U256,
    merge_account: bool,
}

impl LiquidateData {
    pub fn new(
        subaccount_id: i64,
        pct: BigDecimal,
        price: BigDecimal,
        transfer_amount: BigDecimal,
        trade_id: i64,
    ) -> Result<Self> {
        Ok(Self {
            liquidated_account_id: subaccount_id,
            cash_transfer: decimal_to_u256(transfer_amount)?,
            percent_of_acc: decimal_to_u256_with_prec(pct, 18)?,
            price_limit: decimal_to_i256(price)?,
            last_seen_trade_id: trade_id.into(),
            merge_account: true,
        })
    }
}
impl ModuleData for LiquidateData {
    fn address(&self) -> Address {
        let addr = std::env::var("LIQUIDATE_ADDRESS").expect("LIQUIDATE_ADDRESS must be set");
        addr.parse().expect("LIQUIDATE_ADDRESS must be a valid module address")
    }
}

impl ActionData {
    pub fn to_liquidate_params(
        self,
        signer: &LocalWallet,
        liquidated_id: i64,
        percent_bid: BigDecimal,
        details: &AuctionDetailsSchema,
    ) -> Result<LiquidationParams> {
        Ok(LiquidationParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            liquidated_subaccount_id: liquidated_id,
            cash_transfer: details.cash_transfer_with_buffer(),
            price_limit: details.price_limit_with_buffer(),
            percent_bid,
            last_seen_trade_id: details.last_seen_trade_id,
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }
}

pub fn new_liquidate_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    liquidated_id: i64,
    percent_bid: BigDecimal,
    details: &AuctionDetailsSchema,
) -> Result<LiquidationParams> {
    let liquidate_data = LiquidateData::new(
        liquidated_id,
        percent_bid.clone(),
        details.price_limit_with_buffer(),
        details.cash_transfer_with_buffer(),
        details.last_seen_trade_id,
    )?;
    let action_data = ActionData::new(liquidate_data, subaccount_id, signer.address())?;
    let params = action_data.to_liquidate_params(signer, liquidated_id, percent_bid, details)?;
    Ok(params)
}
