use crate::market::MarketData;
use anyhow::Result;
use orderbook_types::types::rfqs::LegUnpriced;
use orderbook_types::types::tickers::result::InstrumentTicker;
use orderbook_types::types::tickers::OptionType;

/// The universe a selection draws from: the instruments the runtime must have live quotes for.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidates {
    pub currency: String,
    /// The tenor window a decision may select within, as seconds to expiry, inclusive.
    pub min_expiry_sec: i64,
    pub max_expiry_sec: i64,
    /// Which option types the selection considers: calls, puts, or both.
    pub option_types: Vec<OptionType>,
}

/// Selects the option structure a strategy wants to trade, and declares the universe it selects
/// from so the runtime can subscribe to exactly that set.
///
/// Selection rules belong to the concrete strategy, not the shared runner. A strategy should own
/// its selector and call it from `get_action` when it needs a new structure.
pub trait Selector: Send {
    /// The instruments this selection considers.
    fn candidates(&self) -> Result<Candidates>;

    fn select_structure(&self, market: &MarketData, decision_at: i64) -> Result<Vec<LegUnpriced>>;

    /// The live tickers matching [Self::candidates], i.e. exactly what the runtime subscribed to.
    ///
    /// Selection must start here rather than re-deriving the filter, so the universe a strategy
    /// picks from cannot drift from the one the runtime keeps quotes for. Stale tickers are
    /// excluded: a quote that stopped updating would otherwise win comparisons on old greeks.
    fn eligible<'a>(
        &self,
        market: &'a MarketData,
        decision_at: i64,
    ) -> Result<Vec<&'a InstrumentTicker>> {
        let candidates = self.candidates()?;
        Ok(market
            .iter_fresh_tickers()
            .filter(|ticker| ticker.is_active && ticker.base_currency == candidates.currency)
            .filter(|ticker| {
                ticker.option_details.as_ref().is_some_and(|details| {
                    let tenor = details.expiry - decision_at;
                    candidates.option_types.contains(&details.option_type)
                        && tenor >= candidates.min_expiry_sec
                        && tenor <= candidates.max_expiry_sec
                })
            })
            .collect())
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
    fn candidates(&self) -> Result<Candidates> {
        Ok(Candidates {
            currency: "ETH".to_owned(),
            min_expiry_sec: 0,
            max_expiry_sec: i64::MAX,
            option_types: vec![OptionType::C],
        })
    }

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
    use orderbook_types::types::tickers::result::OptionPublicDetailsSchema;
    use orderbook_types::types::tickers::InstrumentTicker;

    struct EthWeeklies;

    impl Selector for EthWeeklies {
        fn candidates(&self) -> Result<Candidates> {
            Ok(Candidates {
                currency: "ETH".to_owned(),
                min_expiry_sec: 4 * 86400,
                max_expiry_sec: 10 * 86400,
                option_types: vec![OptionType::C],
            })
        }
        fn select_structure(&self, _: &MarketData, _: i64) -> Result<Vec<LegUnpriced>> {
            unimplemented!()
        }
    }

    fn option(name: &str, currency: &str, expiry: i64, option_type: OptionType) -> InstrumentTicker {
        InstrumentTicker {
            base_currency: currency.to_owned(),
            instrument_name: name.to_owned(),
            is_active: true,
            option_details: Some(OptionPublicDetailsSchema {
                expiry,
                index: format!("{currency}-USD"),
                option_type,
                settlement_price: None,
                strike: BigDecimal::from(3000),
            }),
            timestamp: chrono::Utc::now().timestamp_millis(),
            ..InstrumentTicker::default()
        }
    }

    /// The whole point of candidates living here: selection draws from exactly the set the
    /// runtime subscribed to, so the two cannot drift apart.
    #[test]
    fn eligible_is_the_candidate_set_and_nothing_else() {
        let now = 1_784_156_400;
        let day = 86400;
        let mut market = MarketData::new();
        market.insert_ticker(option("ETH-4D-C", "ETH", now + 4 * day, OptionType::C));
        market.insert_ticker(option("ETH-10D-C", "ETH", now + 10 * day, OptionType::C));
        market.insert_ticker(option("ETH-7D-C", "ETH", now + 7 * day, OptionType::C));
        // outside the tenor band on either side
        market.insert_ticker(option("ETH-3D-C", "ETH", now + 3 * day, OptionType::C));
        market.insert_ticker(option("ETH-11D-C", "ETH", now + 11 * day, OptionType::C));
        // right tenor, wrong everything else
        market.insert_ticker(option("ETH-7D-P", "ETH", now + 7 * day, OptionType::P));
        market.insert_ticker(option("BTC-7D-C", "BTC", now + 7 * day, OptionType::C));
        let mut inactive = option("ETH-7D-DEAD-C", "ETH", now + 7 * day, OptionType::C);
        inactive.is_active = false;
        market.insert_ticker(inactive);
        let mut stale = option("ETH-7D-STALE-C", "ETH", now + 7 * day, OptionType::C);
        stale.timestamp = chrono::Utc::now().timestamp_millis() - 10_000;
        market.insert_ticker(stale);

        let mut names: Vec<&str> = EthWeeklies
            .eligible(&market, now)
            .unwrap()
            .iter()
            .map(|ticker| ticker.instrument_name.as_str())
            .collect();
        names.sort();
        // the band is inclusive at both ends
        assert_eq!(names, vec!["ETH-10D-C", "ETH-4D-C", "ETH-7D-C"]);
    }

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
