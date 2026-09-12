use crate::types::liquidations::enums::MarginType;
use crate::types::liquidations::{AuctionState, AuctionType};
use crate::types::RPCId;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// `auctions.watch` notification params.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionsWatchNotificationParamsSchema {
    /// Subscribed channel name
    pub channel: String,
    pub data: Vec<AuctionsWatchResultSchema>,
}

impl From<&AuctionsWatchNotificationParamsSchema> for AuctionsWatchNotificationParamsSchema {
    fn from(value: &AuctionsWatchNotificationParamsSchema) -> Self {
        value.clone()
    }
}

/// `auctions.watch` notification envelope.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionsWatchNotificationSchema {
    pub method: String,
    pub params: AuctionsWatchNotificationParamsSchema,
}

impl From<&AuctionsWatchNotificationSchema> for AuctionsWatchNotificationSchema {
    fn from(value: &AuctionsWatchNotificationSchema) -> Self {
        value.clone()
    }
}

/// `AuctionDetails` on `auctions.watch`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionDetailsSchema {
    /// Currency of subaccount
    pub currency: Option<String>,
    /// Estimated bid price
    pub estimated_bid_price: BigDecimal,
    /// Estimated discount pnl
    pub estimated_discount_pnl: BigDecimal,
    /// Estimated mark-to-market
    pub estimated_mtm: BigDecimal,
    /// Estimated percent bid
    pub estimated_percent_bid: BigDecimal,
    /// Margin type of subaccount (`PM` or `SM`)
    pub margin_type: MarginType,
    /// Minimum price limit
    pub min_price_limit: BigDecimal,
    /// Subaccount balances (asset name → decimal)
    pub subaccount_balances: std::collections::HashMap<String, BigDecimal>,
}

impl AuctionDetailsSchema {
    pub fn buffer(&self) -> BigDecimal {
        let buffer = self.min_price_limit.abs() * BigDecimal::from_str("0.05").unwrap();
        buffer.max(BigDecimal::from_str("10").unwrap())
    }

    pub fn price_limit_with_buffer(&self) -> BigDecimal {
        &self.estimated_bid_price + self.buffer()
    }
}

/// Payload for `auctions.watch`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionsWatchResultSchema {
    pub details: Option<AuctionDetailsSchema>,
    pub state: AuctionState,
    pub subaccount_id: i64,
    pub timestamp: i64,
}

impl From<&AuctionsWatchResultSchema> for AuctionsWatchResultSchema {
    fn from(value: &AuctionsWatchResultSchema) -> Self {
        value.clone()
    }
}

pub type AuctionsWatchData = Vec<AuctionsWatchResultSchema>;

/// Result of `private/liquidate`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendLiquidateResult {
    pub op_uuid: String,
    pub operation_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendLiquidateResponse {
    pub id: RPCId,
    pub result: SendLiquidateResult,
}

/// Result of `public/start_auction`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartAuctionResult {
    pub op_uuid: String,
    pub operation_id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartAuctionResponse {
    pub id: RPCId,
    pub result: StartAuctionResult,
}

/// One executed bid within an auction (`public/get_liquidation_history`).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionBidEvent {
    pub amounts_liquidated: std::collections::HashMap<String, BigDecimal>,
    pub cash_received: BigDecimal,
    pub discount_pnl: BigDecimal,
    pub percent_liquidated: BigDecimal,
    pub positions_realized_pnl: std::collections::HashMap<String, BigDecimal>,
    pub positions_realized_pnl_excl_fees: std::collections::HashMap<String, BigDecimal>,
    pub realized_pnl: BigDecimal,
    pub realized_pnl_excl_fees: BigDecimal,
    pub timestamp: i64,
    pub tx_hash: String,
}

/// One auction phase and the bids that filled it.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuctionHistory {
    pub auction_id: String,
    pub auction_type: AuctionType,
    pub bids: Vec<AuctionBidEvent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<i64>,
    pub fee: BigDecimal,
    pub start_timestamp: i64,
    pub subaccount_id: i64,
    pub tx_hash: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Pagination {
    pub count: u64,
    pub num_pages: u64,
}

/// Result of `public/get_liquidation_history`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiquidationHistoryResult {
    pub auctions: Vec<AuctionHistory>,
    pub pagination: Pagination,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetLiquidationHistoryResponse {
    pub id: RPCId,
    pub result: LiquidationHistoryResult,
}
