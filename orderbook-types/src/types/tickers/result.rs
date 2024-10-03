#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::types::orders::{LiquidityRole, OrderResponse};
use crate::types::shared::{RPCError, RPCId};
pub use crate::types::tickers::enums::{InstrumentType, OptionType, TickerInterval};
use bigdecimal;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AggregateTradingStatsSchema {
    ///Number of contracts traded during last 24 hours
    pub contract_volume: bigdecimal::BigDecimal,
    ///Highest trade price during last 24h
    pub high: bigdecimal::BigDecimal,
    ///Lowest trade price during last 24h
    pub low: bigdecimal::BigDecimal,
    ///Number of trades during last 24h
    pub num_trades: bigdecimal::BigDecimal,
    ///Current total open interest
    pub open_interest: bigdecimal::BigDecimal,
    ///24-hour price change expressed as a percentage. Options: percent change in vol; Perps: percent change in mark price
    pub percent_change: bigdecimal::BigDecimal,
    ///24-hour price change in USD.
    pub usd_change: bigdecimal::BigDecimal,
}
impl From<&AggregateTradingStatsSchema> for AggregateTradingStatsSchema {
    fn from(value: &AggregateTradingStatsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstrumentTicker {
    ///Minimum valid increment of order amount
    pub amount_step: bigdecimal::BigDecimal,
    ///Blockchain address of the base asset
    pub base_asset_address: String,
    ///Sub ID of the specific base asset as defined in Asset.sol
    pub base_asset_sub_id: String,
    ///Underlying currency of base asset (`ETH`, `BTC`, etc)
    pub base_currency: String,
    ///$ base fee added to every taker order
    pub base_fee: bigdecimal::BigDecimal,
    ///Amount of contracts / tokens available at best ask price
    pub best_ask_amount: bigdecimal::BigDecimal,
    ///Best ask price
    pub best_ask_price: bigdecimal::BigDecimal,
    ///Amount of contracts / tokens available at best bid price
    pub best_bid_amount: bigdecimal::BigDecimal,
    ///Best bid price
    pub best_bid_price: bigdecimal::BigDecimal,
    ///Index price
    pub index_price: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
    ///If `True`: instrument is tradeable within `activation` and `deactivation` timestamps
    pub is_active: bool,
    ///Percent of spot price fee rate for makers
    pub maker_fee_rate: bigdecimal::BigDecimal,
    ///Mark price
    pub mark_price: bigdecimal::BigDecimal,
    ///Percent of option price fee cap, e.g. 12.5%, null if not applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_price_fee_rate_cap: Option<bigdecimal::BigDecimal>,
    ///Maximum price at which an agressive buyer can be matched. Any portion of a market order that would execute above this price will be cancelled. A limit buy order with limit price above this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).
    pub max_price: bigdecimal::BigDecimal,
    ///Maximum valid amount of contracts / tokens per trade
    pub maximum_amount: bigdecimal::BigDecimal,
    ///Minimum price at which an agressive seller can be matched. Any portion of a market order that would execute below this price will be cancelled. A limit sell order with limit price below this value is treated as post only (i.e. it will be rejected if it would cross any existing resting order).
    pub min_price: bigdecimal::BigDecimal,
    ///Minimum valid amount of contracts / tokens per trade
    pub minimum_amount: bigdecimal::BigDecimal,
    ///Details of the option asset (if applicable)
    pub option_details: Option<OptionPublicDetailsSchema>,
    ///Greeks, forward price, iv and mark price of the instrument (options only)
    pub option_pricing: Option<OptionPricingSchema>,
    ///Details of the perp asset (if applicable)
    pub perp_details: Option<PerpPublicDetailsSchema>,
    ///Quote currency (`USD` for perps, `USDC` for options)
    pub quote_currency: String,
    ///Timestamp at which became or will become active (if applicable)
    pub scheduled_activation: i64,
    ///Scheduled deactivation time for instrument (if applicable)
    pub scheduled_deactivation: i64,
    ///Aggregate trading stats for the last 24 hours
    /// TODO errors out right now due to null usd_change
    // pub stats: AggregateTradingStatsSchema,
    ///Percent of spot price fee rate for takers
    pub taker_fee_rate: bigdecimal::BigDecimal,
    ///Tick size of the instrument, i.e. minimum price increment
    pub tick_size: bigdecimal::BigDecimal,
    pub fifo_min_allocation: bigdecimal::BigDecimal,
    pub pro_rata_amount_step: bigdecimal::BigDecimal,
    pub pro_rata_fraction: bigdecimal::BigDecimal,
    ///Timestamp of the ticker feed snapshot
    pub timestamp: i64,
}
impl From<&InstrumentTicker> for InstrumentTicker {
    fn from(value: &InstrumentTicker) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OptionPricingSchema {
    ///Implied volatility of the current best ask
    pub ask_iv: bigdecimal::BigDecimal,
    ///Implied volatility of the current best bid
    pub bid_iv: bigdecimal::BigDecimal,
    ///Delta of the option
    pub delta: bigdecimal::BigDecimal,
    ///Forward price used to calculate option premium
    pub forward_price: bigdecimal::BigDecimal,
    ///Gamma of the option
    pub gamma: bigdecimal::BigDecimal,
    ///Implied volatility of the option
    pub iv: bigdecimal::BigDecimal,
    ///Mark price of the option
    pub mark_price: bigdecimal::BigDecimal,
    ///Rho of the option
    pub rho: bigdecimal::BigDecimal,
    ///Theta of the option
    pub theta: bigdecimal::BigDecimal,
    ///Vega of the option
    pub vega: bigdecimal::BigDecimal,
}
impl From<&OptionPricingSchema> for OptionPricingSchema {
    fn from(value: &OptionPricingSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OptionPublicDetailsSchema {
    ///Unix timestamp of expiry date (in seconds)
    pub expiry: i64,
    ///Underlying settlement price index
    pub index: String,
    pub option_type: OptionType,
    ///Settlement price of the option
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_price: Option<bigdecimal::BigDecimal>,
    pub strike: bigdecimal::BigDecimal,
}
impl From<&OptionPublicDetailsSchema> for OptionPublicDetailsSchema {
    fn from(value: &OptionPublicDetailsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PerpPublicDetailsSchema {
    ///Latest aggregated funding as per `PerpAsset.sol`
    pub aggregate_funding: bigdecimal::BigDecimal,
    ///Current hourly funding rate as per `PerpAsset.sol`
    pub funding_rate: bigdecimal::BigDecimal,
    ///Underlying spot price index for funding rate
    pub index: String,
    ///Max rate per hour as per `PerpAsset.sol`
    pub max_rate_per_hour: bigdecimal::BigDecimal,
    ///Min rate per hour as per `PerpAsset.sol`
    pub min_rate_per_hour: bigdecimal::BigDecimal,
    ///Static interest rate as per `PerpAsset.sol`
    pub static_interest_rate: bigdecimal::BigDecimal,
}
impl From<&PerpPublicDetailsSchema> for PerpPublicDetailsSchema {
    fn from(value: &PerpPublicDetailsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerInstrumentNameIntervalNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: TickerNotificationData,
}
impl From<&TickerInstrumentNameIntervalNotificationParamsSchema>
    for TickerInstrumentNameIntervalNotificationParamsSchema
{
    fn from(value: &TickerInstrumentNameIntervalNotificationParamsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerInstrumentNameIntervalNotificationSchema {
    pub method: String,
    pub params: TickerInstrumentNameIntervalNotificationParamsSchema,
}
impl From<&TickerInstrumentNameIntervalNotificationSchema>
    for TickerInstrumentNameIntervalNotificationSchema
{
    fn from(value: &TickerInstrumentNameIntervalNotificationSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerNotificationData {
    ///Instrument of the ticker feed snapshot
    pub instrument_ticker: InstrumentTicker,
    ///Timestamp of the ticker feed snapshot
    pub timestamp: i64,
}
impl From<&TickerNotificationData> for TickerNotificationData {
    fn from(value: &TickerNotificationData) -> Self {
        value.clone()
    }
}

impl InstrumentTicker {
    pub fn get_max_fee(&self) -> BigDecimal {
        let max_base_fee = &self.base_fee / &self.minimum_amount;
        let max_fee = BigDecimal::from(3) * &self.taker_fee_rate * &self.index_price + max_base_fee;
        max_fee.with_scale_round(6, bigdecimal::RoundingMode::Up)
    }
    pub fn get_unit_fee(&self, role: LiquidityRole) -> BigDecimal {
        match role {
            LiquidityRole::Maker => self.maker_fee_rate.clone() * &self.index_price,
            LiquidityRole::Taker => self.taker_fee_rate.clone() * &self.index_price,
        }
    }
    pub fn get_leg_max_fee(&self, amount: &BigDecimal) -> BigDecimal {
        (BigDecimal::from_str("1.15").unwrap() * &self.taker_fee_rate * &self.index_price * amount
            + &self.base_fee)
            .with_scale_round(6, bigdecimal::RoundingMode::Up)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstrumentData {
    ///Minimum valid increment of order amount
    pub amount_step: bigdecimal::BigDecimal,
    ///Blockchain address of the base asset
    pub base_asset_address: String,
    ///Sub ID of the specific base asset as defined in Asset.sol
    pub base_asset_sub_id: String,
    ///Underlying currency of base asset (`ETH`, `BTC`, etc)
    pub base_currency: String,
    ///$ base fee added to every taker order
    pub base_fee: bigdecimal::BigDecimal,
    ///Instrument name
    pub instrument_name: String,
    ///`erc20`, `option`, or `perp`
    pub instrument_type: InstrumentType,
    ///If `True`: instrument is tradeable within `activation` and `deactivation` timestamps
    pub is_active: bool,
    ///Percent of spot price fee rate for makers
    pub maker_fee_rate: bigdecimal::BigDecimal,
    ///Percent of option price fee cap, e.g. 12.5%, null if not applicable
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_price_fee_rate_cap: Option<bigdecimal::BigDecimal>,
    ///Maximum valid amount of contracts / tokens per trade
    pub maximum_amount: bigdecimal::BigDecimal,
    ///Minimum valid amount of contracts / tokens per trade
    pub minimum_amount: bigdecimal::BigDecimal,
    ///Details of the option asset (if applicable)
    pub option_details: Option<OptionPublicDetailsSchema>,
    ///Details of the perp asset (if applicable)
    pub perp_details: Option<PerpPublicDetailsSchema>,
    ///Quote currency (`USD` for perps, `USDC` for options)
    pub quote_currency: String,
    ///Timestamp at which became or will become active (if applicable)
    pub scheduled_activation: i64,
    ///Scheduled deactivation time for instrument (if applicable)
    pub scheduled_deactivation: i64,
    ///Percent of spot price fee rate for takers
    pub taker_fee_rate: bigdecimal::BigDecimal,
    ///Tick size of the instrument, i.e. minimum price increment
    pub tick_size: bigdecimal::BigDecimal,
}
impl From<&InstrumentData> for InstrumentData {
    fn from(value: &InstrumentData) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstrumentsResponse {
    pub id: RPCId,
    pub result: Vec<InstrumentData>,
}

impl From<&InstrumentsResponse> for InstrumentsResponse {
    fn from(value: &InstrumentsResponse) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerResponse {
    pub id: RPCId,
    pub result: InstrumentTicker,
}

impl From<&TickerResponse> for TickerResponse {
    fn from(value: &TickerResponse) -> Self {
        value.clone()
    }
}
