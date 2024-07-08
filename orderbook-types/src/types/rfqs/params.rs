pub use crate::types::rfqs::enums::Direction;
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
