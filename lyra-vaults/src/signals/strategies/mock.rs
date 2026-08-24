use crate::market::{MarketData, MarketState};
use crate::shared::auction::{LimitOrderAuction, LimitOrderAuctionExecutor, OrderStrategy};
use crate::shared::stages::ExecutorStage;
use crate::signals::{Selector, SignalStrategy};
use anyhow::{bail, Context, Error, Result};
use bigdecimal::{BigDecimal, FromPrimitive, RoundingMode, ToPrimitive, Zero};
use log::{info, warn};
use lyra_client::actions::Direction;
use lyra_utils::black76::OptionContract;
use orderbook_types::types::rfqs::LegUnpriced;
use orderbook_types::types::tickers::result::InstrumentTicker;
use orderbook_types::types::tickers::OptionType;
use serde::Deserialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const MOCK_SIGNAL_POLL_INTERVAL: Duration = Duration::from_secs(60 * 60);

/// How long the mock signal stays on, and then off.
pub const MOCK_SIGNAL_PHASE_SEC: i64 = 2 * 60 * 60;

/// Returns one mock signal that alternates between on (`true`) and off (`false`) every
/// [MOCK_SIGNAL_PHASE_SEC].
///
/// The phases are aligned to the UTC epoch, and a day divides evenly into an even number of
/// phases, so every UTC day starts with an on phase: on 00:00-02:00, off 02:00-04:00, and so on.
pub async fn mock_signals(now: i64) -> Result<Vec<bool>> {
    let phase = now.div_euclid(MOCK_SIGNAL_PHASE_SEC);
    Ok(vec![phase.rem_euclid(2) == 0])
}

/// Parameters for the mock signal-driven covered-call strategy.
#[derive(Clone, Debug, Deserialize)]
pub struct MockCCParams {
    pub option_currency: String,
    pub expiry_days: u64,
    pub min_expiry_hours: u64,
    pub target_delta: BigDecimal,
    pub max_delta: BigDecimal,
    /// Desired underlying notional, denominated in USD.
    pub target_notional: BigDecimal,

    pub max_iv_spread: f64,
    pub init_iv_spread: f64,
    pub iv_spread_per_min: f64,
    pub auction_sec: i64,
    pub price_change_tolerance: BigDecimal,
}

impl MockCCParams {
    pub fn expiry_sec(&self) -> Result<i64> {
        i64::try_from(self.expiry_days)
            .ok()
            .and_then(|days| days.checked_mul(24 * 60 * 60))
            .context("expiry_days is too large")
    }

    pub fn min_expiry_sec(&self) -> Result<i64> {
        i64::try_from(self.min_expiry_hours)
            .ok()
            .and_then(|hours| hours.checked_mul(60 * 60))
            .context("min_expiry_hours is too large")
    }

    fn iv_spread_at(&self, start_timestamp_sec: i64, now: i64) -> f64 {
        let sec_since_start = now.saturating_sub(start_timestamp_sec).max(0);
        let min_since_start = sec_since_start as f64 / 60.0;
        let spread = self.init_iv_spread + min_since_start * self.iv_spread_per_min;
        spread.min(self.max_iv_spread)
    }

    fn target_contracts(&self, ticker: &InstrumentTicker) -> Result<BigDecimal> {
        if self.target_notional <= BigDecimal::zero() {
            bail!("target_notional must be positive");
        }
        if ticker.index_price <= BigDecimal::zero() {
            bail!("{} has a non-positive index price", ticker.instrument_name);
        }

        let amount = (&self.target_notional / &ticker.index_price)
            .with_scale_round(ticker.amount_step.fractional_digit_count(), RoundingMode::Down);
        if amount < ticker.minimum_amount {
            bail!(
                "target notional produces {} contracts for {}, below its minimum amount {}",
                amount,
                ticker.instrument_name,
                ticker.minimum_amount
            );
        }
        Ok(amount)
    }

    fn desired_price_at(
        &self,
        ticker: &InstrumentTicker,
        direction: Direction,
        start_timestamp_sec: i64,
        now: i64,
    ) -> Result<BigDecimal> {
        let details =
            ticker.option_details.as_ref().ok_or_else(|| Error::msg("option details not found"))?;
        let pricing =
            ticker.option_pricing.as_ref().ok_or_else(|| Error::msg("option pricing not found"))?;
        let mark_iv = pricing.iv.to_f64().context("IV cast to f64 failed")?;
        let spread = self.iv_spread_at(start_timestamp_sec, now);
        let iv = match direction {
            Direction::Sell => mark_iv * (1.0 - spread),
            Direction::Buy => mark_iv * (1.0 + spread),
        };
        let seconds_to_expiry = details.expiry - now;
        if seconds_to_expiry <= 0 {
            bail!("{} is expired", ticker.instrument_name);
        }

        let contract = OptionContract {
            strike: details.strike.to_f64().context("strike cast to f64 failed")?,
            expiry_sec: seconds_to_expiry as f64,
            is_call: details.option_type == OptionType::C,
        };
        let forward = pricing.forward_price.to_f64().context("forward cast to f64 failed")?;
        let price = contract.price(forward, iv);
        let price = BigDecimal::from_f64(price)
            .context("option price cannot be represented as a decimal")?
            .round(ticker.tick_size.fractional_digit_count())
            .max(ticker.min_price.clone());
        Ok(price)
    }
}

impl Selector for MockCCParams {
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        let min_expiry = decision_at
            .checked_add(self.min_expiry_sec()?)
            .context("minimum expiry timestamp overflowed")?;
        let max_expiry = decision_at
            .checked_add(self.expiry_sec()?)
            .context("maximum expiry timestamp overflowed")?;

        // fresh tickers only: a stale delta would silently misprice the selection
        let eligible = market
            .iter_fresh_tickers()
            .filter(|ticker| ticker.is_active && ticker.base_currency == self.option_currency)
            .filter(|ticker| {
                ticker.option_details.as_ref().is_some_and(|details| {
                    details.option_type == OptionType::C
                        && details.expiry > min_expiry
                        && details.expiry < max_expiry
                })
            })
            .collect::<Vec<_>>();

        let selected_expiry = eligible
            .iter()
            .filter_map(|ticker| ticker.option_details.as_ref().map(|details| details.expiry))
            .max()
            .ok_or_else(|| Error::msg("no active calls found within the mock CC expiry range"))?;

        let at_expiry = eligible
            .into_iter()
            .filter(|ticker| {
                ticker
                    .option_details
                    .as_ref()
                    .is_some_and(|details| details.expiry == selected_expiry)
            })
            .collect::<Vec<_>>();
        let at_expiry_count = at_expiry.len();

        let selected = at_expiry
            .into_iter()
            .filter(|ticker| {
                ticker.option_pricing.as_ref().is_some_and(|pricing| pricing.delta < self.max_delta)
            })
            .min_by(|left, right| {
                let left_distance =
                    (&left.option_pricing.as_ref().unwrap().delta - &self.target_delta).abs();
                let right_distance =
                    (&right.option_pricing.as_ref().unwrap().delta - &self.target_delta).abs();
                left_distance
                    .cmp(&right_distance)
                    .then_with(|| left.instrument_name.cmp(&right.instrument_name))
            })
            .ok_or_else(|| Error::msg("no calls found within the mock CC delta range"))?;

        info!(
            "MockCC selected {} with delta {} out of {} live calls expiring at {}",
            selected.instrument_name,
            selected.option_pricing.as_ref().unwrap().delta,
            at_expiry_count,
            selected_expiry
        );

        Ok(vec![LegUnpriced {
            instrument_name: selected.instrument_name.clone(),
            direction: Direction::Sell,
            amount: self.target_contracts(selected)?,
        }])
    }
}

#[derive(Clone, Debug)]
enum MockCCOrder {
    SellToTarget { target_contracts: BigDecimal },
    BuyBack,
}

impl MockCCOrder {
    fn direction(&self) -> Direction {
        match self {
            Self::SellToTarget { .. } => Direction::Sell,
            Self::BuyBack => Direction::Buy,
        }
    }
}

#[derive(Clone, Debug)]
struct MockCCOrderStrategy {
    params: MockCCParams,
    order: MockCCOrder,
}

impl MockCCOrderStrategy {
    fn desired_amount(
        &self,
        ticker: &InstrumentTicker,
        current_position: &BigDecimal,
    ) -> (Direction, BigDecimal) {
        let direction = self.order.direction();
        let amount = match &self.order {
            MockCCOrder::SellToTarget { target_contracts } => target_contracts + current_position,
            MockCCOrder::BuyBack => -current_position,
        };
        if amount <= BigDecimal::zero() {
            return (direction, BigDecimal::zero());
        }

        let amount = amount
            .with_scale_round(ticker.amount_step.fractional_digit_count(), RoundingMode::Down);
        if amount < ticker.minimum_amount {
            return (direction, BigDecimal::zero());
        }
        (direction, amount)
    }
}

#[async_trait::async_trait]
impl OrderStrategy for MockCCOrderStrategy {
    async fn get_desired_price(&self, auction: &LimitOrderAuction) -> Result<BigDecimal> {
        let reader = auction.market.read().await;
        let ticker = reader
            .get_ticker(&auction.instrument_name)
            .ok_or_else(|| Error::msg("ticker not found"))?;
        self.params.desired_price_at(
            ticker,
            self.order.direction(),
            auction.start_timestamp_sec,
            utc_now_seconds()?,
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

#[derive(Debug)]
struct NoopStage;

#[async_trait::async_trait]
impl ExecutorStage for NoopStage {
    async fn run(&self) -> Result<()> {
        Ok(())
    }

    async fn reconnect(&mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl SignalStrategy for MockCCParams {
    fn name(&self) -> &str {
        "mock-covered-call"
    }

    async fn get_action(
        &self,
        market: &MarketState,
        signals: Vec<bool>,
    ) -> Result<Box<dyn ExecutorStage>> {
        if signals.is_empty() {
            bail!("signal vector cannot be empty");
        }

        let sell_call = signals.iter().all(|signal| *signal);
        let now = utc_now_seconds()?;
        let reader = market.read().await;
        let current_call = current_call_position(&reader, &self.option_currency)?;

        let (instrument_name, order) = if sell_call {
            match current_call {
                Some((instrument_name, _)) => {
                    let Some(ticker) = reader.get_ticker(&instrument_name) else {
                        // no live feed, e.g. inside min_expiry_hours: hold until settlement
                        warn!("MockCC has no live ticker for {}, holding", instrument_name);
                        return Ok(Box::new(NoopStage));
                    };
                    if !is_tradeable_call(ticker, &self.option_currency, now) {
                        return Ok(Box::new(NoopStage));
                    }
                    let target_contracts = self.target_contracts(ticker)?;
                    (instrument_name, MockCCOrder::SellToTarget { target_contracts })
                }
                None => {
                    let mut structure = self.select_structure(&reader, now)?;
                    let leg = structure
                        .pop()
                        .ok_or_else(|| Error::msg("mock CC selector returned no legs"))?;
                    (
                        leg.instrument_name,
                        MockCCOrder::SellToTarget { target_contracts: leg.amount },
                    )
                }
            }
        } else {
            match current_call {
                Some((instrument_name, amount)) if amount < BigDecimal::zero() => {
                    let Some(ticker) = reader.get_ticker(&instrument_name) else {
                        warn!("MockCC has no live ticker for {}, holding", instrument_name);
                        return Ok(Box::new(NoopStage));
                    };
                    if !is_tradeable_call(ticker, &self.option_currency, now) {
                        return Ok(Box::new(NoopStage));
                    }
                    (instrument_name, MockCCOrder::BuyBack)
                }
                Some((instrument_name, amount)) => {
                    bail!(
                        "cannot buy back {instrument_name}: current position {amount} is not short"
                    );
                }
                None => return Ok(Box::new(NoopStage)),
            }
        };
        // decide the size before building an auction: an auction logs in and subscribes, which
        // is wasted every decision that turns out to have nothing to trade
        let strategy = MockCCOrderStrategy { params: self.clone(), order };
        let amount = {
            let ticker = reader
                .get_ticker(&instrument_name)
                .ok_or_else(|| Error::msg(format!("no live ticker for {instrument_name}")))?;
            let position = reader.get_amount(&instrument_name);
            strategy.desired_amount(ticker, &position).1
        };
        drop(reader);

        if amount.is_zero() {
            info!("MockCC has nothing to trade on {}, holding", instrument_name);
            return Ok(Box::new(NoopStage));
        }

        let auction = LimitOrderAuction::new(
            instrument_name,
            now,
            self.auction_sec,
            self.price_change_tolerance.clone(),
        )
        .await?;
        Ok(Box::new(LimitOrderAuctionExecutor { auction, strategy }))
    }
}

fn current_call_position(
    market: &MarketData,
    option_currency: &str,
) -> Result<Option<(String, BigDecimal)>> {
    let instrument_prefix = format!("{option_currency}-");
    let positions = market
        .iter_positions()
        .filter(|position| {
            !position.amount.is_zero()
                && position.instrument_name.starts_with(&instrument_prefix)
                && position.instrument_name.ends_with("-C")
        })
        .map(|position| (position.instrument_name.clone(), position.amount.clone()))
        .collect::<Vec<_>>();

    match positions.as_slice() {
        [] => Ok(None),
        [position] => Ok(Some(position.clone())),
        _ => bail!("expected at most one open call position, found {}", positions.len()),
    }
}

fn is_tradeable_call(ticker: &InstrumentTicker, option_currency: &str, now: i64) -> bool {
    ticker.is_active
        && ticker.base_currency == option_currency
        && ticker
            .option_details
            .as_ref()
            .is_some_and(|details| details.option_type == OptionType::C && details.expiry > now)
}

fn utc_now_seconds() -> Result<i64> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system clock is before the Unix epoch")?;
    i64::try_from(now.as_secs()).context("UTC timestamp is too large to represent in seconds")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::market::{new_market_state, Balance, MarketData};
    use orderbook_types::types::tickers::result::{
        OptionPricingSchema, OptionPricingSlimSchema, OptionPublicDetailsSchema,
    };
    use orderbook_types::types::tickers::InstrumentType;
    use std::str::FromStr;

    fn decimal(value: &str) -> BigDecimal {
        BigDecimal::from_str(value).unwrap()
    }

    fn params() -> MockCCParams {
        MockCCParams {
            option_currency: "ETH".to_owned(),
            expiry_days: 30,
            min_expiry_hours: 24,
            target_delta: decimal("0.30"),
            max_delta: decimal("0.50"),
            target_notional: decimal("10000"),
            max_iv_spread: 0.20,
            init_iv_spread: 0.01,
            iv_spread_per_min: 0.01,
            auction_sec: 30 * 60,
            price_change_tolerance: decimal("0.01"),
        }
    }

    fn option_ticker(
        name: &str,
        currency: &str,
        expiry: i64,
        option_type: OptionType,
        delta: &str,
        active: bool,
    ) -> InstrumentTicker {
        let mut pricing: OptionPricingSchema = OptionPricingSlimSchema::default().into();
        pricing.delta = decimal(delta);
        pricing.iv = decimal("0.50");
        pricing.forward_price = decimal("2000");

        InstrumentTicker {
            amount_step: decimal("0.1"),
            base_currency: currency.to_owned(),
            index_price: decimal("2000"),
            instrument_name: name.to_owned(),
            instrument_type: InstrumentType::Option,
            is_active: active,
            max_price: decimal("10000"),
            min_price: decimal("0.01"),
            minimum_amount: decimal("0.1"),
            option_details: Some(OptionPublicDetailsSchema {
                expiry,
                index: format!("{currency}-USD"),
                option_type,
                settlement_price: None,
                strike: decimal("2000"),
            }),
            option_pricing: Some(pricing),
            tick_size: decimal("0.01"),
            timestamp: chrono::Utc::now().timestamp_millis(),
            ..InstrumentTicker::default()
        }
    }

    /// Decisions must land inside a phase to observe it, so the poll interval cannot exceed it.
    #[tokio::test]
    async fn mock_signal_alternates_every_two_hours() {
        assert_eq!(MOCK_SIGNAL_POLL_INTERVAL, Duration::from_secs(60 * 60));
        assert!(MOCK_SIGNAL_POLL_INTERVAL.as_secs() <= MOCK_SIGNAL_PHASE_SEC as u64);

        let hour = 60 * 60;
        let midnight = 1_735_689_600; // 2025-01-01T00:00:00Z
        for (offset, expected) in [
            (0, true),
            (hour, true),
            (2 * hour - 1, true),
            (2 * hour, false),
            (3 * hour, false),
            (4 * hour, true),
            (6 * hour, false),
            (24 * hour, true),
        ] {
            assert_eq!(
                mock_signals(midnight + offset).await.unwrap(),
                vec![expected],
                "{} sec past midnight",
                offset
            );
        }

        // and the same alternation holds before the epoch
        assert_eq!(mock_signals(-1).await.unwrap(), vec![false]);
    }

    #[test]
    fn selector_uses_latest_expiry_then_nearest_delta() {
        let params = params();
        let now = 1_735_689_600;
        let earlier_expiry = now + 7 * 24 * 60 * 60;
        let latest_expiry = now + 14 * 24 * 60 * 60;
        let mut market = MarketData::new();

        market.insert_ticker(option_ticker(
            "ETH-EARLY-2000-C",
            "ETH",
            earlier_expiry,
            OptionType::C,
            "0.30",
            true,
        ));
        market.insert_ticker(option_ticker(
            "ETH-LATEST-1900-C",
            "ETH",
            latest_expiry,
            OptionType::C,
            "0.24",
            true,
        ));
        market.insert_ticker(option_ticker(
            "ETH-LATEST-2100-C",
            "ETH",
            latest_expiry,
            OptionType::C,
            "0.31",
            true,
        ));
        market.insert_ticker(option_ticker(
            "ETH-LATEST-2000-P",
            "ETH",
            latest_expiry,
            OptionType::P,
            "0.30",
            true,
        ));
        market.insert_ticker(option_ticker(
            "BTC-LATEST-2000-C",
            "BTC",
            latest_expiry,
            OptionType::C,
            "0.30",
            true,
        ));

        let structure = params.select_structure(&market, now).unwrap();
        assert_eq!(structure.len(), 1);
        assert_eq!(structure[0].instrument_name, "ETH-LATEST-2100-C");
        assert_eq!(structure[0].direction, Direction::Sell);
        assert_eq!(structure[0].amount, decimal("5.0"));
    }

    /// A ticker whose feed went quiet keeps its last delta, which must not drive the selection.
    #[test]
    fn selector_ignores_stale_tickers() {
        let params = params();
        let now = 1_735_689_600;
        let expiry = now + 14 * 24 * 60 * 60;
        let mut market = MarketData::new();

        let mut nearest_delta =
            option_ticker("ETH-LATEST-2900-C", "ETH", expiry, OptionType::C, "0.10", true);
        nearest_delta.timestamp = chrono::Utc::now().timestamp_millis() - 10_000;
        market.insert_ticker(nearest_delta.clone());
        market.insert_ticker(option_ticker(
            "ETH-LATEST-3500-C",
            "ETH",
            expiry,
            OptionType::C,
            "0.01",
            true,
        ));

        let structure = params.select_structure(&market, now).unwrap();
        assert_eq!(structure[0].instrument_name, "ETH-LATEST-3500-C");

        // and with every ticker stale there is nothing to select at all
        let mut market = MarketData::new();
        market.insert_ticker(nearest_delta);
        assert!(params.select_structure(&market, now).is_err());
    }

    #[test]
    fn amount_tracks_target_short_and_current_buyback_position() {
        let ticker = option_ticker("ETH-CALL-C", "ETH", 2_000_000_000, OptionType::C, "0.30", true);
        let sell = MockCCOrderStrategy {
            params: params(),
            order: MockCCOrder::SellToTarget { target_contracts: decimal("5") },
        };
        assert_eq!(sell.desired_amount(&ticker, &decimal("0")), (Direction::Sell, decimal("5")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("-2")), (Direction::Sell, decimal("3")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("-5")), (Direction::Sell, decimal("0")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("-6")), (Direction::Sell, decimal("0")));
        assert_eq!(sell.desired_amount(&ticker, &decimal("1")), (Direction::Sell, decimal("6")));

        let buy_back = MockCCOrderStrategy { params: params(), order: MockCCOrder::BuyBack };
        assert_eq!(
            buy_back.desired_amount(&ticker, &decimal("-2")),
            (Direction::Buy, decimal("2"))
        );
        assert_eq!(buy_back.desired_amount(&ticker, &decimal("1")), (Direction::Buy, decimal("0")));
    }

    #[test]
    fn sell_price_falls_and_buy_price_rises_as_iv_spread_grows() {
        let params = params();
        let now = 1_735_689_600;
        let ticker = option_ticker(
            "ETH-CALL-C",
            "ETH",
            now + 14 * 24 * 60 * 60,
            OptionType::C,
            "0.30",
            true,
        );

        let sell_early = params.desired_price_at(&ticker, Direction::Sell, now, now).unwrap();
        let sell_late =
            params.desired_price_at(&ticker, Direction::Sell, now, now + 10 * 60).unwrap();
        let buy_early = params.desired_price_at(&ticker, Direction::Buy, now, now).unwrap();
        let buy_late =
            params.desired_price_at(&ticker, Direction::Buy, now, now + 10 * 60).unwrap();

        assert!(sell_late < sell_early);
        assert!(buy_late > buy_early);
        assert!(buy_early > sell_early);
    }

    #[test]
    fn current_call_recovery_is_scoped_to_the_configured_currency() {
        let mut market = MarketData::new();
        market.insert_position(Balance {
            instrument_name: "BTC-20260101-100000-C".to_owned(),
            amount: decimal("-3"),
            timestamp: 0,
        });
        market.insert_position(Balance {
            instrument_name: "ETH-20260101-10000-C".to_owned(),
            amount: decimal("-2"),
            timestamp: 0,
        });

        assert_eq!(
            current_call_position(&market, "ETH").unwrap(),
            Some(("ETH-20260101-10000-C".to_owned(), decimal("-2")))
        );
        assert_eq!(
            current_call_position(&market, "BTC").unwrap(),
            Some(("BTC-20260101-100000-C".to_owned(), decimal("-3")))
        );
    }

    /// Building an auction needs env and a ws client, so a decision that reaches one panics here:
    /// this passing is the assertion that an at-target decision never gets that far.
    #[tokio::test]
    async fn on_signal_is_a_noop_when_already_at_target() {
        let market = new_market_state();
        let name = "ETH-20260828-2500-C";
        let ticker = option_ticker(name, "ETH", 2_000_000_000, OptionType::C, "0.10", true);
        // index 2000 against a 10000 target notional, i.e. 5 contracts
        let target = params().target_contracts(&ticker).unwrap();
        assert_eq!(target, decimal("5.0"));

        let mut writer = market.write().await;
        writer.insert_ticker(ticker);
        writer.insert_position(Balance {
            instrument_name: name.to_owned(),
            amount: -target,
            timestamp: 0,
        });
        drop(writer);

        let mut stage = params().get_action(&market, vec![true]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
    }

    #[tokio::test]
    async fn off_signal_is_a_noop_when_there_is_no_short_call() {
        let market = new_market_state();
        let mut stage = params().get_action(&market, vec![false]).await.unwrap();
        stage.run_with_reconnect().await.unwrap();
        assert!(params().get_action(&market, vec![]).await.is_err());
    }
}
