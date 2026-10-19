//! Swather: a weekly naked short 10-delta call, fully covered, single leg and single vintage.
//!
//! Per the strategy spec §3, with §1 tenor selection and §5 liquidity gates:
//!
//! - **Tenor**: the listed expiry nearest 7 days, within a hard 4-10 DTE band. Out of band, no trade.
//! - **Strike**: the call whose delta is nearest 0.10, and never more than 0.10 away from it.
//! - **Gate**: `iv_rank >= 40 AND mom_30d <= +5%`, held on for 72 hours after its last True.
//! - **Entry**: flat and gate on, sell at the bid.
//! - **Exit**: gate off and the leg is past its 3-day minimum hold, buy back at the ask — unless
//!   it is inside 1 DTE, where the spec's production hardening says ride to settlement instead.
//! - **Ladder**: none (`ladder_n = 1`). The whole stack sells one strike at one expiry, so every
//!   order is a plain single-leg orderbook order and no RFQ is involved.
//!
//! Sizing is parameterised as a USD notional rather than the spec's "contracts = coins in the
//! stack", because a testnet subaccount has no coin stack to measure.

use crate::clickhouse::gate::{fetch_gate, GateParams};
use crate::clickhouse::ClickhouseClient;
use crate::market::{MarketData, MarketState};
use crate::shared::auction::{LimitOrderAuction, LimitOrderAuctionExecutor};
use crate::shared::stages::ExecutorStage;
use crate::signals::strategies::short_leg::{
    AuctionConfig, LegTarget, ShortLegOrder, ShortLegOrderStrategy, DAY_SEC,
};
use crate::signals::{Candidates, NoopStage, Selector, SignalStrategy};
use anyhow::{bail, Context, Result};
use bigdecimal::{BigDecimal, Zero};
use lyra_client::actions::Direction;
use orderbook_types::types::rfqs::LegUnpriced;
use orderbook_types::types::tickers::result::InstrumentTicker;
use orderbook_types::types::tickers::OptionType;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct SwatherParams {
    pub option_currency: String,
    /// Tenor, strike and sizing of the short leg, shared with the other short-premium books.
    pub leg: LegTarget,
    /// How the auction walks its limit price.
    pub auction_cfg: AuctionConfig,
    /// Minimum hold before a signal exit, in days, for a leg opened inside `long_dated_dte`.
    pub min_hold_days: f64,
    /// Minimum hold for a leg opened at or beyond `long_dated_dte`.
    pub min_hold_days_long_dated: f64,
    /// DTE at open that counts as long dated.
    pub long_dated_dte: f64,
    /// Never buy back inside this many days to expiry: ride to settlement instead (spec §6).
    pub no_close_below_dte: f64,

    pub gate: GateParams,
}

impl SwatherParams {
    pub fn tenor_band_sec(&self) -> Result<(i64, i64)> {
        self.leg.tenor_band_sec()
    }

    pub fn target_contracts(
        &self,
        market: &MarketData,
        ticker: &InstrumentTicker,
    ) -> Result<BigDecimal> {
        self.leg.target_contracts(market, ticker)
    }

    pub fn passes_liquidity_gates(&self, ticker: &InstrumentTicker, side: Direction) -> bool {
        self.leg.passes_liquidity_gates(ticker, side)
    }

    pub fn iv_spread_at(&self, start_timestamp_sec: i64, now: i64) -> f64 {
        self.auction_cfg.spread_at(start_timestamp_sec, now)
    }

    pub fn desired_price_at(
        &self,
        ticker: &InstrumentTicker,
        direction: Direction,
        start_timestamp_sec: i64,
        now: i64,
    ) -> Result<BigDecimal> {
        self.leg.desired_price_at(&self.auction_cfg, ticker, direction, start_timestamp_sec, now)
    }

    /// Minimum hold in seconds for a leg opened at `dte_at_open_sec` to expiry.
    pub fn min_hold_sec(&self, dte_at_open_sec: i64) -> i64 {
        let days = match dte_at_open_sec as f64 / DAY_SEC >= self.long_dated_dte {
            true => self.min_hold_days_long_dated,
            false => self.min_hold_days,
        };
        (days * DAY_SEC) as i64
    }

    /// Selects the call to sell: the expiry nearest the tenor target within the band, then the
    /// strike nearest the target delta, subject to the liquidity and delta gates.
    pub fn select_call(&self, market: &MarketData, now: i64) -> Result<InstrumentTicker> {
        let eligible = self.eligible(market, now)?;
        self.leg.select_leg(&eligible, OptionType::C, now)
    }
}

/// A live short call and what the engine knows about it.
#[derive(Clone, Debug, PartialEq)]
pub struct LiveLeg {
    pub instrument_name: String,
    pub amount: BigDecimal,
    pub expiry: i64,
    /// Earliest fill timestamp, in seconds, or None when no trade history is loaded.
    pub opened_at: Option<i64>,
}

impl LiveLeg {
    pub fn dte_sec(&self, now: i64) -> i64 {
        self.expiry - now
    }
}

impl Selector for SwatherParams {
    fn candidates(&self) -> Result<Candidates> {
        let (min_expiry_sec, max_expiry_sec) = self.tenor_band_sec()?;
        Ok(Candidates {
            currency: self.option_currency.clone(),
            min_expiry_sec,
            max_expiry_sec,
            option_types: vec![OptionType::C],
        })
    }

    /// The single leg this book trades: short the selected call, sized to the target notional.
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        let ticker = self.select_call(market, decision_at)?;
        Ok(vec![LegUnpriced {
            instrument_name: ticker.instrument_name.clone(),
            direction: Direction::Sell,
            amount: self.leg.target_contracts(market, &ticker)?,
        }])
    }
}

impl SwatherParams {
    /// One hourly tick of the state machine (spec §7): settle, then signal-exit, then entry.
    /// Settlement needs no action here — a settled option simply leaves the subaccount.
    pub async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>> {
        if signals.is_empty() {
            bail!("signal vector cannot be empty");
        }
        let gate_on = signals.iter().all(|signal| *signal);
        let now = chrono::Utc::now().timestamp();
        let reader = market.read().await;
        let live = live_leg(&reader, &self.option_currency)?;

        let (ticker, order) = match (&live, gate_on) {
            // LIVE + gate on: top the short up to target if a partial fill left it short
            (Some(live), true) => {
                let Some(ticker) = reader.get_ticker(&live.instrument_name) else {
                    log::warn!("Swather has no live ticker for {}, holding", live.instrument_name);
                    return Ok(Box::new(NoopStage));
                };
                let target_contracts = match self.target_contracts(&reader, ticker) {
                    Ok(contracts) => contracts,
                    Err(e) => {
                        log::info!("Swather cannot size a leg this hour: {:#}", e);
                        return Ok(Box::new(NoopStage));
                    }
                };
                (ticker.clone(), ShortLegOrder::SellToTarget { target_contracts })
            }
            // LIVE + gate off: the signal exit, subject to the minimum hold and the 1 DTE rule
            (Some(live), false) => {
                let dte_sec = live.dte_sec(now);
                if (dte_sec as f64 / DAY_SEC) < self.no_close_below_dte {
                    log::info!(
                        "Swather signal exit on {} suppressed at {:.2} DTE, riding to settlement",
                        live.instrument_name,
                        dte_sec as f64 / DAY_SEC
                    );
                    return Ok(Box::new(NoopStage));
                }
                match live.opened_at {
                    Some(opened_at) => {
                        let held_sec = now - opened_at;
                        let required = self.min_hold_sec(live.expiry - opened_at);
                        if held_sec < required {
                            log::info!(
                                "Swather holding {}: {:.2}d of the {:.2}d minimum hold",
                                live.instrument_name,
                                held_sec as f64 / DAY_SEC,
                                required as f64 / DAY_SEC
                            );
                            return Ok(Box::new(NoopStage));
                        }
                    }
                    // no trade history: refuse to exit rather than guess the hold
                    None => {
                        log::warn!(
                            "Swather has no trade history for {}, holding rather than exiting",
                            live.instrument_name
                        );
                        return Ok(Box::new(NoopStage));
                    }
                }
                let Some(ticker) = reader.get_ticker(&live.instrument_name) else {
                    log::warn!("Swather has no live ticker for {}, holding", live.instrument_name);
                    return Ok(Box::new(NoopStage));
                };
                log::info!("Swather signal exit on {}", live.instrument_name);
                (ticker.clone(), ShortLegOrder::BuyBack)
            }
            // FLAT + gate on: entry. Nothing selectable is a skipped hour, not a failure —
            // the tenor band can be empty, and out of band is never a fallback (spec §1).
            (None, true) => {
                let ticker = match self.select_call(&reader, now) {
                    Ok(ticker) => ticker,
                    Err(e) => {
                        log::info!("Swather found no call to sell this hour: {:#}", e);
                        return Ok(Box::new(NoopStage));
                    }
                };
                let target_contracts = match self.target_contracts(&reader, &ticker) {
                    Ok(contracts) => contracts,
                    Err(e) => {
                        log::info!("Swather cannot size a leg this hour: {:#}", e);
                        return Ok(Box::new(NoopStage));
                    }
                };
                (ticker, ShortLegOrder::SellToTarget { target_contracts })
            }
            // FLAT + gate off: nothing to do
            (None, false) => return Ok(Box::new(NoopStage)),
        };

        // size the order before building an auction: an auction logs in and subscribes, which is
        // wasted on every decision that turns out to have nothing to trade
        let strategy =
            ShortLegOrderStrategy { target: self.leg.clone(), auction_cfg: self.auction_cfg.clone(), order };
        let position = reader.get_amount(&ticker.instrument_name);
        let (_, amount) = strategy.desired_amount(&ticker, &position);
        drop(reader);

        if amount.is_zero() {
            log::info!("Swather has nothing to trade on {}, holding", ticker.instrument_name);
            return Ok(Box::new(NoopStage));
        }

        let auction = LimitOrderAuction::new(
            ticker.instrument_name.clone(),
            now,
            self.auction_cfg.auction_sec,
            self.auction_cfg.price_change_tolerance.clone(),
        )
        .await?;
        Ok(Box::new(LimitOrderAuctionExecutor { auction, strategy }))
    }
}

/// The vault's live short call, if any, with its open time taken from the trade history.
pub fn live_leg(market: &MarketData, option_currency: &str) -> Result<Option<LiveLeg>> {
    let prefix = format!("{option_currency}-");
    let positions = market
        .iter_positions()
        .filter(|position| {
            !position.amount.is_zero()
                && position.instrument_name.starts_with(&prefix)
                && position.instrument_name.ends_with("-C")
        })
        .collect::<Vec<_>>();

    let position = match positions.as_slice() {
        [] => return Ok(None),
        [position] => *position,
        _ => bail!("expected at most one open call position, found {}", positions.len()),
    };
    if position.amount > BigDecimal::zero() {
        bail!(
            "cannot manage {}: position {} is long, not short",
            position.instrument_name,
            position.amount
        );
    }

    let expiry = market
        .get_tickers()
        .get(&position.instrument_name)
        .and_then(|ticker| ticker.option_details.as_ref())
        .map(|details| details.expiry)
        .or_else(|| expiry_from_name(&position.instrument_name))
        .with_context(|| format!("cannot determine the expiry of {}", position.instrument_name))?;

    // the earliest fill is the open, and the trade timestamps are milliseconds
    let opened_at = market.get_trades(&position.instrument_name).and_then(|trades| {
        trades.values().map(|trade| trade.timestamp / 1000).min()
    });

    Ok(Some(LiveLeg {
        instrument_name: position.instrument_name.clone(),
        amount: position.amount.clone(),
        expiry,
        opened_at,
    }))
}

/// Falls back to the instrument name for an expiry, e.g. `ETH-20260828-3500-C` settles 08:00 UTC.
fn expiry_from_name(instrument_name: &str) -> Option<i64> {
    let date = instrument_name.split('-').nth(1)?;
    let parsed = chrono::NaiveDate::parse_from_str(date, "%Y%m%d").ok()?;
    Some(parsed.and_hms_opt(8, 0, 0)?.and_utc().timestamp())
}

/// Swather wired to its live signal source.
#[derive(Clone)]
pub struct SwatherVault {
    pub params: SwatherParams,
    pub clickhouse: Arc<ClickhouseClient>,
}

impl std::fmt::Debug for SwatherVault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SwatherVault").field("params", &self.params).finish()
    }
}

impl Selector for SwatherVault {
    fn candidates(&self) -> Result<Candidates> {
        self.params.candidates()
    }
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        self.params.select_structure(market, decision_at)
    }
}

#[async_trait::async_trait]
impl SignalStrategy for SwatherVault {
    fn name(&self) -> &str {
        "swather"
    }

    /// The effective gate: `iv_rank >= 40 AND mom <= +5%`, held on for 72 hours after its last
    /// True. An error here means the vault holds rather than acting.
    async fn signals(&self, decision_at: i64) -> Result<Vec<bool>> {
        let gate = fetch_gate(&self.clickhouse, &self.params.gate, decision_at).await?;
        Ok(vec![gate.effective])
    }

    async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>> {
        self.params.get_action(market, signals).await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::{new_market_state, Balance};
    use orderbook_types::types::orders::TradeResponse;
    use orderbook_types::types::tickers::result::{
        OptionPricingSchema, OptionPricingSlimSchema, OptionPublicDetailsSchema,
    };
    use orderbook_types::types::tickers::InstrumentType;
    use std::str::FromStr;


    fn decimal(value: &str) -> BigDecimal {
        BigDecimal::from_str(value).unwrap()
    }

    fn params() -> SwatherParams {
        SwatherParams {
            option_currency: "ETH".to_owned(),
            leg: LegTarget {
                spot_currency: "ETH".to_owned(),
                notional_ratio: decimal("1.0"),
                max_contracts: decimal("5"),
                target_delta: decimal("0.10"),
                max_delta_dev: decimal("0.10"),
                max_mark_dev: decimal("0.25"),
                tenor_target_days: 7.0,
                tenor_band_days: 3.0,
            },
            auction_cfg: AuctionConfig {
                init_iv_spread: 0.0,
                iv_spread_per_min: 0.005,
                max_iv_spread: 0.2,
                auction_sec: 1800,
                price_change_tolerance: decimal("0.1"),
            },
            min_hold_days: 3.0,
            min_hold_days_long_dated: 5.0,
            long_dated_dte: 14.0,
            no_close_below_dte: 1.0,
            gate: GateParams::swather("ETH"),
        }
    }

    /// A tradeable call: two-sided, bid at 90% of mark, so it passes L1/L2.
    fn call(name: &str, expiry: i64, delta: &str) -> InstrumentTicker {
        priced_call(name, expiry, delta, "9.0", "11.0", "10.0")
    }

    fn priced_call(
        name: &str,
        expiry: i64,
        delta: &str,
        bid: &str,
        ask: &str,
        mark: &str,
    ) -> InstrumentTicker {
        let mut pricing: OptionPricingSchema = OptionPricingSlimSchema::default().into();
        pricing.delta = decimal(delta);
        pricing.iv = decimal("0.50");
        pricing.forward_price = decimal("2000");
        InstrumentTicker {
            amount_step: decimal("0.1"),
            base_currency: "ETH".to_owned(),
            best_bid_price: decimal(bid),
            best_ask_price: decimal(ask),
            index_price: decimal("2000"),
            instrument_name: name.to_owned(),
            instrument_type: InstrumentType::Option,
            is_active: true,
            mark_price: decimal(mark),
            max_price: decimal("10000"),
            min_price: decimal("0.01"),
            minimum_amount: decimal("0.1"),
            option_details: Some(OptionPublicDetailsSchema {
                expiry,
                index: "ETH-USD".to_owned(),
                option_type: OptionType::C,
                settlement_price: None,
                strike: decimal("2200"),
            }),
            option_pricing: Some(pricing),
            tick_size: decimal("0.01"),
            timestamp: chrono::Utc::now().timestamp_millis(),
            ..InstrumentTicker::default()
        }
    }

    fn trade(instrument_name: &str, timestamp_ms: i64) -> TradeResponse {
        serde_json::from_value(serde_json::json!({
            "direction": "sell",
            "index_price": "2000",
            "instrument_name": instrument_name,
            "is_transfer": false,
            "label": "",
            "liquidity_role": "taker",
            "mark_price": "10",
            "order_id": "00000000-0000-0000-0000-000000000000",
            "quote_id": null,
            "realized_pnl": "0",
            "subaccount_id": 1,
            "timestamp": timestamp_ms,
            "trade_amount": "5",
            "trade_fee": "0",
            "trade_id": format!("trade-{timestamp_ms}"),
            "trade_price": "10",
            "tx_hash": null,
            "tx_status": "settled",
            "expected_rebate": "0",
            "pnl": "0",
            "realized_pnl_excl_fees": "0"
        }))
        .unwrap()
    }

    fn day(days: f64) -> i64 {
        (days * DAY_SEC) as i64
    }

    #[test]
    fn tenor_band_is_four_to_ten_days() {
        assert_eq!(params().tenor_band_sec().unwrap(), (day(4.0), day(10.0)));
        // the band must be non-empty and start above zero: a 0-DTE entry is never allowed
        let mut wide = params();
        wide.leg.tenor_band_days = 7.0;
        assert!(wide.tenor_band_sec().is_err());
    }

    #[test]
    fn selection_takes_the_expiry_nearest_the_target_then_the_delta() {
        let now = chrono::Utc::now().timestamp();
        let mut market = MarketData::new();
        // out of band on both sides: never selectable
        market.insert_ticker(call("ETH-3D-C", now + day(3.0), "0.10"));
        market.insert_ticker(call("ETH-11D-C", now + day(11.0), "0.10"));
        // in band, but further from 7 days than the 6-day expiry
        market.insert_ticker(call("ETH-9D-C", now + day(9.0), "0.10"));
        // the nearest in-band expiry, with three strikes to choose between
        market.insert_ticker(call("ETH-6D-2500-C", now + day(6.0), "0.30"));
        market.insert_ticker(call("ETH-6D-3000-C", now + day(6.0), "0.12"));
        market.insert_ticker(call("ETH-6D-3500-C", now + day(6.0), "0.04"));

        let selected = params().select_call(&market, now).unwrap();
        assert_eq!(selected.instrument_name, "ETH-6D-3000-C");
    }

    #[test]
    fn an_empty_tenor_band_is_no_trade() {
        let now = chrono::Utc::now().timestamp();
        let mut market = MarketData::new();
        market.insert_ticker(call("ETH-2D-C", now + day(2.0), "0.10"));
        market.insert_ticker(call("ETH-30D-C", now + day(30.0), "0.10"));
        // never fall back to an out-of-band expiry: an uncontrolled tenor is an uncontrolled vega
        let err = params().select_call(&market, now).unwrap_err();
        assert!(err.to_string().contains("no live C within the tenor band"), "{err}");
    }

    #[test]
    fn selection_rejects_strikes_beyond_the_delta_tolerance() {
        let now = chrono::Utc::now().timestamp();
        let mut market = MarketData::new();
        // nearest to 0.10 on a thin chain, but 0.25 away from it
        market.insert_ticker(call("ETH-7D-DEEP-C", now + day(7.0), "0.35"));
        let err = params().select_call(&market, now).unwrap_err();
        assert!(err.to_string().contains("within 0.10 of 0.10 delta"), "{err}");
    }

    #[test]
    fn selection_rejects_untradeable_books() {
        let now = chrono::Utc::now().timestamp();
        let expiry = now + day(7.0);
        let p = params();

        // one sided: no ask
        let mut market = MarketData::new();
        market.insert_ticker(priced_call("ETH-7D-A-C", expiry, "0.10", "9.0", "0", "10.0"));
        assert!(p.select_call(&market, now).is_err());

        // collapsed bid: 0.70 of mark, below the 0.75 floor
        let mut market = MarketData::new();
        market.insert_ticker(priced_call("ETH-7D-B-C", expiry, "0.10", "7.0", "11.0", "10.0"));
        assert!(p.select_call(&market, now).is_err());

        // exactly at the floor is acceptable
        let mut market = MarketData::new();
        market.insert_ticker(priced_call("ETH-7D-C-C", expiry, "0.10", "7.5", "11.0", "10.0"));
        assert_eq!(p.select_call(&market, now).unwrap().instrument_name, "ETH-7D-C-C");
    }

    #[test]
    fn a_stale_ticker_is_not_selectable() {
        let now = chrono::Utc::now().timestamp();
        let mut market = MarketData::new();
        let mut stale = call("ETH-7D-C", now + day(7.0), "0.10");
        stale.timestamp = chrono::Utc::now().timestamp_millis() - 10_000;
        market.insert_ticker(stale);
        assert!(params().select_call(&market, now).is_err());
    }

    #[test]
    fn buying_back_gates_on_the_ask_not_the_bid() {
        let now = chrono::Utc::now().timestamp();
        let p = params();
        // ask blown out to 1.30 of mark: do not lift it
        let blown = priced_call("ETH-7D-C", now + day(7.0), "0.10", "9.0", "13.0", "10.0");
        assert!(!p.passes_liquidity_gates(&blown, Direction::Buy));
        // but it is still fine to sell into that book
        assert!(p.passes_liquidity_gates(&blown, Direction::Sell));
        // and at 1.25 exactly the buy is allowed
        let edge = priced_call("ETH-7D-C", now + day(7.0), "0.10", "9.0", "12.5", "10.0");
        assert!(p.passes_liquidity_gates(&edge, Direction::Buy));
    }

    #[test]
    fn sizing_follows_the_spot_collateral() {
        let now = chrono::Utc::now().timestamp();
        let ticker = call("ETH-7D-C", now + day(7.0), "0.10");
        let p = params();
        let mut market = MarketData::new();

        // nothing held: nothing to write against
        assert!(p.target_contracts(&market, &ticker).is_err());

        // 3 ETH of collateral at a 1.0 ratio writes 3 contracts
        market.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("3"),
            timestamp: 0,
        });
        assert_eq!(p.target_contracts(&market, &ticker).unwrap(), decimal("3.0"));

        // the cap binds once the stack grows past it
        market.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("40"),
            timestamp: 0,
        });
        assert_eq!(p.target_contracts(&market, &ticker).unwrap(), decimal("5.0"));

        // a partial ratio writes less than the stack, floored to the amount step
        let mut half = params();
        half.leg.notional_ratio = decimal("0.5");
        half.leg.max_contracts = decimal("100");
        assert_eq!(half.target_contracts(&market, &ticker).unwrap(), decimal("20.0"));
        let mut odd = half.clone();
        odd.leg.notional_ratio = decimal("0.031");
        assert_eq!(odd.target_contracts(&market, &ticker).unwrap(), decimal("1.2"));

        // a dust balance cannot reach the instrument's minimum amount
        let mut dust = MarketData::new();
        dust.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("0.01"),
            timestamp: 0,
        });
        assert!(p.target_contracts(&dust, &ticker).is_err());

        // and a nonsensical ratio or cap is refused outright
        let mut zero_ratio = params();
        zero_ratio.leg.notional_ratio = decimal("0");
        assert!(zero_ratio.target_contracts(&market, &ticker).is_err());
        let mut zero_cap = params();
        zero_cap.leg.max_contracts = decimal("0");
        assert!(zero_cap.target_contracts(&market, &ticker).is_err());
    }

    #[test]
    fn amounts_top_up_a_short_and_stop_at_the_target() {
        let now = chrono::Utc::now().timestamp();
        let ticker = call("ETH-7D-C", now + day(7.0), "0.10");
        let p = params();
        let sell = ShortLegOrderStrategy {
            target: p.leg.clone(),
            auction_cfg: p.auction_cfg.clone(),
            order: ShortLegOrder::SellToTarget { target_contracts: decimal("5") },
        };
        assert_eq!(sell.desired_amount(&ticker, &decimal("0")), (Direction::Sell, decimal("5")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("-2")), (Direction::Sell, decimal("3")));
        // already at or beyond the target: nothing to do
        assert_eq!(sell.desired_amount(&ticker, &decimal("-5")), (Direction::Sell, decimal("0")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("-6")), (Direction::Sell, decimal("0")));

        let buy = ShortLegOrderStrategy {
            target: p.leg.clone(),
            auction_cfg: p.auction_cfg.clone(),
            order: ShortLegOrder::BuyBack,
        };
        assert_eq!(buy.desired_amount(&ticker, &decimal("-5")), (Direction::Buy, decimal("5")));
        assert_eq!(buy.desired_amount(&ticker, &decimal("0")), (Direction::Buy, decimal("0")));

        // and an untradeable book sizes to zero, so the hour is skipped and retried
        let collapsed = priced_call("ETH-7D-C", now + day(7.0), "0.10", "1.0", "11.0", "10.0");
        assert_eq!(sell.desired_amount(&collapsed, &decimal("0")).1, decimal("0"));
    }

    #[test]
    fn minimum_hold_depends_on_the_dte_at_open() {
        let p = params();
        // a weekly leg carries the 3-day hold
        assert_eq!(p.min_hold_sec(day(7.0)), day(3.0));
        assert_eq!(p.min_hold_sec(day(10.0)), day(3.0));
        // and anything opened at 14 DTE or beyond carries 5 days
        assert_eq!(p.min_hold_sec(day(14.0)), day(5.0));
        assert_eq!(p.min_hold_sec(day(30.0)), day(5.0));
    }

    #[test]
    fn live_leg_reads_the_position_expiry_and_open_time() {
        let now = chrono::Utc::now().timestamp();
        let expiry = now + day(5.0);
        let mut market = MarketData::new();
        assert_eq!(live_leg(&market, "ETH").unwrap(), None);

        market.insert_ticker(call("ETH-20260828-3000-C", expiry, "0.10"));
        market.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        // without trade history the open time is unknown
        let leg = live_leg(&market, "ETH").unwrap().unwrap();
        assert_eq!(leg.instrument_name, "ETH-20260828-3000-C");
        assert_eq!(leg.expiry, expiry);
        assert_eq!(leg.opened_at, None);
        assert_eq!(leg.dte_sec(now), day(5.0));

        // the earliest fill is the open, even out of order, and timestamps are milliseconds
        market.insert_trade(trade("ETH-20260828-3000-C", (now - day(2.0)) * 1000));
        market.insert_trade(trade("ETH-20260828-3000-C", (now - day(4.0)) * 1000));
        assert_eq!(live_leg(&market, "ETH").unwrap().unwrap().opened_at, Some(now - day(4.0)));

        // another currency's position is not ours to manage
        assert_eq!(live_leg(&market, "BTC").unwrap(), None);
    }

    #[test]
    fn a_long_call_position_is_refused() {
        let now = chrono::Utc::now().timestamp();
        let mut market = MarketData::new();
        market.insert_ticker(call("ETH-20260828-3000-C", now + day(5.0), "0.10"));
        market.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        assert!(live_leg(&market, "ETH").unwrap_err().to_string().contains("is long, not short"));
    }

    #[test]
    fn the_expiry_falls_back_to_the_instrument_name() {
        // 08:00 UTC on the expiry date, per the venue convention
        let expected = chrono::DateTime::parse_from_rfc3339("2026-08-28T08:00:00Z")
            .unwrap()
            .timestamp();
        assert_eq!(expiry_from_name("ETH-20260828-3500-C"), Some(expected));
        assert_eq!(expiry_from_name("ETH-PERP"), None);
        assert_eq!(expiry_from_name("nonsense"), None);
    }

    /// Every hold path must avoid building an auction, which needs env and a ws client and so
    /// panics here: these passing is the assertion that no order is attempted.
    #[tokio::test]
    async fn gate_off_holds_when_the_leg_is_inside_its_minimum_hold() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        writer.insert_ticker(call("ETH-20260828-3000-C", now + day(5.0), "0.10"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        // opened one day ago, so two days short of the three-day minimum hold
        writer.insert_trade(trade("ETH-20260828-3000-C", (now - day(1.0)) * 1000));
        drop(writer);

        let mut stage = params().get_action(&market, vec![false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[tokio::test]
    async fn gate_off_holds_inside_one_dte_even_past_the_minimum_hold() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        // 0.5 days to expiry: the thinnest book of the week, so ride to settlement
        writer.insert_ticker(call("ETH-20260828-3000-C", now + day(0.5), "0.10"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        writer.insert_trade(trade("ETH-20260828-3000-C", (now - day(6.0)) * 1000));
        drop(writer);

        let mut stage = params().get_action(&market, vec![false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[tokio::test]
    async fn gate_off_holds_when_the_open_time_is_unknown() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        writer.insert_ticker(call("ETH-20260828-3000-C", now + day(5.0), "0.10"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        drop(writer);

        // no trade history: refuse to exit rather than guess the hold
        let mut stage = params().get_action(&market, vec![false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[tokio::test]
    async fn gate_off_and_flat_does_nothing() {
        let market = new_market_state();
        let mut stage = params().get_action(&market, vec![false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
        // and an empty signal vector is an error, never an accidental entry
        assert!(params().get_action(&market, vec![]).await.is_err());
    }

    #[tokio::test]
    async fn gate_on_holds_when_the_short_is_already_at_target() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        writer.insert_ticker(call("ETH-20260828-3000-C", now + day(5.0), "0.10"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260828-3000-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        writer.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        drop(writer);

        let mut stage = params().get_action(&market, vec![true]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[test]
    fn the_iv_shock_starts_at_mark_and_stops_at_the_cap() {
        let p = params();
        let start = 1_784_156_400;
        // init_iv_spread 0.0: the first quote is at mark
        assert_eq!(p.iv_spread_at(start, start), 0.0);
        // then 0.005 per minute
        assert!((p.iv_spread_at(start, start + 600) - 0.05).abs() < 1e-12);
        assert!((p.iv_spread_at(start, start + 1800) - 0.15).abs() < 1e-12);
        // and never past the cap, however long it runs
        assert_eq!(p.iv_spread_at(start, start + 86400), p.auction_cfg.max_iv_spread);
        // a clock that runs backwards must not price aggressively
        assert_eq!(p.iv_spread_at(start, start - 600), p.auction_cfg.init_iv_spread);
    }

    #[test]
    fn selling_starts_at_mark_and_walks_down() {
        let p = params();
        let now = chrono::Utc::now().timestamp();
        // wide book, so the walk is never clamped by it: bid 0.01, ask 999
        let ticker = priced_call("ETH-7D-C", now + day(7.0), "0.10", "0.01", "999", "10.0");

        let at_start = p.desired_price_at(&ticker, Direction::Sell, now, now).unwrap();
        let after_10m = p.desired_price_at(&ticker, Direction::Sell, now, now + 600).unwrap();
        let after_30m = p.desired_price_at(&ticker, Direction::Sell, now, now + 1800).unwrap();
        // progressively cheaper, i.e. progressively more likely to be hit
        assert!(after_10m < at_start, "{after_10m} vs {at_start}");
        assert!(after_30m < after_10m, "{after_30m} vs {after_10m}");
        // The cap holds the shock, not the price: theta keeps working as the auction runs, so
        // compare two auctions at ONE instant — one that just hit the cap at 40 minutes, and one
        // that started ten hours earlier. Same shock, same price.
        let at_cap = p.desired_price_at(&ticker, Direction::Sell, now, now + 2400).unwrap();
        let long_past_cap =
            p.desired_price_at(&ticker, Direction::Sell, now - 36000, now + 2400).unwrap();
        assert_eq!(at_cap, long_past_cap);
    }

    #[test]
    fn buying_back_walks_the_other_way() {
        let p = params();
        let now = chrono::Utc::now().timestamp();
        let ticker = priced_call("ETH-7D-C", now + day(7.0), "0.10", "0.01", "999", "10.0");

        let at_start = p.desired_price_at(&ticker, Direction::Buy, now, now).unwrap();
        let after_30m = p.desired_price_at(&ticker, Direction::Buy, now, now + 1800).unwrap();
        // paying progressively more to get out
        assert!(after_30m > at_start, "{after_30m} vs {at_start}");
        // both sides start from the same mark
        let sell_at_start = p.desired_price_at(&ticker, Direction::Sell, now, now).unwrap();
        assert_eq!(at_start, sell_at_start);
    }

    #[test]
    fn the_walk_never_quotes_through_the_book() {
        let p = params();
        let now = chrono::Utc::now().timestamp();
        // a tight book around the theoretical price, so the walk starts inside it and only
        // reaches the touch once the shock has run
        let ticker = priced_call("ETH-7D-C", now + day(7.0), "0.10", "5.0", "5.8", "5.4");
        let at_start = p.desired_price_at(&ticker, Direction::Sell, now, now).unwrap();
        assert!(at_start > ticker.best_bid_price, "starts inside the book, not at the touch");

        // far past the cap, so the walk is as aggressive as it ever gets
        let sell = p.desired_price_at(&ticker, Direction::Sell, now, now + 36000).unwrap();
        assert_eq!(sell, ticker.best_bid_price, "selling stops at the bid");

        let buy = p.desired_price_at(&ticker, Direction::Buy, now, now + 36000).unwrap();
        assert_eq!(buy, ticker.best_ask_price, "buying stops at the ask");
    }

    #[test]
    fn an_expired_or_unpriced_instrument_has_no_price() {
        let p = params();
        let now = chrono::Utc::now().timestamp();
        let expired = priced_call("ETH-0D-C", now - 60, "0.10", "9.0", "11.0", "10.0");
        assert!(p.desired_price_at(&expired, Direction::Sell, now, now).is_err());

        let mut unpriced = priced_call("ETH-7D-C", now + day(7.0), "0.10", "9.0", "11.0", "10.0");
        unpriced.option_pricing = None;
        assert!(p.desired_price_at(&unpriced, Direction::Sell, now, now).is_err());
        unpriced.option_details = None;
        assert!(p.desired_price_at(&unpriced, Direction::Sell, now, now).is_err());
    }
}
