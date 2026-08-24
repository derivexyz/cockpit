use crate::market::MarketData;
use anyhow::Result;
use orderbook_types::types::rfqs::LegUnpriced;

/// Selects the option structure a strategy wants to trade.
///
/// Selection rules belong to the concrete strategy, not the shared runner.
/// A strategy should own its selector and call it from `get_action` when it
/// needs a new structure.
pub trait Selector: Send {
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>>;
}

impl<T> Selector for Box<T>
where
    T: Selector + ?Sized,
{
    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>> {
        (**self).select_structure(market, decision_at)
    }
}

#[cfg(test)]
use bigdecimal::BigDecimal;
#[cfg(test)]
use orderbook_types::types::orders::Direction;

/// Deterministic selector for strategy and runner tests.
#[cfg(test)]
pub struct MockSelector {
    direction: Direction,
    amount: BigDecimal,
}

#[cfg(test)]
impl MockSelector {
    pub fn new(direction: Direction, amount: BigDecimal) -> Self {
        Self { direction, amount }
    }
}

#[cfg(test)]
impl Selector for MockSelector {
    fn select_structure(&self, market: &MarketData, _decision_at: i64) -> Result<Vec<LegUnpriced>> {
        let mut tickers = market.iter_tickers().collect::<Vec<_>>();
        tickers.sort_by(|left, right| left.instrument_name.cmp(&right.instrument_name));
        let ticker = tickers
            .first()
            .ok_or_else(|| anyhow::anyhow!("cannot select a structure without tickers"))?;

        Ok(vec![LegUnpriced {
            instrument_name: ticker.instrument_name.clone(),
            direction: self.direction,
            amount: self.amount.clone(),
        }])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orderbook_types::types::tickers::InstrumentTicker;

    #[test]
    fn mock_selector_returns_the_first_ticker_by_name() {
        let mut market = MarketData::new();
        market.insert_ticker(InstrumentTicker {
            instrument_name: "ETH-Z".into(),
            ..InstrumentTicker::default()
        });
        market.insert_ticker(InstrumentTicker {
            instrument_name: "ETH-A".into(),
            ..InstrumentTicker::default()
        });

        let selector = MockSelector::new(Direction::Sell, BigDecimal::from(2));
        let legs = selector.select_structure(&market, 0).unwrap();

        assert_eq!(legs.len(), 1);
        assert_eq!(legs[0].instrument_name, "ETH-A");
        assert_eq!(legs[0].direction, Direction::Sell);
        assert_eq!(legs[0].amount, BigDecimal::from(2));
    }
}
