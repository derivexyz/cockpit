pub use crate::types::rfqs::enums::Direction;
pub use crate::types::rfqs::enums::OrderStatus;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegUnpriced {
    ///Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Leg direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
}
impl From<&LegUnpriced> for LegUnpriced {
    fn from(value: &LegUnpriced) -> Self {
        value.clone()
    }
}
impl LegUnpriced {
    pub fn sort(mut legs: Vec<Self>) {
        legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
    }
    pub fn signed_amount(&self) -> bigdecimal::BigDecimal {
        self.direction.sign() * &self.amount
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegPriced {
    ///Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    ///Leg direction
    pub direction: Direction,
    ///Instrument name
    pub instrument_name: String,
    ///Leg price
    pub price: bigdecimal::BigDecimal,
}
impl From<&LegPriced> for LegPriced {
    fn from(value: &LegPriced) -> Self {
        value.clone()
    }
}
impl LegPriced {
    pub fn sort(mut legs: Vec<Self>) {
        legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
    }
    pub fn signed_amount(&self) -> bigdecimal::BigDecimal {
        self.direction.sign() * &self.amount
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfqParams {
    ///Optional user-defined label for the RFQ
    #[serde(default)]
    pub label: String,
    ///RFQ legs
    pub legs: Vec<LegUnpriced>,
    ///An optional max total cost for the RFQ. Only used when the RFQ sender executes as buyer. Polling endpoints and channels will ignore quotes where the total cost across all legs is above this value. Positive values mean the RFQ sender expects to pay $, negative mean the RFQ sender expects to receive $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<bigdecimal::BigDecimal>,
    ///An optional min total cost for the RFQ. Only used when the RFQ sender executes as seller. Polling endpoints and channels will ignore quotes where the total cost across all legs is below this value. Positive values mean the RFQ sender expects to receive $, negative mean the RFQ sender expects to pay $.This field is not disclosed to the market makers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<bigdecimal::BigDecimal>,
    ///Subaccount ID
    pub subaccount_id: i64,
}

impl From<&RfqParams> for RfqParams {
    fn from(value: &RfqParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteParams {
    ///Quote direction, `buy` means trading each leg at its direction, `sell` means trading each leg in the opposite direction.
    pub direction: Direction,
    ///Optional user-defined label for the quote
    #[serde(default)]
    pub label: String,
    ///Quote legs
    pub legs: Vec<LegPriced>,
    ///Max fee ($ for the full trade). Request will be rejected if the supplied max fee is below the estimated fee for this trade.
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the quote is tagged for market maker protections (default false)
    #[serde(default)]
    pub mmp: bool,
    ///Unique nonce defined as a concatenated `UTC timestamp in ms` and `random number up to 6 digits` (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///RFQ ID the quote is for
    pub rfq_id: uuid::Uuid,
    ///Ethereum signature of the quote
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be at least 310 seconds from now. Once time till signature expiry reaches 300 seconds, the quote will be considered expired. This buffer is meant to ensure the trade can settle on chain in case of a blockchain congestion.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed the quote
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}

impl From<&QuoteParams> for QuoteParams {
    fn from(value: &QuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecuteQuoteParams {
    ///Quote direction, `buy` means trading each leg at its direction, `sell` means trading each leg in the opposite direction.
    pub direction: Direction,
    ///Optional user-defined label for the quote
    #[serde(default)]
    pub label: String,
    ///Quote legs
    pub legs: Vec<LegPriced>,
    ///Max fee ($ for the full trade). Request will be rejected if the supplied max fee is below the estimated fee for this trade.
    pub max_fee: bigdecimal::BigDecimal,
    ///Unique nonce defined as a concatenated `UTC timestamp in ms` and `random number up to 6 digits` (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Quote ID to execute against
    pub quote_id: uuid::Uuid,
    ///RFQ ID to execute (must be sent by `subaccount_id`)
    pub rfq_id: uuid::Uuid,
    ///Ethereum signature of the quote
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be at least 310 seconds from now. Once time till signature expiry reaches 300 seconds, the quote will be considered expired. This buffer is meant to ensure the trade can settle on chain in case of a blockchain congestion.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed the quote
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&ExecuteQuoteParams> for ExecuteQuoteParams {
    fn from(value: &ExecuteQuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceQuoteParams {
    ///Quote direction, `buy` means trading each leg at its direction, `sell` means trading each leg in the opposite direction.
    pub direction: Direction,
    ///Optional user-defined label for the quote
    #[serde(default)]
    pub label: String,
    ///Quote legs
    pub legs: Vec<LegPriced>,
    ///Max fee ($ for the full trade). Request will be rejected if the supplied max fee is below the estimated fee for this trade.
    pub max_fee: bigdecimal::BigDecimal,
    ///Whether the quote is tagged for market maker protections (default false)
    #[serde(default)]
    pub mmp: bool,
    ///Unique nonce defined as a concatenated `UTC timestamp in ms` and `random number up to 6 digits` (e.g. 1695836058725001, where 001 is the random number)
    pub nonce: i64,
    ///Cancel quote by nonce (choose either quote_id or nonce).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce_to_cancel: Option<i64>,
    ///Cancel quote by quote_id (choose either quote_id or nonce).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id_to_cancel: Option<uuid::Uuid>,
    ///RFQ ID the quote is for
    pub rfq_id: uuid::Uuid,
    ///Ethereum signature of the quote
    pub signature: String,
    ///Unix timestamp in seconds. Expiry MUST be at least 310 seconds from now.
    pub signature_expiry_sec: i64,
    ///Owner wallet address or registered session key that signed the quote
    pub signer: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}

impl From<&ReplaceQuoteParams> for ReplaceQuoteParams {
    fn from(value: &ReplaceQuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollRfqsParams {
    /// Earliest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provided, defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    /// Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of results per page (default 100, max 1000)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// RFQ ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    /// Filter returned RFQs by rfq requestor subaccount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_subaccount_id: Option<i64>,
    /// RFQ status filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// Subaccount ID for auth purposes, returned data will be scoped to this subaccount.
    pub subaccount_id: i64,
    /// Latest `last_update_timestamp` to filter by (in ms since Unix epoch). If not provided, defaults to returning all data up to current time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<i64>,
}

impl From<&PollRfqsParams> for PollRfqsParams {
    fn from(value: &PollRfqsParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetQuotesParams {
    /// Earliest timestamp to filter by (in ms since Unix epoch). If not provided, defaults to 0.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    /// Page number of results to return (default 1, returns last if above `num_pages`)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Number of results per page (default 100, max 1000)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Quote ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    /// RFQ ID filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    /// Quote status filter, if applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<OrderStatus>,
    /// Subaccount ID for auth purposes, returned data will be scoped to this subaccount.
    pub subaccount_id: i64,
    /// Latest timestamp to filter by (in ms since Unix epoch). If not provided, defaults to returning all data up to current time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<i64>,
}

impl From<&GetQuotesParams> for GetQuotesParams {
    fn from(value: &GetQuotesParams) -> Self {
        value.clone()
    }
}
