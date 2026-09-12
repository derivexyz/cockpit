use crate::utils::decimal_to_u256_with_prec;

use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{Address, EthAbiCodec, EthAbiType, LocalWallet, Signer, U256};
use ethers::utils::hex;
use serde::{Deserialize, Serialize};

use crate::actions::helpers::{get_asset_address, get_asset_decimals, ModuleData};
use crate::actions::ActionData;
use derive_types::types::shared::serde_nonce;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WithdrawParams {
    pub subaccount_id: i64,
    pub asset_name: String,
    pub amount_in_underlying: BigDecimal,
    pub max_fee_usd: BigDecimal,
    pub force_batch: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub signature: String,
}

/// ABI: address asset, uint256 maxFeeUsd, address recipient, uint256 amount, bool forceBatch
#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct WithdrawalData {
    asset_address: Address,
    max_fee_usd: U256,
    recipient: Address,
    erc20_amount: U256,
    force_batch: bool,
}

impl WithdrawalData {
    pub fn new(
        amount: &BigDecimal,
        asset_name: &str,
        max_fee_usd: &BigDecimal,
        recipient: &str,
        force_batch: bool,
    ) -> Result<Self> {
        let asset_address = get_asset_address(asset_name);
        let asset_decimals = get_asset_decimals(asset_name);
        Ok(WithdrawalData {
            asset_address: asset_address.parse()?,
            max_fee_usd: decimal_to_u256_with_prec(max_fee_usd.clone(), 18)?,
            recipient: recipient.parse()?,
            erc20_amount: decimal_to_u256_with_prec(amount.clone(), asset_decimals)?,
            force_batch,
        })
    }
}

impl ModuleData for WithdrawalData {
    fn address(&self) -> Address {
        let addr = std::env::var("WITHDRAWAL_ADDRESS").expect("WITHDRAWAL_ADDRESS must be set");
        addr.parse().expect("WITHDRAWAL_ADDRESS must be a valid module address")
    }
}

impl ActionData {
    pub fn to_withdraw_params(
        self,
        signer: &LocalWallet,
        amount: BigDecimal,
        asset_name: String,
        max_fee_usd: BigDecimal,
        force_batch: bool,
        recipient: Option<String>,
    ) -> Result<WithdrawParams> {
        Ok(WithdrawParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            amount_in_underlying: amount,
            asset_name,
            max_fee_usd,
            force_batch,
            recipient,
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }
}

pub fn new_withdraw_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    amount: BigDecimal,
    asset_name: String,
) -> Result<WithdrawParams> {
    new_withdraw_params_full(
        signer,
        subaccount_id,
        amount,
        asset_name,
        BigDecimal::from(1),
        false,
        None,
    )
}

pub fn new_withdraw_params_full(
    signer: &LocalWallet,
    subaccount_id: i64,
    amount: BigDecimal,
    asset_name: String,
    max_fee_usd: BigDecimal,
    force_batch: bool,
    recipient: Option<String>,
) -> Result<WithdrawParams> {
    let recipient_addr = recipient.clone().unwrap_or_else(|| {
        std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY must be set")
    });
    let withdrawal_data =
        WithdrawalData::new(&amount, &asset_name, &max_fee_usd, &recipient_addr, force_batch)?;
    let action_data = ActionData::new(withdrawal_data, subaccount_id, signer.address())?;
    action_data.to_withdraw_params(
        signer,
        amount,
        asset_name,
        max_fee_usd,
        force_batch,
        recipient,
    )
}
