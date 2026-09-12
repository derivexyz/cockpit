use crate::types::shared::serde_nonce;
use crate::types::vaults::enums::PerformanceResolution;
use crate::types::vaults::VaultRequestId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetVaultParams {
    pub subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetVaultsParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetVaultActionHistoryParams {
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetVaultPerformanceHistoryParams {
    pub subaccount_id: i64,
    pub resolution: PerformanceResolution,
    #[serde(rename = "from", default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    #[serde(rename = "to", default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletVaultParams {
    pub wallet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetVaultRequestHistoryParams {
    pub wallet: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLiveQueueParams {
    pub subaccount_id: i64,
    pub limit: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestVaultDepositParams {
    pub amount: bigdecimal::BigDecimal,
    pub deposit_spot_asset: String,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    pub vault_subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RequestVaultWithdrawParams {
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub shares_to_burn: bigdecimal::BigDecimal,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    pub vault_subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CancelAllVaultRequestsParams {
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    pub vault_subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateVaultParams {
    pub cooldown_sec: i64,
    pub deposit_spot_asset: String,
    pub initial_deposit: bigdecimal::BigDecimal,
    pub initial_share_price_usd: bigdecimal::BigDecimal,
    pub management_fee_bps: i64,
    pub manager_id: i64,
    pub max_fee_usd: bigdecimal::BigDecimal,
    pub max_slippage_bps: i64,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub performance_fee_bps: i64,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_asset: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MintSharesParams {
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub request_id: VaultRequestId,
    pub share_price: bigdecimal::BigDecimal,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    pub deposit_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BurnSharesParams {
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub request_id: VaultRequestId,
    pub share_price: bigdecimal::BigDecimal,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    pub withdraw_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateVaultInfoParams {
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtm_cap: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whitelist_only: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RejectDepositRequestParams {
    pub request_id: VaultRequestId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForceBurnParams {
    pub holder: String,
    pub subaccount_id: i64,
}
