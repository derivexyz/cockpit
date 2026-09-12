use crate::actions::helpers::ModuleData;
use crate::actions::ActionData;
use crate::utils::decimal_to_u256;
use anyhow::{anyhow, Result};
use bigdecimal::BigDecimal;
use derive_types::types::vaults::{
    BurnSharesParams, CancelAllVaultRequestsParams, CreateVaultParams, MintSharesParams,
    RequestVaultDepositParams, RequestVaultWithdrawParams, VaultAction, VaultRequestId,
};
use ethers::abi::{AbiEncode};
use ethers::prelude::{Address, EthAbiCodec, EthAbiType, LocalWallet, Signer, U256};
use ethers::utils::hex;

pub const INTENT_SIGNATURE_TTL_SEC: i64 = 7 * 24 * 60 * 60;
const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";

fn vault_module_address() -> Address {
    let addr = std::env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS must be set");
    addr.parse().expect("VAULT_ADDRESS must be a valid module address")
}

fn parse_bytes32(value: &str, field: &str) -> Result<[u8; 32]> {
    let stripped = value.strip_prefix("0x").unwrap_or(value);
    let bytes = hex::decode(stripped).map_err(|e| anyhow!("{field}: {e}"))?;
    bytes
        .try_into()
        .map_err(|_| anyhow!("{field} must be 32 bytes"))
}

#[derive(Clone, Debug, PartialEq, EthAbiType, EthAbiCodec)]
pub struct VaultCreateData {
    kind: U256,
    manager_id: U256,
    deposit_spot_asset: Address,
    initial_deposit: U256,
    management_fee_bps: U256,
    performance_fee_bps: U256,
    max_slippage_bps: U256,
    cooldown_sec: U256,
    max_fee_usd: U256,
    initial_share_price_usd: U256,
    benchmark_asset: Address,
    has_benchmark: bool,
}

impl VaultCreateData {
    pub fn new(
        manager_id: i64,
        deposit_spot_asset: &str,
        initial_deposit: BigDecimal,
        management_fee_bps: i64,
        performance_fee_bps: i64,
        max_slippage_bps: i64,
        cooldown_sec: i64,
        max_fee_usd: BigDecimal,
        initial_share_price_usd: BigDecimal,
        benchmark_asset: Option<&str>,
    ) -> Result<Self> {
        Ok(Self {
            kind: U256::from(VaultAction::Create as u8),
            manager_id: manager_id.into(),
            deposit_spot_asset: deposit_spot_asset.parse()?,
            initial_deposit: decimal_to_u256(initial_deposit)?,
            management_fee_bps: management_fee_bps.into(),
            performance_fee_bps: performance_fee_bps.into(),
            max_slippage_bps: max_slippage_bps.into(),
            cooldown_sec: cooldown_sec.into(),
            max_fee_usd: decimal_to_u256(max_fee_usd)?,
            initial_share_price_usd: decimal_to_u256(initial_share_price_usd)?,
            benchmark_asset: benchmark_asset.unwrap_or(ZERO_ADDRESS).parse()?,
            has_benchmark: benchmark_asset.is_some(),
        })
    }
}

impl ModuleData for VaultCreateData {
    fn address(&self) -> Address {
        vault_module_address()
    }
}

#[derive(Clone, Debug, PartialEq, EthAbiType, EthAbiCodec)]
pub struct VaultDepositData {
    kind: U256,
    vault_subaccount_id: U256,
    deposit_spot_asset: Address,
    amount: U256,
}

impl VaultDepositData {
    pub fn new(vault_subaccount_id: i64, deposit_spot_asset: &str, amount: BigDecimal) -> Result<Self> {
        Ok(Self {
            kind: U256::from(VaultAction::Deposit as u8),
            vault_subaccount_id: vault_subaccount_id.into(),
            deposit_spot_asset: deposit_spot_asset.parse()?,
            amount: decimal_to_u256(amount)?,
        })
    }
}

impl ModuleData for VaultDepositData {
    fn address(&self) -> Address {
        vault_module_address()
    }
}

#[derive(Clone, Debug, PartialEq, EthAbiType, EthAbiCodec)]
pub struct VaultWithdrawData {
    kind: U256,
    vault_subaccount_id: U256,
    shares_to_burn: U256,
}

impl VaultWithdrawData {
    pub fn new(vault_subaccount_id: i64, shares_to_burn: BigDecimal) -> Result<Self> {
        Ok(Self {
            kind: U256::from(VaultAction::Withdraw as u8),
            vault_subaccount_id: vault_subaccount_id.into(),
            shares_to_burn: decimal_to_u256(shares_to_burn)?,
        })
    }
}

impl ModuleData for VaultWithdrawData {
    fn address(&self) -> Address {
        vault_module_address()
    }
}

#[derive(Clone, Debug, PartialEq, EthAbiType, EthAbiCodec)]
pub struct VaultCancelData {
    kind: U256,
    vault_subaccount_id: U256,
}

impl VaultCancelData {
    pub fn new(vault_subaccount_id: i64) -> Self {
        Self {
            kind: U256::from(VaultAction::Cancel as u8),
            vault_subaccount_id: vault_subaccount_id.into(),
        }
    }
}

impl ModuleData for VaultCancelData {
    fn address(&self) -> Address {
        vault_module_address()
    }
}

#[derive(Clone, Debug, PartialEq, EthAbiType, EthAbiCodec)]
pub struct VaultSettleData {
    kind: U256,
    share_price: U256,
    user_action_hash: [u8; 32],
}

impl VaultSettleData {
    pub fn mint(share_price: BigDecimal, deposit_hash: &str) -> Result<Self> {
        Ok(Self {
            kind: U256::from(VaultAction::MintShares as u8),
            share_price: decimal_to_u256(share_price)?,
            user_action_hash: parse_bytes32(deposit_hash, "deposit_hash")?,
        })
    }
    pub fn burn(share_price: BigDecimal, withdraw_hash: &str) -> Result<Self> {
        Ok(Self {
            kind: U256::from(VaultAction::BurnShares as u8),
            share_price: decimal_to_u256(share_price)?,
            user_action_hash: parse_bytes32(withdraw_hash, "withdraw_hash")?,
        })
    }
}

impl ModuleData for VaultSettleData {
    fn address(&self) -> Address {
        vault_module_address()
    }
}

fn intent_expiry() -> i64 {
    (chrono::Utc::now() + chrono::Duration::seconds(INTENT_SIGNATURE_TTL_SEC)).timestamp()
}

fn sign_vault_action(
    signer: &LocalWallet,
    action: ActionData,
) -> Result<(i64, i64, String, String)> {
    let nonce = action.nonce.as_u64() as i64;
    let signature_expiry_sec = action.expiry.as_u64() as i64;
    let signer_hex = hex::encode_prefixed(action.signer);
    let signature = signer.sign_hash(action.hash().into())?.to_string();
    Ok((nonce, signature_expiry_sec, signer_hex, signature))
}

pub fn new_vault_deposit_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    vault_subaccount_id: i64,
    deposit_spot_asset: String,
    amount: BigDecimal,
) -> Result<RequestVaultDepositParams> {
    let data = VaultDepositData::new(vault_subaccount_id, &deposit_spot_asset, amount.clone())?;
    let action = ActionData::new_with_expiry(data, subaccount_id, signer.address(), Some(intent_expiry()))?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(RequestVaultDepositParams {
        amount,
        deposit_spot_asset,
        nonce,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id,
        vault_subaccount_id,
    })
}

pub fn new_vault_withdraw_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    vault_subaccount_id: i64,
    shares_to_burn: BigDecimal,
) -> Result<RequestVaultWithdrawParams> {
    let data = VaultWithdrawData::new(vault_subaccount_id, shares_to_burn.clone())?;
    let action = ActionData::new_with_expiry(data, subaccount_id, signer.address(), Some(intent_expiry()))?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(RequestVaultWithdrawParams {
        nonce,
        shares_to_burn,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id,
        vault_subaccount_id,
    })
}

pub fn new_cancel_all_vault_requests_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    vault_subaccount_id: i64,
) -> Result<CancelAllVaultRequestsParams> {
    let data = VaultCancelData::new(vault_subaccount_id);
    let action = ActionData::new_with_expiry(data, subaccount_id, signer.address(), Some(intent_expiry()))?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(CancelAllVaultRequestsParams {
        nonce,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id,
        vault_subaccount_id,
    })
}

pub fn new_create_vault_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    manager_id: i64,
    deposit_spot_asset: String,
    initial_deposit: BigDecimal,
    initial_share_price_usd: BigDecimal,
    management_fee_bps: i64,
    performance_fee_bps: i64,
    max_slippage_bps: i64,
    cooldown_sec: i64,
    max_fee_usd: BigDecimal,
    benchmark_asset: Option<String>,
) -> Result<CreateVaultParams> {
    let data = VaultCreateData::new(
        manager_id,
        &deposit_spot_asset,
        initial_deposit.clone(),
        management_fee_bps,
        performance_fee_bps,
        max_slippage_bps,
        cooldown_sec,
        max_fee_usd.clone(),
        initial_share_price_usd.clone(),
        benchmark_asset.as_deref(),
    )?;
    let action = ActionData::new(data, subaccount_id, signer.address())?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(CreateVaultParams {
        cooldown_sec,
        deposit_spot_asset,
        initial_deposit,
        initial_share_price_usd,
        management_fee_bps,
        manager_id,
        max_fee_usd,
        max_slippage_bps,
        nonce,
        performance_fee_bps,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id,
        benchmark_asset,
    })
}

pub fn new_mint_shares_params(
    signer: &LocalWallet,
    vault_subaccount_id: i64,
    request_id: VaultRequestId,
    share_price: BigDecimal,
    deposit_hash: String,
) -> Result<MintSharesParams> {
    let data = VaultSettleData::mint(share_price.clone(), &deposit_hash)?;
    let action = ActionData::new(data, vault_subaccount_id, signer.address())?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(MintSharesParams {
        nonce,
        request_id,
        share_price,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id: vault_subaccount_id,
        deposit_hash,
    })
}

pub fn new_burn_shares_params(
    signer: &LocalWallet,
    vault_subaccount_id: i64,
    request_id: VaultRequestId,
    share_price: BigDecimal,
    withdraw_hash: String,
) -> Result<BurnSharesParams> {
    let data = VaultSettleData::burn(share_price.clone(), &withdraw_hash)?;
    let action = ActionData::new(data, vault_subaccount_id, signer.address())?;
    let (nonce, signature_expiry_sec, signer_hex, signature) = sign_vault_action(signer, action)?;
    Ok(BurnSharesParams {
        nonce,
        request_id,
        share_price,
        signature,
        signature_expiry_sec,
        signer: signer_hex,
        subaccount_id: vault_subaccount_id,
        withdraw_hash,
    })
}
