use crate::utils::{decimal_to_i256, decimal_to_u256_with_prec};

use anyhow::Result;
use bigdecimal::BigDecimal;
pub use derive_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, EthAbiCodec, EthAbiType, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;

use crate::actions::helpers::ModuleData;
use crate::actions::ActionData;
use derive_types::types::liquidations::{AuctionDetailsSchema, LiquidationParams};

/// ABI payload for the liquidate module: account id, percent (1e18), price limit.
#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct LiquidateData {
    liquidated_account_id: i64,
    percent_of_acc: U256,
    price_limit: I256,
}

impl LiquidateData {
    pub fn new(subaccount_id: i64, pct: BigDecimal, price: BigDecimal) -> Result<Self> {
        Ok(Self {
            liquidated_account_id: subaccount_id,
            percent_of_acc: decimal_to_u256_with_prec(pct, 18)?,
            price_limit: decimal_to_i256(price)?,
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
        price_limit: BigDecimal,
    ) -> Result<LiquidationParams> {
        Ok(LiquidationParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            liquidate_subaccount_id: liquidated_id,
            price_limit,
            percent_of_acc: percent_bid,
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
    new_liquidate_params_with_price(
        signer,
        subaccount_id,
        liquidated_id,
        percent_bid,
        details.price_limit_with_buffer(),
    )
}

pub fn new_liquidate_params_with_price(
    signer: &LocalWallet,
    subaccount_id: i64,
    liquidated_id: i64,
    percent_bid: BigDecimal,
    price_limit: BigDecimal,
) -> Result<LiquidationParams> {
    let liquidate_data = LiquidateData::new(liquidated_id, percent_bid.clone(), price_limit.clone())?;
    let action_data = ActionData::new(liquidate_data, subaccount_id, signer.address())?;
    action_data.to_liquidate_params(signer, liquidated_id, percent_bid, price_limit)
}
