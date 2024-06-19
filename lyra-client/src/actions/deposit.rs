use crate::actions::helpers::{
    get_asset_address, get_asset_decimals, get_manager_address, MarginType,
};
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
use serde::Deserialize;

use crate::actions::helpers::ModuleData;
use crate::actions::ActionData;
use orderbook_types::generated::private_deposit::PrivateDepositParamsSchema;

pub type DepositParams = PrivateDepositParamsSchema;

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct DepositData {
    erc20_amount: U256,
    asset_address: Address,
    manager_address: Address,
}

impl DepositData {
    pub fn new(amount: &BigDecimal, asset_name: &String, margin_type: MarginType) -> Result<Self> {
        let asset_address = get_asset_address(&asset_name);
        let asset_decimals = get_asset_decimals(&asset_name);
        let manager_address = get_manager_address(&asset_name, margin_type);
        Ok(DepositData {
            erc20_amount: decimal_to_u256_with_prec(amount.clone(), asset_decimals)?,
            asset_address: asset_address.parse()?,
            manager_address: manager_address.parse()?,
        })
    }
}
impl ModuleData for DepositData {
    fn address(&self) -> Address {
        let addr = std::env::var("DEPOSIT_ADDRESS").expect("DEPOSIT_ADDRESS must be set");
        addr.parse().expect("DEPOSIT_ADDRESS must be a valid module address")
    }
}

impl ActionData {
    pub fn to_deposit_params(
        self,
        signer: &LocalWallet,
        amount: BigDecimal,
        asset_name: String,
    ) -> Result<DepositParams> {
        Ok(DepositParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            amount,
            asset_name,
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }
}

pub fn new_deposit_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    amount: BigDecimal,
    asset_name: String,
    margin_type: MarginType,
) -> Result<DepositParams> {
    let deposit_data = DepositData::new(&amount, &asset_name, margin_type)?;
    let action_data = ActionData::new(deposit_data, subaccount_id, signer.address())?;
    let params = action_data.to_deposit_params(signer, amount, asset_name)?;
    Ok(params)
}
