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

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct InstrumentSlimTicker {
    /// Amount of contracts / tokens available at best ask price
    #[serde(rename = "A")]
    pub best_ask_amount: BigDecimal,
    /// Amount of contracts / tokens available at best bid price
    #[serde(rename = "B")]
    pub best_bid_amount: BigDecimal,
    /// Index price
    #[serde(rename = "I")]
    pub index_price: BigDecimal,
    /// Mark price
    #[serde(rename = "M")]
    pub mark_price: BigDecimal,
    /// Best ask price
    #[serde(rename = "a")]
    pub best_ask_price: BigDecimal,
    /// Best bid price
    #[serde(rename = "b")]
    pub best_bid_price: BigDecimal,
    /// Current hourly funding rate
    #[serde(rename = "f", default, skip_serializing_if = "Option::is_none")]
    pub funding_rate: Option<BigDecimal>,
    /// Maximum price at which an agressive buyer can be matched
    #[serde(rename = "maxp")]
    pub max_price: BigDecimal,
    /// Minimum price at which an agressive seller can be matched
    #[serde(rename = "minp")]
    pub min_price: BigDecimal,
    /// Greeks, forward price, iv and mark price of the instrument (options only)
    #[serde(rename = "option_pricing", default, skip_serializing_if = "Option::is_none")]
    pub option_pricing: Option<OptionPricingSlimSchema>,
    /// Creation timestamp of the snapshot in milliseconds
    #[serde(rename = "t")]
    pub timestamp: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct OptionPricingSlimSchema {
    /// Implied volatility of the current best ask
    #[serde(rename = "ai")]
    pub ask_iv: BigDecimal,
    /// Implied volatility of the current best bid
    #[serde(rename = "bi")]
    pub bid_iv: BigDecimal,
    /// Delta of the option
    #[serde(rename = "d")]
    pub delta: BigDecimal,
    /// Discount factor used to calculate option premium
    #[serde(rename = "df")]
    pub discount_factor: BigDecimal,
    /// Forward price used to calculate option premium
    #[serde(rename = "f")]
    pub forward_price: BigDecimal,
    /// Gamma of the option
    #[serde(rename = "g")]
    pub gamma: BigDecimal,
    /// Implied volatility of the option
    #[serde(rename = "i")]
    pub iv: BigDecimal,
    /// Mark price of the option
    #[serde(rename = "m")]
    pub mark_price: BigDecimal,
    /// Rho of the option
    #[serde(rename = "r")]
    pub rho: BigDecimal,
    /// Theta of the option
    #[serde(rename = "t")]
    pub theta: BigDecimal,
    /// Vega of the option
    #[serde(rename = "v")]
    pub vega: BigDecimal,
}

impl From<OptionPricingSlimSchema> for OptionPricingSchema {
    fn from(value: OptionPricingSlimSchema) -> Self {
        OptionPricingSchema {
            ask_iv: value.ask_iv,
            bid_iv: value.bid_iv,
            delta: value.delta,
            forward_price: value.forward_price,
            gamma: value.gamma,
            iv: value.iv,
            mark_price: value.mark_price,
            rho: value.rho,
            theta: value.theta,
            vega: value.vega,
        }
    }
}

impl From<&OptionPricingSlimSchema> for OptionPricingSchema {
    fn from(value: &OptionPricingSlimSchema) -> Self {
        value.clone().into()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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
    ///Percent of spot price fee rate for takers
    pub taker_fee_rate: bigdecimal::BigDecimal,
    ///Tick size of the instrument, i.e. minimum price increment
    pub tick_size: bigdecimal::BigDecimal,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerSlimInstrumentNameIntervalNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: TickerSlimNotificationData,
}
impl From<&TickerSlimInstrumentNameIntervalNotificationParamsSchema>
    for TickerSlimInstrumentNameIntervalNotificationParamsSchema
{
    fn from(value: &TickerSlimInstrumentNameIntervalNotificationParamsSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerSlimInstrumentNameIntervalNotificationSchema {
    pub method: String,
    pub params: TickerSlimInstrumentNameIntervalNotificationParamsSchema,
}
impl From<&TickerSlimInstrumentNameIntervalNotificationSchema>
    for TickerSlimInstrumentNameIntervalNotificationSchema
{
    fn from(value: &TickerSlimInstrumentNameIntervalNotificationSchema) -> Self {
        value.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TickerSlimNotificationData {
    ///Instrument of the ticker feed snapshot
    pub instrument_ticker: InstrumentSlimTicker,
    ///Timestamp of the ticker feed snapshot
    pub timestamp: i64,
}
impl From<&TickerSlimNotificationData> for TickerSlimNotificationData {
    fn from(value: &TickerSlimNotificationData) -> Self {
        value.clone()
    }
}

impl InstrumentTicker {
    pub fn get_max_fee(&self) -> BigDecimal {
        let max_base_fee = &self.base_fee / &self.minimum_amount;
        let use_mark = self.mark_price > self.index_price;
        let idx = if use_mark { &self.mark_price } else { &self.index_price };
        let max_fee = BigDecimal::from(3) * &self.taker_fee_rate * idx + max_base_fee;
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

    pub fn from_slim_and_data(slim: InstrumentSlimTicker, data: &InstrumentData) -> Self {
        InstrumentTicker {
            amount_step: data.amount_step.clone(),
            base_asset_address: data.base_asset_address.clone(),
            base_asset_sub_id: data.base_asset_sub_id.clone(),
            base_currency: data.base_currency.clone(),
            base_fee: data.base_fee.clone(),
            best_ask_amount: slim.best_ask_amount,
            best_ask_price: slim.best_ask_price,
            best_bid_amount: slim.best_bid_amount,
            best_bid_price: slim.best_bid_price,
            index_price: slim.index_price,
            instrument_name: data.instrument_name.clone(),
            instrument_type: data.instrument_type,
            is_active: data.is_active,
            maker_fee_rate: data.maker_fee_rate.clone(),
            mark_price: slim.mark_price,
            mark_price_fee_rate_cap: data.mark_price_fee_rate_cap.clone(),
            max_price: slim.max_price,
            maximum_amount: data.maximum_amount.clone(),
            min_price: slim.min_price,
            minimum_amount: data.minimum_amount.clone(),
            option_details: data.option_details.clone(),
            option_pricing: slim.option_pricing.map(OptionPricingSchema::from),
            perp_details: data.perp_details.clone(),
            quote_currency: data.quote_currency.clone(),
            scheduled_activation: data.scheduled_activation,
            scheduled_deactivation: data.scheduled_deactivation,
            taker_fee_rate: data.taker_fee_rate.clone(),
            tick_size: data.tick_size.clone(),
            timestamp: slim.timestamp,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
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
pub struct InstrumentResponse {
    pub id: RPCId,
    pub result: InstrumentData,
}

impl From<&InstrumentResponse> for InstrumentResponse {
    fn from(value: &InstrumentResponse) -> Self {
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
