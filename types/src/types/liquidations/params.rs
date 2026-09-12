use crate::types::shared::serde_nonce;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

/// Parameters for `private/liquidate`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiquidationParams {
    /// Bidder subaccount.
    pub subaccount_id: i64,
    /// Subaccount being liquidated.
    pub liquidate_subaccount_id: i64,
    /// Fraction of the account to liquidate (`"1.0"` = 100%). Must be a whole percent (multiple of `0.01`).
    pub percent_of_acc: BigDecimal,
    /// Signed limit price (`"0"` opts out).
    pub price_limit: BigDecimal,
    /// Unix timestamp in seconds. Signature becomes invalid after this time.
    pub signature_expiry_sec: i64,
    /// Unique nonce: UTC nanoseconds as a decimal string
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    /// Owner wallet address or registered session key that signed the action.
    pub signer: String,
    /// Ethereum signature of the action.
    pub signature: String,
}

/// Parameters for `public/start_auction`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartAuctionParams {
    /// Subaccount to auction.
    pub subaccount_id: i64,
}

/// Parameters for `public/get_liquidation_history`.
/// Omit `subaccount_id` to span every account. Timestamp window is unix ms over auction start.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GetLiquidationHistoryParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_id: Option<i64>,
}
