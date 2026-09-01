//! Weathervane: sells 5-delta weekly puts in an uptrend, 5-delta weekly calls in a downtrend,
//! and holds nothing in between (NEW_SPEC).
//!
//! Two independent sleeves off one signal, 30-day spot momentum with a +/-5% dead zone:
//!
//! - **bull sleeve**: `mom30 > +5%` for 6 consecutive hours and flat -> sell a put.
//! - **bear sleeve**: `mom30 < -5%` for 6 consecutive hours and flat -> sell a call.
//!
//! A sleeve writes its leg in one decision and never adds to it: once live, it is a hold until
//! the leg settles, even if the fill came up short of target. The auction's own 30 minutes are
//! the only partial-fill recovery.
//!
//! There is **no exit path of any kind**. Every leg runs to its 08:00 UTC settlement; the gate
//! going false only stops that sleeve re-opening. That is the whole strategy, so nothing here may
//! grow a defensive close.
//!
//! The sleeves are independent, so a leg opened before a regime flip overlaps the new one: for a
//! few days the vault can be short both a put and a call at full coverage. The spec measures that
//! at 3.6% of hours and accepts it (`peak_coverage: 2.0`, `overlap_allowed: true`).
//!
//! Single leg per sleeve, no ladder, so every order is a plain orderbook order and no RFQ is
//! involved. Execution (tenor and strike selection, sizing, the IV price walk) is shared with the
//! other short-premium books in [super::short_leg].

use crate::clickhouse::gate::{fetch_momentum_gate, MomentumGateParams};
use crate::clickhouse::ClickhouseClient;
use crate::market::{MarketData, MarketState};
use crate::shared::auction::{LimitOrderAuction, LimitOrderAuctionExecutor};
use crate::shared::stages::ExecutorStage;
use crate::signals::strategies::short_leg::{
    AuctionConfig, LegTarget, ShortLegOrder, ShortLegOrderStrategy,
};
use crate::signals::{Candidates, NoopStage, Selector, SignalStrategy};
use anyhow::{bail, Result};
use bigdecimal::{BigDecimal, Zero};
use lyra_client::actions::Direction;
use orderbook_types::types::rfqs::LegUnpriced;
use orderbook_types::types::tickers::result::InstrumentTicker;
use orderbook_types::types::tickers::OptionType;
use serde::Deserialize;
use std::sync::Arc;

/// Which sleeve a decision belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleeve {
    /// Uptrend: short puts.
    Bull,
    /// Downtrend: short calls.
    Bear,
}

impl Sleeve {
    pub fn option_type(&self) -> OptionType {
        match self {
            Self::Bull => OptionType::P,
            Self::Bear => OptionType::C,
        }
    }
    /// The instrument-name suffix a leg of this sleeve carries.
    fn name_suffix(&self) -> &'static str {
        match self {
            Self::Bull => "-P",
            Self::Bear => "-C",
        }
    }
}

/// The signal vector this strategy is driven by, in order.
///
/// The usual "on when every value is true" convention does not fit two independent sleeves, so
/// the order is fixed here and nowhere else: **index 0 is the bull side, index 1 the bear side**.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WeathervaneSignals {
    pub bull: bool,
    pub bear: bool,
}

impl WeathervaneSignals {
    pub fn from_signals(signals: &[bool]) -> Result<Self> {
        match signals {
            [bull, bear] => Ok(Self { bull: *bull, bear: *bear }),
            _ => bail!("expected [bull, bear] signals, got {} values", signals.len()),
        }
    }

    pub fn to_vec(self) -> Vec<bool> {
        vec![self.bull, self.bear]
    }

    /// The sleeve wanting to open this hour, if any. Both sides on is impossible from one
    /// momentum reading, and is treated as no signal rather than as two trades.
    pub fn active_sleeve(&self) -> Option<Sleeve> {
        match (self.bull, self.bear) {
            (true, false) => Some(Sleeve::Bull),
            (false, true) => Some(Sleeve::Bear),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeathervaneParams {
    pub option_currency: String,
    /// Tenor, strike and sizing of each sleeve's leg. Both sleeves write the same delta.
    pub leg: LegTarget,
    /// How the auction walks its limit price.
    pub auction_cfg: AuctionConfig,

    pub gate: MomentumGateParams,
}

impl WeathervaneParams {
    /// The live short leg of a sleeve, if it holds one.
    ///
    /// The sleeves are keyed by option type: a put position belongs to the bull sleeve and a call
    /// position to the bear sleeve, so both can be live at once without being confused.
    pub fn live_leg(
        &self,
        market: &MarketData,
        sleeve: Sleeve,
    ) -> Result<Option<(String, BigDecimal)>> {
        let prefix = format!("{}-", self.option_currency);
        let positions = market
            .iter_positions()
            .filter(|position| {
                !position.amount.is_zero()
                    && position.instrument_name.starts_with(&prefix)
                    && position.instrument_name.ends_with(sleeve.name_suffix())
            })
            .collect::<Vec<_>>();

        let position = match positions.as_slice() {
            [] => return Ok(None),
            [position] => *position,
            _ => bail!(
                "expected at most one open {:?} position, found {}",
                sleeve,
                positions.len()
            ),
        };
        if position.amount > BigDecimal::zero() {
            bail!(
                "cannot manage {}: position {} is long, not short",
                position.instrument_name,
                position.amount
            );
        }
        Ok(Some((position.instrument_name.clone(), position.amount.clone())))
    }

    /// The leg the given sleeve would open now, or None when it must not open one.
    ///
    /// A sleeve holds one position at a time and never adds to it: if it is live, this hour is a
    /// hold whatever the size looks like against target (spec `bull_entry` / `bear_entry`: "gate
    /// true (after persistence) and sleeve flat -> sell"). A short fill therefore rides to expiry
    /// rather than being completed hours later at a drifted delta — the auction gets its own 30
    /// minutes to complete the size, and that is the only recovery there is.
    fn leg_to_open(
        &self,
        market: &MarketData,
        sleeve: Sleeve,
        now: i64,
    ) -> Result<Option<InstrumentTicker>> {
        if let Some((instrument_name, amount)) = self.live_leg(market, sleeve)? {
            log::info!(
                "Weathervane {:?} sleeve is short {} of {}, not re-entering",
                sleeve,
                amount,
                instrument_name
            );
            return Ok(None);
        }
        // nothing selectable is a skipped hour, not a failure: the spec's band may simply be
        // empty, or every strike in it may fail the delta and liquidity gates
        let eligible = self.eligible(market, now)?;
        match self.leg.select_leg(&eligible, sleeve.option_type(), now) {
            Ok(ticker) => Ok(Some(ticker)),
            Err(e) => {
                log::info!(
                    "Weathervane {:?} sleeve found nothing to sell this hour: {:#}",
                    sleeve,
                    e
                );
                Ok(None)
            }
        }
    }
}

impl Selector for WeathervaneParams {
    /// Both types: the bull sleeve sells puts and the bear sleeve calls, and either may need a
    /// quote on any given hour.
    fn candidates(&self) -> Result<Candidates> {
        let (min_expiry_sec, max_expiry_sec) = self.leg.tenor_band_sec()?;
        Ok(Candidates {
            currency: self.option_currency.clone(),
            min_expiry_sec,
            max_expiry_sec,
            option_types: vec![OptionType::C, OptionType::P],
        })
    }

    /// The structure the bull sleeve would open. Selection is per sleeve, so this reports the
    /// uptrend leg; [WeathervaneParams::leg_to_write] is what a decision actually uses.
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        let eligible = self.eligible(market, decision_at)?;
        let ticker = self.leg.select_leg(&eligible, Sleeve::Bull.option_type(), decision_at)?;
        Ok(vec![LegUnpriced {
            instrument_name: ticker.instrument_name.clone(),
            direction: Direction::Sell,
            amount: self.leg.target_contracts(market, &ticker)?,
        }])
    }
}

impl WeathervaneParams {
    /// One hourly tick. There is no exit path: a sleeve either opens (or tops up) its leg while
    /// its side of the gate is on, or holds. Settlement needs no action — a settled option simply
    /// leaves the subaccount, which returns that sleeve to flat.
    pub async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>> {
        let signals = WeathervaneSignals::from_signals(&signals)?;
        let now = chrono::Utc::now().timestamp();
        let Some(sleeve) = signals.active_sleeve() else {
            log::info!("Weathervane is in the dead zone, holding");
            return Ok(Box::new(NoopStage));
        };

        let reader = market.read().await;
        let Some(ticker) = self.leg_to_open(&reader, sleeve, now)? else {
            return Ok(Box::new(NoopStage));
        };

        // size the order before building an auction: an auction logs in and subscribes, which is
        // wasted on every decision that turns out to have nothing to trade
        let target_contracts = match self.leg.target_contracts(&reader, &ticker) {
            Ok(contracts) => contracts,
            Err(e) => {
                log::info!(
                    "Weathervane {:?} sleeve cannot size a leg this hour: {:#}",
                    sleeve,
                    e
                );
                return Ok(Box::new(NoopStage));
            }
        };
        let strategy = ShortLegOrderStrategy {
            target: self.leg.clone(),
            auction_cfg: self.auction_cfg.clone(),
            order: ShortLegOrder::SellToTarget { target_contracts },
        };
        let position = reader.get_amount(&ticker.instrument_name);
        let (_, amount) = strategy.desired_amount(&ticker, &position);
        drop(reader);

        if amount.is_zero() {
            log::info!(
                "Weathervane {:?} sleeve has nothing to trade on {}, holding",
                sleeve,
                ticker.instrument_name
            );
            return Ok(Box::new(NoopStage));
        }

        log::info!(
            "Weathervane {:?} sleeve selling {} of {}",
            sleeve,
            amount,
            ticker.instrument_name
        );
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

/// Weathervane wired to its live signal source.
#[derive(Clone)]
pub struct WeathervaneVault {
    pub params: WeathervaneParams,
    pub clickhouse: Arc<ClickhouseClient>,
}

impl std::fmt::Debug for WeathervaneVault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WeathervaneVault").field("params", &self.params).finish()
    }
}

impl Selector for WeathervaneVault {
    fn candidates(&self) -> Result<Candidates> {
        self.params.candidates()
    }
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        self.params.select_structure(market, decision_at)
    }
}

#[async_trait::async_trait]
impl SignalStrategy for WeathervaneVault {
    fn name(&self) -> &str {
        "weathervane"
    }

    /// `[bull, bear]`: 30-day momentum above / below the dead zone, each after its persistence.
    async fn signals(&self, decision_at: i64) -> Result<Vec<bool>> {
        let gate = fetch_momentum_gate(&self.clickhouse, &self.params.gate, decision_at).await?;
        Ok(WeathervaneSignals { bull: gate.bull, bear: gate.bear }.to_vec())
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
    use orderbook_types::types::tickers::result::{
        OptionPricingSchema, OptionPricingSlimSchema, OptionPublicDetailsSchema,
    };
    use orderbook_types::types::tickers::InstrumentType;
    use std::str::FromStr;

    fn decimal(value: &str) -> BigDecimal {
        BigDecimal::from_str(value).unwrap()
    }

    fn day(days: f64) -> i64 {
        (days * 86400.0) as i64
    }

    fn params() -> WeathervaneParams {
        WeathervaneParams {
            option_currency: "ETH".to_owned(),
            leg: LegTarget {
                spot_currency: "ETH".to_owned(),
                notional_ratio: decimal("1.0"),
                max_contracts: decimal("5"),
                target_delta: decimal("0.05"),
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
            gate: MomentumGateParams::weathervane("ETH"),
        }
    }

    /// A tradeable option: two-sided, bid at 90% of mark. `delta` is signed, as the venue reports.
    fn option(
        name: &str,
        expiry: i64,
        option_type: OptionType,
        delta: &str,
        strike: &str,
    ) -> InstrumentTicker {
        let mut pricing: OptionPricingSchema = OptionPricingSlimSchema::default().into();
        pricing.delta = decimal(delta);
        pricing.iv = decimal("0.50");
        pricing.forward_price = decimal("2000");
        InstrumentTicker {
            amount_step: decimal("0.1"),
            base_currency: "ETH".to_owned(),
            best_bid_price: decimal("9.0"),
            best_ask_price: decimal("11.0"),
            index_price: decimal("2000"),
            instrument_name: name.to_owned(),
            instrument_type: InstrumentType::Option,
            is_active: true,
            mark_price: decimal("10.0"),
            max_price: decimal("10000"),
            min_price: decimal("0.01"),
            minimum_amount: decimal("0.1"),
            option_details: Some(OptionPublicDetailsSchema {
                expiry,
                index: "ETH-USD".to_owned(),
                option_type,
                settlement_price: None,
                strike: decimal(strike),
            }),
            option_pricing: Some(pricing),
            tick_size: decimal("0.01"),
            timestamp: chrono::Utc::now().timestamp_millis(),
            ..InstrumentTicker::default()
        }
    }

    #[test]
    fn the_signal_vector_is_bull_then_bear() {
        let bull = WeathervaneSignals::from_signals(&[true, false]).unwrap();
        assert_eq!(bull, WeathervaneSignals { bull: true, bear: false });
        assert_eq!(bull.active_sleeve(), Some(Sleeve::Bull));
        assert_eq!(Sleeve::Bull.option_type(), OptionType::P, "uptrend sells puts");

        let bear = WeathervaneSignals::from_signals(&[false, true]).unwrap();
        assert_eq!(bear.active_sleeve(), Some(Sleeve::Bear));
        assert_eq!(Sleeve::Bear.option_type(), OptionType::C, "downtrend sells calls");

        // the dead zone, and the impossible both-sides case, are both "no signal"
        assert_eq!(WeathervaneSignals::from_signals(&[false, false]).unwrap().active_sleeve(), None);
        assert_eq!(WeathervaneSignals::from_signals(&[true, true]).unwrap().active_sleeve(), None);
        // and the vector must be exactly two values, never silently reinterpreted
        assert!(WeathervaneSignals::from_signals(&[]).is_err());
        assert!(WeathervaneSignals::from_signals(&[true]).is_err());
        assert!(WeathervaneSignals::from_signals(&[true, false, true]).is_err());
        assert_eq!(WeathervaneSignals { bull: true, bear: false }.to_vec(), vec![true, false]);
    }

    #[test]
    fn both_option_types_are_quoted() {
        let candidates = params().candidates().unwrap();
        assert_eq!(candidates.currency, "ETH");
        assert_eq!(candidates.min_expiry_sec, day(4.0));
        assert_eq!(candidates.max_expiry_sec, day(10.0));
        // either sleeve may need a quote on any hour, so both types must be live
        assert!(candidates.option_types.contains(&OptionType::C));
        assert!(candidates.option_types.contains(&OptionType::P));
    }

    #[test]
    fn selection_picks_the_right_type_at_five_delta() {
        let now = chrono::Utc::now().timestamp();
        let p = params();
        let mut market = MarketData::new();
        // puts carry a negative delta, so selection must compare magnitudes
        market.insert_ticker(option("ETH-7D-1800-P", now + day(7.0), OptionType::P, "-0.05", "1800"));
        market.insert_ticker(option("ETH-7D-1900-P", now + day(7.0), OptionType::P, "-0.12", "1900"));
        market.insert_ticker(option("ETH-7D-2200-C", now + day(7.0), OptionType::C, "0.06", "2200"));
        market.insert_ticker(option("ETH-7D-2100-C", now + day(7.0), OptionType::C, "0.20", "2100"));
        // out of band: never selectable
        market.insert_ticker(option("ETH-2D-1800-P", now + day(2.0), OptionType::P, "-0.05", "1800"));

        let eligible = p.eligible(&market, now).unwrap();
        let put = p.leg.select_leg(&eligible, OptionType::P, now).unwrap();
        assert_eq!(put.instrument_name, "ETH-7D-1800-P");
        let call = p.leg.select_leg(&eligible, OptionType::C, now).unwrap();
        assert_eq!(call.instrument_name, "ETH-7D-2200-C");
    }

    #[test]
    fn the_tenor_nearest_seven_days_wins() {
        let now = chrono::Utc::now().timestamp();
        let p = params();
        let mut market = MarketData::new();
        market.insert_ticker(option("ETH-5D-P", now + day(5.0), OptionType::P, "-0.05", "1800"));
        market.insert_ticker(option("ETH-6D-P", now + day(6.0), OptionType::P, "-0.05", "1800"));
        market.insert_ticker(option("ETH-10D-P", now + day(10.0), OptionType::P, "-0.05", "1800"));
        let eligible = p.eligible(&market, now).unwrap();
        assert_eq!(
            p.leg.select_leg(&eligible, OptionType::P, now).unwrap().instrument_name,
            "ETH-6D-P"
        );
    }

    #[test]
    fn sleeves_are_tracked_separately_by_option_type() {
        let p = params();
        let mut market = MarketData::new();
        assert_eq!(p.live_leg(&market, Sleeve::Bull).unwrap(), None);
        assert_eq!(p.live_leg(&market, Sleeve::Bear).unwrap(), None);

        // the overlap case the spec accepts: short a put and a call at once
        market.insert_position(Balance {
            instrument_name: "ETH-20260904-1800-P".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        market.insert_position(Balance {
            instrument_name: "ETH-20260904-2200-C".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        assert_eq!(
            p.live_leg(&market, Sleeve::Bull).unwrap(),
            Some(("ETH-20260904-1800-P".to_owned(), decimal("-5")))
        );
        assert_eq!(
            p.live_leg(&market, Sleeve::Bear).unwrap(),
            Some(("ETH-20260904-2200-C".to_owned(), decimal("-5")))
        );

        // a long position is refused rather than managed
        let mut long = MarketData::new();
        long.insert_position(Balance {
            instrument_name: "ETH-20260904-1800-P".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        assert!(p.live_leg(&long, Sleeve::Bull).unwrap_err().to_string().contains("is long"));
    }

    /// Every hold path must avoid building an auction, which needs env and a ws client and so
    /// panics here: these passing is the assertion that no order is attempted.
    #[tokio::test]
    async fn the_dead_zone_trades_nothing() {
        let market = new_market_state();
        let mut stage = params().get_action(&market, vec![false, false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
        // a malformed vector is an error, never an accidental trade
        assert!(params().get_action(&market, vec![]).await.is_err());
        assert!(params().get_action(&market, vec![true]).await.is_err());
    }

    #[tokio::test]
    async fn a_gate_flip_does_not_close_the_other_sleeve() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        // short a put from the bull sleeve, already at target
        writer.insert_ticker(option("ETH-20260904-1800-P", now + day(7.0), OptionType::P, "-0.05", "1800"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260904-1800-P".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        drop(writer);

        // the gate has flipped to bear, but there is no exit path: the put is left alone, and the
        // bear sleeve finds no call to sell because none is quoted here
        let mut stage = params().get_action(&market, vec![false, true]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();

        // and back in the dead zone the put is still not touched
        let mut stage = params().get_action(&market, vec![false, false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    /// The spec's all-or-nothing rule: if no leg passes the gates this hour, skip and retry
    /// next hour rather than failing the decision loop.
    #[tokio::test]
    async fn an_unselectable_hour_is_skipped_not_an_error() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        // in band and the right type, but the bid has collapsed to 30% of mark
        let mut collapsed =
            option("ETH-7D-1800-P", now + day(7.0), OptionType::P, "-0.05", "1800");
        collapsed.best_bid_price = decimal("3.0");
        writer.insert_ticker(collapsed);
        drop(writer);

        let mut stage = params().get_action(&market, vec![true, false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[tokio::test]
    async fn a_sleeve_at_target_holds_rather_than_reopening() {
        let now = chrono::Utc::now().timestamp();
        let market = new_market_state();
        let mut writer = market.write().await;
        writer.insert_ticker(option("ETH-20260904-1800-P", now + day(7.0), OptionType::P, "-0.05", "1800"));
        writer.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        writer.insert_position(Balance {
            instrument_name: "ETH-20260904-1800-P".to_owned(),
            amount: decimal("-5"),
            timestamp: 0,
        });
        drop(writer);

        let mut stage = params().get_action(&market, vec![true, false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    /// A live sleeve never re-enters, even when its fill looks short of target: a position that
    /// came up light rides to expiry rather than being completed at a later, drifted delta.
    #[tokio::test]
    async fn a_live_sleeve_never_reenters_even_when_short_of_target() {
        let now = chrono::Utc::now().timestamp();
        let p = params();
        let market = new_market_state();
        let mut writer = market.write().await;
        // 5 ETH of collateral targets 5 contracts, but the sleeve only holds 2
        writer.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        writer.insert_ticker(option("ETH-20260904-1700-P", now + day(7.0), OptionType::P, "-0.02", "1700"));
        writer.insert_ticker(option("ETH-20260904-1800-P", now + day(7.0), OptionType::P, "-0.05", "1800"));
        writer.insert_position(Balance {
            instrument_name: "ETH-20260904-1700-P".to_owned(),
            amount: decimal("-2"),
            timestamp: 0,
        });
        drop(writer);

        let reader = market.read().await;
        assert!(
            p.leg_to_open(&reader, Sleeve::Bull, now).unwrap().is_none(),
            "a live sleeve must not re-enter, however partial it looks"
        );
        drop(reader);
        // end to end: the gate is on and the sleeve is under target, and still nothing trades
        // (building an auction here would panic on the missing env)
        let mut stage = p.get_action(&market, vec![true, false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();

        // once flat, the same hour would open the fresh 5-delta selection
        let mut writer = market.write().await;
        writer.clear_subaccount();
        writer.insert_position(Balance {
            instrument_name: "ETH".to_owned(),
            amount: decimal("5"),
            timestamp: 0,
        });
        drop(writer);
        let reader = market.read().await;
        let leg = p.leg_to_open(&reader, Sleeve::Bull, now).unwrap().unwrap();
        assert_eq!(leg.instrument_name, "ETH-20260904-1800-P");
    }
}
