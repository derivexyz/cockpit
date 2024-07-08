pub use crate::types::rfqs::enums::{
    CancelReason, Direction, LiquidityRole, OrderStatus, TxStatus,
};
pub use crate::types::rfqs::params::LegPriced;
pub use crate::types::shared::PaginationInfoSchema;
use crate::types::tickers::InstrumentTicker;
use crate::types::RPCId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteResultPublic {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///Quote direction
    pub direction: Direction,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///Quote legs
    pub legs: Vec<LegPriced>,
    ///Hash of the legs of the best quote to be signed by the taker.
    pub legs_hash: String,
    ///Liquidity role
    pub liquidity_role: LiquidityRole,
    ///Quote ID
    pub quote_id: uuid::Uuid,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Status
    pub status: OrderStatus,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///Blockchain transaction hash (only for executed quotes)
    pub tx_hash: Option<String>,
    ///Blockchain transaction status (only for executed quotes)
    pub tx_status: Option<TxStatus>,
    ///Wallet of the sender
    pub wallet: String,
}
impl From<&QuoteResultPublic> for QuoteResultPublic {
    fn from(value: &QuoteResultPublic) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollQuotesResult {
    ///Pagination info
    pub pagination: PaginationInfoSchema,
    ///Quotes matching filter criteria
    pub quotes: Vec<QuoteResultPublic>,
}
impl From<&PollQuotesResult> for PollQuotesResult {
    fn from(value: &PollQuotesResult) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollQuotesResponse {
    pub id: RPCId,
    pub result: PollQuotesResult,
}
