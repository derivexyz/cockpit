pub use crate::types::rfqs::enums::{Direction, RFQStatus};
use crate::types::shared::{serde_nonce, serde_option_nonce};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LegUnpriced {
    /// Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    /// Leg direction
    pub direction: Direction,
    /// Instrument name
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
    /// Amount in units of the base
    pub amount: bigdecimal::BigDecimal,
    /// Leg direction
    pub direction: Direction,
    /// Instrument name
    pub instrument_name: String,
    /// Leg price
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
    pub fn from_unpriced(leg: &LegUnpriced, price: bigdecimal::BigDecimal) -> Self {
        LegPriced {
            amount: leg.amount.clone(),
            direction: leg.direction,
            instrument_name: leg.instrument_name.clone(),
            price,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RfqParams {
    /// Optional user-defined label for the RFQ
    #[serde(default)]
    pub label: String,
    /// RFQ legs
    pub legs: Vec<LegUnpriced>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_total_cost: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_total_cost: Option<bigdecimal::BigDecimal>,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counterparties: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_fill_step: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

impl From<&RfqParams> for RfqParams {
    fn from(value: &RfqParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QuoteParams {
    pub direction: Direction,
    #[serde(default)]
    pub label: String,
    pub legs: Vec<LegPriced>,
    pub max_fee: bigdecimal::BigDecimal,
    #[serde(default)]
    pub mmp: bool,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub rfq_id: uuid::Uuid,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

impl From<&QuoteParams> for QuoteParams {
    fn from(value: &QuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExecuteQuoteParams {
    pub direction: Direction,
    #[serde(default)]
    pub label: String,
    pub legs: Vec<LegPriced>,
    pub max_fee: bigdecimal::BigDecimal,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    pub quote_id: uuid::Uuid,
    pub rfq_id: uuid::Uuid,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_taker_protection: Option<bool>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
}
impl From<&ExecuteQuoteParams> for ExecuteQuoteParams {
    fn from(value: &ExecuteQuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReplaceQuoteParams {
    pub direction: Direction,
    #[serde(default)]
    pub label: String,
    pub legs: Vec<LegPriced>,
    pub max_fee: bigdecimal::BigDecimal,
    #[serde(default)]
    pub mmp: bool,
    #[serde(with = "serde_nonce")]
    pub nonce: i64,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "serde_option_nonce")]
    pub nonce_to_cancel: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id_to_cancel: Option<uuid::Uuid>,
    pub rfq_id: uuid::Uuid,
    pub signature: String,
    pub signature_expiry_sec: i64,
    pub signer: String,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra_fee: Option<bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub referral_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
}

impl From<&ReplaceQuoteParams> for ReplaceQuoteParams {
    fn from(value: &ReplaceQuoteParams) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PollRfqsParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_subaccount_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RFQStatus>,
    pub subaccount_id: i64,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_timestamp: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<RFQStatus>,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_timestamp: Option<i64>,
}

impl From<&GetQuotesParams> for GetQuotesParams {
    fn from(value: &GetQuotesParams) -> Self {
        value.clone()
    }
}
