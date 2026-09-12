use crate::types::shared::{PaginationInfoSchema, RPCId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultConfig {
    pub cooldown_sec: i64,
    pub deposit_spot_asset: String,
    pub management_fee_bps: i64,
    pub max_slippage_bps: i64,
    pub performance_fee_bps: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_asset: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolVault {
    pub closed: bool,
    pub config: VaultConfig,
    pub global_hwm: bigdecimal::BigDecimal,
    pub last_fee_settled_at_sec: i64,
    pub protocol_fee_share_bps: i64,
    pub subaccount_id: i64,
    pub total_shares: bigdecimal::BigDecimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Vault {
    pub curator: String,
    pub curator_shares: bigdecimal::BigDecimal,
    pub description: String,
    pub name: String,
    pub protocol: ProtocolVault,
    pub whitelist_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_price: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mtm_cap: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav_benchmark: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav_usd: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub simulated_share_price_usd: Option<bigdecimal::BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultResponse {
    pub id: RPCId,
    pub result: Vault,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultsResult {
    pub pagination: PaginationInfoSchema,
    pub vaults: Vec<Vault>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultsResponse {
    pub id: RPCId,
    pub result: VaultsResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicVaultAction {
    pub curator_shares_minted: bigdecimal::BigDecimal,
    pub event_ts: i64,
    pub event_type: String,
    pub holder: String,
    pub management_shares_minted: bigdecimal::BigDecimal,
    pub nav: bigdecimal::BigDecimal,
    pub new_high_water_mark: bigdecimal::BigDecimal,
    pub old_high_water_mark: bigdecimal::BigDecimal,
    pub operation_uuid: String,
    pub performance_shares_minted: bigdecimal::BigDecimal,
    pub protocol_shares_minted: bigdecimal::BigDecimal,
    pub share_price: bigdecimal::BigDecimal,
    pub shares_delta: bigdecimal::BigDecimal,
    pub status: String,
    pub subaccount_id: i64,
    pub total_shares: bigdecimal::BigDecimal,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginatedVaultActionHistory {
    pub events: Vec<PublicVaultAction>,
    pub pagination: PaginationInfoSchema,
    pub subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginatedVaultActionHistoryResponse {
    pub id: RPCId,
    pub result: PaginatedVaultActionHistory,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultPerformancePoint {
    pub curator_shares: bigdecimal::BigDecimal,
    pub global_hwm: bigdecimal::BigDecimal,
    pub share_price: bigdecimal::BigDecimal,
    pub total_shares: bigdecimal::BigDecimal,
    pub ts: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benchmark_price: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav_benchmark: Option<bigdecimal::BigDecimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultPerformanceHistoryResult {
    pub points: Vec<VaultPerformancePoint>,
    pub resolution: crate::types::vaults::enums::PerformanceResolution,
    pub subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultPerformanceHistoryResponse {
    pub id: RPCId,
    pub result: VaultPerformanceHistoryResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultIdsResult {
    pub subaccount_ids: Vec<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultIdsResponse {
    pub id: RPCId,
    pub result: VaultIdsResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultShareEntry {
    pub shares: bigdecimal::BigDecimal,
    pub vault: Vault,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultSharesResult {
    pub vaults: Vec<VaultShareEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultSharesResponse {
    pub id: RPCId,
    pub result: VaultSharesResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultRequestId {
    pub vault_nonce: String,
    pub vault_subaccount_id: i64,
    pub wallet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedActionPayload {
    pub data: Vec<u8>,
    pub expiry: i64,
    pub module: String,
    pub nonce: i64,
    pub owner: String,
    pub signer: String,
    pub subaccount_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignedAction {
    pub action: SignedActionPayload,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultRequest {
    pub creation_timestamp_ms: i64,
    pub id: VaultRequestId,
    pub signed_action: SignedAction,
    pub subaccount_id: i64,
    pub user_action_hash: String,
    pub wallet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultipleVaultRequestsResult {
    pub requests: Vec<VaultRequest>,
    pub total: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultipleVaultRequestsResponse {
    pub id: RPCId,
    pub result: MultipleVaultRequestsResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultActionHistoryEntry {
    pub after_shares: bigdecimal::BigDecimal,
    pub amount: bigdecimal::BigDecimal,
    pub before_shares: bigdecimal::BigDecimal,
    pub creation_timestamp_ms: i64,
    pub entry_price: bigdecimal::BigDecimal,
    pub error_reason: String,
    pub event_ts: i64,
    pub event_type: String,
    pub exit_price: bigdecimal::BigDecimal,
    pub operation_id: i64,
    pub operation_uuid: String,
    pub share_price: bigdecimal::BigDecimal,
    pub shares_delta: bigdecimal::BigDecimal,
    pub shares_requested: bigdecimal::BigDecimal,
    pub status: String,
    pub user_action_hash: String,
    pub vault_nonce: String,
    pub vault_subaccount_id: i64,
    pub wallet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginatedVaultRequestHistory {
    pub actions: Vec<VaultActionHistoryEntry>,
    pub pagination: PaginationInfoSchema,
    pub wallet: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginatedVaultRequestHistoryResponse {
    pub id: RPCId,
    pub result: PaginatedVaultRequestHistory,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultRequestAck {
    pub request_id: VaultRequestId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultRequestAckResponse {
    pub id: RPCId,
    pub result: VaultRequestAck,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultOpResult {
    pub op_uuid: String,
    pub operation_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultOpResponse {
    pub id: RPCId,
    pub result: VaultOpResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultCancelResult {
    pub cancelled_request_ids: Vec<VaultRequestId>,
    pub op_uuid: String,
    pub operation_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VaultCancelResponse {
    pub id: RPCId,
    pub result: VaultCancelResult,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OffchainAck {
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OffchainAckResponse {
    pub id: RPCId,
    pub result: OffchainAck,
}
