use crate::lrtc::params::LRTCParams;
use crate::market::{new_market_state, MarketState};
use anyhow::{Error, Result};
use bigdecimal::num_traits::real::Real;
use bigdecimal::{BigDecimal, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use bigdecimal::One;
use log::{debug, error, info, warn};
use lyra_client::auth::{load_signer, sign_auth_header};
use lyra_client::json_rpc::{http_rpc, Notification, Response, WsClient, WsClientExt};
use orderbook_types::types::orders::Direction;
use orderbook_types::types::rfqs::LegUnpriced;
use orderbook_types::types::tickers::result::{
    InstrumentTicker, InstrumentsResponse, OptionType, TickerNotificationData,
};
use serde_json::{json, Value};
use tokio::select;

use crate::helpers::{get_expiry_options, subscribe_tickers, sync_subaccount, TickerInterval};
use crate::longpp::params::LongPPParams;
use crate::shared::rfq::get_legs_mark_unit_cost;

/// Returns the option name that satisfies the LRT-C params (target expiry and delta)
pub async fn select_new_spread(params: &LongPPParams) -> Result<Vec<LegUnpriced>> {
    let market = new_market_state();
    let client = WsClient::new_client().await?;
    let now = chrono::Utc::now().timestamp();
    let err = Error::msg("No options found within the LongPP params");
    let is_long = params.is_long;
    let strike_dir = if params.is_call { BigDecimal::one() } else { -BigDecimal::one() };
    let expiry_options = get_expiry_options(
        &params.option_currency,
        params.expiry_sec(),
        params.min_expiry_sec(),
        params.is_call,
    )
    .await?;

    let sub = subscribe_tickers(market.clone(), expiry_options, TickerInterval::_1000Ms);
    let _ = select! {
        _ = sub => {},
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(3)) => {},
    };

    let reader = market.read().await;
    let tickers = reader.get_tickers();
    let mut strike_to_tickers = HashMap::<BigDecimal, &InstrumentTicker>::new();
    for (name, ticker) in tickers {
        if let Some(ref d) = ticker.option_details {
            strike_to_tickers.insert(d.strike.clone(), ticker);
        }
    }
    let spreads: Vec<(Vec<LegUnpriced>, BigDecimal)> = reader
        .iter_tickers()
        .filter_map(|ticker| {
            let details = ticker.option_details.as_ref().unwrap();
            let other_strike = &details.strike + &params.strike_diff * &strike_dir;
            let other_ticker = strike_to_tickers.get(&other_strike);
            if other_ticker.is_none() {
                info!("No ticker for strike {}", other_strike);
                return None;
            }
            let other_ticker = other_ticker.unwrap();
            let legs = vec![
                LegUnpriced {
                    instrument_name: ticker.instrument_name.clone(),
                    direction: if is_long { Direction::Buy } else { Direction::Sell },
                    amount: BigDecimal::one(),
                },
                LegUnpriced {
                    instrument_name: other_ticker.instrument_name.clone(),
                    direction: if is_long { Direction::Sell } else { Direction::Buy },
                    amount: BigDecimal::one(),
                },
            ];
            let mark = get_legs_mark_unit_cost(&legs, tickers).unwrap().abs();
            let r = mark / &params.strike_diff;
            if r > params.max_premium_to_strike_ratio || r < params.min_premium_to_strike_ratio {
                return None;
            }
            Some((legs, r))
        })
        .collect();

    let target_ratio = &params.target_premium_to_strike_ratio;
    let selected_spread = spreads.into_iter().min_by_key(|(_, r)| (r - target_ratio).abs());
    match selected_spread {
        Some((mut spread, _)) => {
            spread.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
            Ok(spread)
        }
        None => Err(err),
    }
}

/// Returns the legs array from an existing position
/// Expects the market state to be synced to the subaccount
pub async fn maybe_select_from_positions(market: &MarketState) -> Result<Option<Vec<LegUnpriced>>> {
    let reader = market.read().await;
    let option_names: Vec<String> = reader
        .iter_positions()
        .filter(|&p| {
            p.amount != BigDecimal::zero()
                && (p.instrument_name.ends_with("-C") || p.instrument_name.ends_with("-P"))
        })
        .map(|p| p.instrument_name.clone())
        .collect();

    match option_names.len() {
        0 => Ok(None),
        1 => Err(Error::msg("Unexpected single open option without spread")),
        2 => {
            let mut legs: Vec<LegUnpriced> = reader
                .iter_positions()
                .filter(|&p| option_names.contains(&p.instrument_name))
                .map(|b| {
                    let direction = if b.amount > BigDecimal::zero() {
                        Direction::Buy
                    } else {
                        Direction::Sell
                    };
                    LegUnpriced {
                        instrument_name: b.instrument_name.clone(),
                        direction,
                        amount: BigDecimal::one(),
                    }
                })
                .collect();
            legs.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
            Ok(Some(legs))
        }
        _ => Err(Error::msg("Unexpected 3 or more open options positions")),
    }
}
