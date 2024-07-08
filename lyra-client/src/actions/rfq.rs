use crate::actions::helpers::ModuleData;
use crate::actions::{ActionData, OrderArgs};
use crate::utils::{decimal_to_i256, decimal_to_u256, decimal_to_u256_with_prec, u256_to_decimal};
use anyhow::Result;
use bigdecimal::{BigDecimal, Zero};
use ethers::abi;
use ethers::abi::{AbiDecode, AbiEncode, Tokenizable, Tokenize};
use ethers::prelude::{
    abigen, Address, EthAbiCodec, EthAbiType, EthEvent, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;
use log::{debug, info};
pub use orderbook_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
pub use orderbook_types::types::rfqs::LegPriced;
use orderbook_types::types::rfqs::{
    ExecuteQuoteParams, LegUnpriced, QuoteParams, QuoteResultPublic, RfqParams,
};
use orderbook_types::types::tickers::InstrumentTicker;
use serde::Deserialize;
use std::collections::HashMap;
use uuid::Uuid;

fn get_rfq_max_fee(
    legs: &Vec<LegPriced>,
    tickers: &HashMap<String, InstrumentTicker>,
) -> BigDecimal {
    legs.iter().map(|leg| tickers[&leg.instrument_name].get_leg_max_fee(&leg.amount)).sum()
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, EthAbiType)]
pub struct LegData {
    asset_address: Address,
    sub_id: U256,
    price: U256,
    amount: I256,
}
impl LegData {
    pub fn from_maker_leg(
        leg: &LegPriced,
        ticker: &InstrumentTicker,
        maker_direction: Direction,
    ) -> Result<Self> {
        let sign = leg.direction.sign() * maker_direction.sign();
        Ok(Self {
            asset_address: ticker.base_asset_address.parse()?,
            sub_id: ticker.base_asset_sub_id.parse::<u128>()?.into(),
            price: decimal_to_u256(leg.price.clone())?,
            amount: decimal_to_i256(leg.amount.clone() * sign)?,
        })
    }
    pub fn from_maker_legs(
        legs: &Vec<LegPriced>,
        tickers: &HashMap<String, InstrumentTicker>,
        maker_direction: Direction,
    ) -> Result<Vec<Self>> {
        legs.iter()
            .map(|leg| {
                LegData::from_maker_leg(leg, &tickers[&leg.instrument_name], maker_direction)
            })
            .collect()
    }
}

#[derive(Clone, EthAbiType, Default, Debug, PartialEq, Eq, Hash)]
pub struct QuoteData {
    max_fee: U256,
    legs: Vec<LegData>,
}

/// NOTE - had to re-implement AbiEncode instead of EthAbiCodec derive because
/// for w/e reason the derive does not wrap into Tuple() and encoding gets wrong
impl AbiEncode for QuoteData {
    fn encode(self) -> Vec<u8> {
        let tokens = { abi::Token::Tuple(self.into_tokens()) };
        abi::encode(&[tokens])
    }
}

impl QuoteData {
    pub fn into_execute(self) -> ExecuteData {
        let legs_hash = ethers::utils::keccak256(&self.legs.encode());
        ExecuteData { legs_hash, max_fee: self.max_fee }
    }
    pub fn from_quote_result(
        quote: &QuoteResultPublic,
        tickers: &HashMap<String, InstrumentTicker>,
    ) -> Result<Self> {
        Self::from_legs(&quote.legs, quote.direction, tickers)
    }
    pub fn from_legs(
        legs: &Vec<LegPriced>,
        direction: Direction,
        tickers: &HashMap<String, InstrumentTicker>,
    ) -> Result<Self> {
        let max_fee = get_rfq_max_fee(&legs, tickers);
        let legs = LegData::from_maker_legs(legs, tickers, direction)?;
        Ok(Self { max_fee: decimal_to_u256(max_fee)?, legs })
    }
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiCodec, EthAbiType)]
pub struct ExecuteData {
    legs_hash: [u8; 32],
    max_fee: U256,
}

impl ModuleData for QuoteData {
    fn address(&self) -> Address {
        let addr = std::env::var("RFQ_ADDRESS").expect("RFQ_ADDRESS must be set");
        addr.parse().expect("RFQ_ADDRESS must be a valid module address")
    }
}

impl ModuleData for ExecuteData {
    fn address(&self) -> Address {
        let addr = std::env::var("RFQ_ADDRESS").expect("RFQ_ADDRESS must be set");
        addr.parse().expect("RFQ_ADDRESS must be a valid module address")
    }
}

#[derive(Debug, Deserialize)]
pub struct QuoteArgs {
    pub rfq_id: Uuid,
    pub direction: Direction,
    pub legs: Vec<LegPriced>,
}

pub fn new_quote_params(
    signer: &LocalWallet,
    tickers: &HashMap<String, InstrumentTicker>,
    subaccount_id: i64,
    args: QuoteArgs,
) -> Result<QuoteParams> {
    let quote_data = QuoteData::from_legs(&args.legs, args.direction, &tickers)?;
    let quote_action = ActionData::new(quote_data, subaccount_id, signer.address())?;
    quote_action.to_quote_params(signer, &tickers, args)
}

pub fn new_execute_params(
    signer: &LocalWallet,
    tickers: &HashMap<String, InstrumentTicker>,
    subaccount_id: i64,
    quote: &QuoteResultPublic,
) -> Result<ExecuteQuoteParams> {
    let quote_data = QuoteData::from_quote_result(&quote, &tickers)?;
    let execute_data = quote_data.into_execute();
    let execute_action = ActionData::new(execute_data, subaccount_id, signer.address())?;
    execute_action.to_execute_params(signer, &tickers, &quote)
}

impl ActionData {
    pub fn to_quote_params(
        self,
        signer: &LocalWallet,
        tickers: &HashMap<String, InstrumentTicker>,
        args: QuoteArgs,
    ) -> Result<QuoteParams> {
        let max_fee = get_rfq_max_fee(&args.legs, tickers);
        Ok(QuoteParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            direction: args.direction,
            label: "".to_string(),
            legs: args.legs,
            max_fee,
            nonce: self.nonce.as_u64() as i64,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            mmp: false,
            rfq_id: args.rfq_id,
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }
    pub fn to_execute_params(
        self,
        signer: &LocalWallet,
        tickers: &HashMap<String, InstrumentTicker>,
        quote: &QuoteResultPublic,
    ) -> Result<ExecuteQuoteParams> {
        let max_fee = get_rfq_max_fee(&quote.legs, &tickers);
        Ok(ExecuteQuoteParams {
            subaccount_id: self.subaccount_id.as_u64() as i64,
            direction: quote.direction.opposite(),
            label: "".to_string(),
            legs: quote.legs.clone(),
            max_fee,
            nonce: self.nonce.as_u64() as i64,
            quote_id: quote.quote_id,
            rfq_id: quote.rfq_id,
            signature_expiry_sec: self.expiry.as_u64() as i64,
            signer: hex::encode_prefixed(self.signer),
            signature: signer.sign_hash(self.hash().into())?.to_string(),
        })
    }
}
