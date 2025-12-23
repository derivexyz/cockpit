pub use crate::types::rfqs::enums::{
    CancelReason, Direction, LiquidityRole, OrderStatus, TxStatus,
};
pub use crate::types::rfqs::params::LegPriced;
pub use crate::types::rfqs::LegUnpriced;
pub use crate::types::shared::PaginationInfoSchema;
use crate::types::RPCId;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RFQResultPrivate {
    pub rfq_id: uuid::Uuid,
    #[serde(default)]
    pub label: String,
    pub legs: Vec<LegUnpriced>,
    ///An optional max total cost for the RFQ. Only used when the RFQ sender executes as buyer. Polling endpoints and channels will ignore quotes where the total cost across all legs is above this value. Positive values mean the RFQ sender expects to pay $, negative mean the RFQ sender expects to receive $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<bigdecimal::BigDecimal>,
    ///An optional min total cost for the RFQ. Only used when the RFQ sender executes as seller. Polling endpoints and channels will ignore quotes where the total cost across all legs is below this value. Positive values mean the RFQ sender expects to receive $, negative mean the RFQ sender expects to pay $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<bigdecimal::BigDecimal>,
    pub subaccount_id: i64,
    pub status: OrderStatus,
    pub cancel_reason: CancelReason,
    pub creation_timestamp: i64,
    pub last_update_timestamp: i64,
    pub valid_until: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filled_direction: Option<Direction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_cost: Option<bigdecimal::BigDecimal>,
}

impl From<&RFQResultPrivate> for RFQResultPrivate {
    fn from(value: &RFQResultPrivate) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RFQResponsePrivate {
    pub id: RPCId,
    pub result: RFQResultPrivate,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRFQsResult {
    pub rfqs: Vec<RFQResultPrivate>,
    pub pagination: PaginationInfoSchema,
}

impl From<&GetRFQsResult> for GetRFQsResult {
    fn from(value: &GetRFQsResult) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetRFQsResponse {
    pub id: RPCId,
    pub result: GetRFQsResult,
}

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

impl QuoteResultPublic {
    pub fn total_cost(&self) -> BigDecimal {
        self.legs
            .iter()
            .map(|leg| leg.direction.sign() * &leg.amount * &leg.price)
            .sum::<BigDecimal>()
            * self.direction.sign()
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfqResultPublicSchema {
    ///Cancel reason, if any
    pub cancel_reason: CancelReason,
    ///Creation timestamp in ms since Unix epoch
    pub creation_timestamp: i64,
    ///Last update timestamp in ms since Unix epoch
    pub last_update_timestamp: i64,
    ///RFQ legs
    pub legs: Vec<LegUnpriced>,
    ///RFQ ID
    pub rfq_id: uuid::Uuid,
    ///Status
    pub status: OrderStatus,
    ///Subaccount ID
    pub subaccount_id: i64,
    ///RFQ expiry timestamp in ms since Unix epoch
    pub valid_until: i64,
    ///Direction at which the RFQ was filled (only if filled)
    pub filled_direction: Option<Direction>,
    ///Total cost for the RFQ (only if filled)
    pub total_cost: Option<bigdecimal::BigDecimal>,
    ///Step size for partial fills (default: 1)
    pub partial_fill_step: bigdecimal::BigDecimal,
    ///Percentage of the RFQ that has been filled, from 0 to 1.
    pub filled_pct: bigdecimal::BigDecimal,
    ///Average taker fill rate, from 0 to 1. Returns null for users with insufficient RFQ history.
    pub fill_rate: Option<bigdecimal::BigDecimal>,
    ///Taker fill rate, weighted towards the recent several days of activity, from 0 to 1. Returns null for users with insufficient recent RFQ history.
    pub recent_fill_rate: Option<bigdecimal::BigDecimal>,
}

impl From<&RfqResultPublicSchema> for RfqResultPublicSchema {
    fn from(value: &RfqResultPublicSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletRfqsNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<RfqResultPublicSchema>,
}
impl From<&WalletRfqsNotificationParamsSchema> for WalletRfqsNotificationParamsSchema {
    fn from(value: &WalletRfqsNotificationParamsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WalletRfqsNotificationSchema {
    pub method: String,
    pub params: WalletRfqsNotificationParamsSchema,
}
impl From<&WalletRfqsNotificationSchema> for WalletRfqsNotificationSchema {
    fn from(value: &WalletRfqsNotificationSchema) -> Self {
        value.clone()
    }
}
