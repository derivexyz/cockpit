//! Execution shared by the single-leg short-premium books.
//!
//! Every one of these strategies does the same three things once it has decided to trade: pick
//! the listed option nearest a target tenor and delta, size it against a notional, and walk a
//! limit price from mark towards the book on a progressively shocked IV. Only the decision of
//! *whether* and *which side* differs, so that is all a strategy implements itself.

use crate::market::MarketData;
use crate::shared::auction::{LimitOrderAuction, OrderStrategy};
use anyhow::{bail, Context, Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, RoundingMode, ToPrimitive, Zero};
use lyra_client::actions::Direction;
use lyra_utils::black76::OptionContract;
use orderbook_types::types::tickers::result::InstrumentTicker;
use orderbook_types::types::tickers::OptionType;
use serde::Deserialize;

pub const DAY_SEC: f64 = 86400.0;

/// How one auction runs: how long it lasts, and how its limit price is walked over that time as
/// a fraction of the instrument's mark IV.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct AuctionConfig {
    /// Where the price walk starts. 0.0 quotes at mark.
    pub init_iv_spread: f64,
    /// How much further the IV is shocked per minute of the auction.
    pub iv_spread_per_min: f64,
    /// The furthest the IV may be shocked, whatever the elapsed time.
    pub max_iv_spread: f64,
    /// How long the auction runs before it gives up and leaves the decision to the next hour.
    pub auction_sec: i64,
    /// Re-quote once the desired price has moved this far from the resting order.
    pub price_change_tolerance: BigDecimal,
}

impl AuctionConfig {
    /// The IV shock in force at `now`: starts at `init_iv_spread`, grows per minute of the
    /// auction, and never exceeds `max_iv_spread`.
    pub fn spread_at(&self, start_timestamp_sec: i64, now: i64) -> f64 {
        let min_since_start = (now - start_timestamp_sec).max(0) as f64 / 60.0;
        (self.init_iv_spread + min_since_start * self.iv_spread_per_min).min(self.max_iv_spread)
    }
}

/// The tenor and strike a strategy wants, how much of it to write, and the tolerances it will
/// accept.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LegTarget {
    /// The spot asset held as collateral, e.g. `LBTC`. Size follows this balance, so the book
    /// only ever writes what the vault is actually holding.
    pub spot_currency: String,
    /// Contracts to write per unit of the spot balance. 1.0 writes the whole stack.
    pub notional_ratio: BigDecimal,
    /// Hard cap on contracts, whatever the balance says. The guardrail while a book is being
    /// proved out: a large collateral balance cannot turn into a large position by itself.
    pub max_contracts: BigDecimal,
    /// Target absolute delta of the short leg, e.g. `0.05`.
    pub target_delta: BigDecimal,
    /// Reject a strike further than this from the target delta (spec L3).
    pub max_delta_dev: BigDecimal,
    /// Reject a bid below `(1 - dev) * mark`, or an ask above `(1 + dev) * mark` (spec L2).
    pub max_mark_dev: BigDecimal,
    /// Tenor centre, in days.
    pub tenor_target_days: f64,
    /// Tenor half-band, in days: entries only within `target +/- band`.
    pub tenor_band_days: f64,
}

impl LegTarget {
    /// The tenor band in seconds, as `(min, max)`.
    pub fn tenor_band_sec(&self) -> Result<(i64, i64)> {
        let min = (self.tenor_target_days - self.tenor_band_days) * DAY_SEC;
        let max = (self.tenor_target_days + self.tenor_band_days) * DAY_SEC;
        if !(min > 0.0) || max <= min {
            bail!("tenor band {}d +/- {}d is empty", self.tenor_target_days, self.tenor_band_days);
        }
        Ok((min as i64, max as i64))
    }

    /// Contracts to write: the spot collateral times [Self::notional_ratio], capped at
    /// [Self::max_contracts] and floored to the instrument's amount step.
    ///
    /// Sizing reads the balance every decision rather than caching it, so a deposit or a
    /// withdrawal is reflected on the next hour without restarting the vault.
    pub fn target_contracts(
        &self,
        market: &MarketData,
        ticker: &InstrumentTicker,
    ) -> Result<BigDecimal> {
        let zero = BigDecimal::zero();
        if self.notional_ratio <= zero {
            bail!("notional_ratio must be positive");
        }
        if self.max_contracts <= zero {
            bail!("max_contracts must be positive");
        }
        let spot = market.get_amount(&self.spot_currency);
        if spot <= zero {
            bail!("no {} collateral to write against, balance {}", self.spot_currency, spot);
        }

        let wanted = (&spot * &self.notional_ratio).min(self.max_contracts.clone());
        let amount = wanted
            .with_scale_round(ticker.amount_step.fractional_digit_count(), RoundingMode::Down);
        if amount < ticker.minimum_amount {
            bail!(
                "{} of {} collateral sizes {} contracts of {}, below its minimum {}",
                self.notional_ratio,
                self.spot_currency,
                amount,
                ticker.instrument_name,
                ticker.minimum_amount
            );
        }
        log::info!(
            "sizing {} contracts of {}: {} {} x {} capped at {}",
            amount,
            ticker.instrument_name,
            spot,
            self.spot_currency,
            self.notional_ratio,
            self.max_contracts
        );
        Ok(amount)
    }

    /// L1 + L2 for the side being crossed: a one-sided or stale book is untradeable.
    pub fn passes_liquidity_gates(&self, ticker: &InstrumentTicker, side: Direction) -> bool {
        let zero = BigDecimal::zero();
        if ticker.best_bid_price <= zero
            || ticker.best_ask_price <= zero
            || ticker.mark_price <= zero
        {
            return false;
        }
        match side {
            // selling crosses the bid, so refuse a collapsed one
            Direction::Sell => {
                ticker.best_bid_price
                    >= (BigDecimal::from(1) - &self.max_mark_dev) * &ticker.mark_price
            }
            // buying crosses the ask, so refuse a blown-out one
            Direction::Buy => {
                ticker.best_ask_price
                    <= (BigDecimal::from(1) + &self.max_mark_dev) * &ticker.mark_price
            }
        }
    }

    /// Picks the leg to sell out of `eligible`: the expiry nearest the tenor target, then within
    /// that expiry the strike nearest the target delta, subject to the delta and liquidity gates.
    ///
    /// `eligible` is expected to come from [crate::signals::Selector::eligible], i.e. already
    /// filtered to live tickers of the right currency, type and tenor band.
    pub fn select_leg(
        &self,
        eligible: &[&InstrumentTicker],
        option_type: OptionType,
        now: i64,
    ) -> Result<InstrumentTicker> {
        let of_type: Vec<&&InstrumentTicker> = eligible
            .iter()
            .filter(|ticker| {
                ticker.option_details.as_ref().is_some_and(|d| d.option_type == option_type)
            })
            .collect();

        let target_sec = (self.tenor_target_days * DAY_SEC) as i64;
        let expiry = of_type
            .iter()
            .filter_map(|ticker| ticker.option_details.as_ref().map(|d| d.expiry))
            .min_by_key(|expiry| (expiry - now - target_sec).abs())
            .ok_or_else(|| {
                Error::msg(format!("no live {:?} within the tenor band", option_type))
            })?;

        // deltas are signed: puts are negative, so compare on magnitude
        let selected = of_type
            .into_iter()
            .filter(|ticker| {
                ticker.option_details.as_ref().is_some_and(|d| d.expiry == expiry)
            })
            .filter(|ticker| self.passes_liquidity_gates(ticker, Direction::Sell))
            .filter(|ticker| {
                ticker.option_pricing.as_ref().is_some_and(|pricing| {
                    (pricing.delta.abs() - &self.target_delta).abs() <= self.max_delta_dev
                })
            })
            .min_by(|left, right| {
                let distance = |ticker: &InstrumentTicker| {
                    (ticker.option_pricing.as_ref().unwrap().delta.abs() - &self.target_delta).abs()
                };
                distance(left)
                    .cmp(&distance(right))
                    .then_with(|| left.instrument_name.cmp(&right.instrument_name))
            })
            .ok_or_else(|| {
                Error::msg(format!(
                    "no tradeable {:?} within {} of {} delta at expiry {}",
                    option_type, self.max_delta_dev, self.target_delta, expiry
                ))
            })?;

        log::info!(
            "selected {} at {:.3} delta, {:.2} DTE, bid {} vs mark {}",
            selected.instrument_name,
            selected.option_pricing.as_ref().unwrap().delta.to_f64().unwrap_or(f64::NAN),
            (expiry - now) as f64 / DAY_SEC,
            selected.best_bid_price,
            selected.mark_price
        );
        Ok((*selected).clone())
    }

    /// The limit price for one side of the leg: the option repriced at a progressively shocked
    /// IV, rather than whatever the book happens to show.
    ///
    /// The walk starts at mark (`init_iv_spread = 0`) and shocks IV against us as the auction
    /// runs — down when selling, up when buying — so a fill is paid for in vol points we chose
    /// rather than in whatever the spread was at that instant. `max_iv_spread` is the guardrail:
    /// past it the price stops moving and the auction simply expires unfilled.
    ///
    /// The walk is also never allowed past the book, so at its most aggressive it crosses at the
    /// bid (or ask) and no further, which is the spec's "cross" fill as a limit rather than a
    /// starting point.
    pub fn desired_price_at(
        &self,
        auction_cfg: &AuctionConfig,
        ticker: &InstrumentTicker,
        direction: Direction,
        start_timestamp_sec: i64,
        now: i64,
    ) -> Result<BigDecimal> {
        let details = ticker.option_details.as_ref().ok_or_else(|| {
            Error::msg(format!("{} has no option details", ticker.instrument_name))
        })?;
        let pricing = ticker.option_pricing.as_ref().ok_or_else(|| {
            Error::msg(format!("{} has no option pricing", ticker.instrument_name))
        })?;
        let seconds_to_expiry = details.expiry - now;
        if seconds_to_expiry <= 0 {
            bail!("{} is expired", ticker.instrument_name);
        }

        let mark_iv = pricing.iv.to_f64().context("IV cast to f64 failed")?;
        let spread = auction_cfg.spread_at(start_timestamp_sec, now);
        let iv = match direction {
            Direction::Sell => mark_iv * (1.0 - spread),
            Direction::Buy => mark_iv * (1.0 + spread),
        };
        let contract = OptionContract {
            strike: details.strike.to_f64().context("strike cast to f64 failed")?,
            expiry_sec: seconds_to_expiry as f64,
            is_call: details.option_type == OptionType::C,
        };
        let forward = pricing.forward_price.to_f64().context("forward cast to f64 failed")?;
        let price = BigDecimal::from_f64(contract.price(forward, iv))
            .context("option price cannot be represented as a decimal")?
            .round(ticker.tick_size.fractional_digit_count())
            .max(ticker.min_price.clone())
            .min(ticker.max_price.clone());

        // stop at the book: never quote through a crossed fill
        let price = match direction {
            Direction::Sell => price.max(ticker.best_bid_price.clone()),
            Direction::Buy => price.min(ticker.best_ask_price.clone()),
        };
        log::debug!(
            "{} {} at {:.4} spread: mark_iv {:.4} -> {:.4}, price {}",
            direction.to_string(),
            ticker.instrument_name,
            spread,
            mark_iv,
            iv,
            price
        );
        Ok(price)
    }
}

/// What an auction is being asked to do.
#[derive(Clone, Debug, PartialEq)]
pub enum ShortLegOrder {
    /// Sell until short `target_contracts`.
    SellToTarget { target_contracts: BigDecimal },
    /// Buy back the whole short.
    BuyBack,
}

impl ShortLegOrder {
    pub fn direction(&self) -> Direction {
        match self {
            Self::SellToTarget { .. } => Direction::Sell,
            Self::BuyBack => Direction::Buy,
        }
    }
}

/// Prices and sizes one leg for one decision.
#[derive(Clone, Debug)]
pub struct ShortLegOrderStrategy {
    pub target: LegTarget,
    pub auction_cfg: AuctionConfig,
    pub order: ShortLegOrder,
}

impl ShortLegOrderStrategy {
    pub fn desired_amount(
        &self,
        ticker: &InstrumentTicker,
        current_position: &BigDecimal,
    ) -> (Direction, BigDecimal) {
        let direction = self.order.direction();
        let zero = BigDecimal::zero();
        // the crossed side must still be sane, else skip this hour and retry (spec L5)
        if !self.target.passes_liquidity_gates(ticker, direction) {
            return (direction, zero);
        }
        let amount = match &self.order {
            ShortLegOrder::SellToTarget { target_contracts } => target_contracts + current_position,
            ShortLegOrder::BuyBack => -current_position,
        };
        if amount <= zero {
            return (direction, zero);
        }
        let amount = amount
            .with_scale_round(ticker.amount_step.fractional_digit_count(), RoundingMode::Down);
        match amount < ticker.minimum_amount {
            true => (direction, zero),
            false => (direction, amount),
        }
    }
}

#[async_trait::async_trait]
impl OrderStrategy for ShortLegOrderStrategy {
    async fn get_desired_price(&self, auction: &LimitOrderAuction) -> Result<BigDecimal> {
        let reader = auction.market.read().await;
        let ticker = reader
            .get_ticker(&auction.instrument_name)
            .ok_or_else(|| Error::msg("ticker not found"))?;
        self.target.desired_price_at(
            &self.auction_cfg,
            ticker,
            self.order.direction(),
            auction.start_timestamp_sec,
            chrono::Utc::now().timestamp(),
        )
    }

    async fn get_desired_amount(
        &self,
        auction: &LimitOrderAuction,
        _price: &BigDecimal,
    ) -> Result<(Direction, BigDecimal)> {
        if auction.remain_sec() <= 0 {
            return Ok((self.order.direction(), BigDecimal::zero()));
        }
        let reader = auction.market.read().await;
        let ticker = reader
            .get_ticker(&auction.instrument_name)
            .ok_or_else(|| Error::msg("ticker not found"))?;
        let current_position = reader.get_amount(&auction.instrument_name);
        Ok(self.desired_amount(ticker, &current_position))
    }
}
