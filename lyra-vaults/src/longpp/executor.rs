use crate::helpers::{fetch_ticker, get_option_expiry, sleep_till, sync_subaccount};
use crate::longpp::params::LongPPParams;
use crate::longpp::selector::{maybe_select_from_positions, select_new_spread};
use crate::longpp::stages::LongPPExecutorStage;
use crate::longpp::stages::LongPPExecutorStage::{
    AwaitSettlement, OptionAuction, SpotAuction, SpotOnly,
};
use crate::market::new_market_state;
use crate::shared::auction::{LimitOrderAuction, LimitOrderAuctionExecutor};
use crate::shared::rfq::{RFQAuction, RFQAuctionExecutor};
use crate::shared::stages::{ExecutorStage, TSACollateralOnly, TSAWaitForSettlement};
use anyhow::Result;
use bigdecimal::{BigDecimal, Zero};
use log::info;
use orderbook_types::types::rfqs::LegUnpriced;

pub struct LongPPExecutor {
    params: LongPPParams,
    stage: LongPPExecutorStage,
}

impl LongPPExecutor {
    /// Create a new LongPPExecutor inferring the state from the positions / market
    /// Cues for the state:
    /// - Spot Only MUST have no options and USDC < threshold and USDC >= -threshold
    /// - Option Auction has USDC >= 0 and # of options > 0 and expiry >= auction len
    /// - Await Settlement has USDC >= 0 and # of options > 0 and expiry < auction len
    /// - Spot Auction has no options and USDC < 0 or USDC > threshold
    /// Usually the executor will start in the Spot Only state, the other states are meant for
    /// recovery from hard crashes during e.g. spot or option auction
    pub async fn new(params: LongPPParams) -> Result<Self> {
        let market = new_market_state();
        let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID").unwrap().parse()?;
        sync_subaccount(market.clone(), subaccount_id, vec![]).await?;

        let open_legs = maybe_select_from_positions(&market).await?;
        info!("Current option positions: {:?}", open_legs);

        let reader = market.read().await;
        let cash_bal = reader.get_amount(&params.spot_auction_params.cash_name);
        drop(reader);

        let is_cash_within_threshold =
            params.spot_auction_params.is_cash_within_threshold(&cash_bal);

        if open_legs.is_none() && is_cash_within_threshold {
            info!("Starting in Spot Only stage");
            return Ok(Self { params, stage: SpotOnly(TSACollateralOnly::new().await?) });
        } else if open_legs.is_none() && !is_cash_within_threshold {
            info!("Starting in Spot Auction stage");
            let stage = LongPPExecutor::new_spot_auction_stage(params.clone()).await?;
            return Ok(Self { params, stage });
        }
        let open_legs = open_legs.unwrap();
        let option_expiry = get_option_expiry(&open_legs[0].instrument_name).await?;

        // in case of an executor restart during an auction, we will continue the auction
        // if it is likely to still be ongoing
        let now = chrono::Utc::now().timestamp();
        let approx_auction_start = params.option_auction_start(option_expiry);
        let is_still_ongoing =
            now < approx_auction_start + params.option_auction_params.auction_sec;
        let is_expiry_still_valid = option_expiry > now + params.min_expiry_sec();

        return if is_still_ongoing && is_expiry_still_valid {
            info!("Starting in Option Auction stage");
            let stage = LongPPExecutor::new_option_stage(params.clone(), open_legs).await?;
            Ok(Self { params, stage })
        } else {
            info!("Starting in Await Settlement stage");
            let stage = LongPPExecutor::new_settlement_stage(params.clone(), open_legs).await?;
            Ok(Self { params, stage })
        };
    }

    pub async fn new_settlement_stage(
        params: LongPPParams,
        legs: Vec<LegUnpriced>,
    ) -> Result<LongPPExecutorStage> {
        let option_names = legs.into_iter().map(|l| l.instrument_name).collect();
        Ok(AwaitSettlement(
            TSAWaitForSettlement::new(params.spot_auction_delay_min, option_names).await?,
        ))
    }

    pub async fn new_option_stage(
        params: LongPPParams,
        legs: Vec<LegUnpriced>,
    ) -> Result<LongPPExecutorStage> {
        let option_name = legs[0].instrument_name.clone();
        let option_expiry = get_option_expiry(&option_name).await?;
        let auction = RFQAuction::new(
            legs,
            params.option_auction_start(option_expiry),
            params.option_auction_params.lot_init_sleep_sec,
            params.option_auction_params.auction_sec,
        )
        .await?;
        let stage = OptionAuction(RFQAuctionExecutor {
            auction,
            strategy: params.option_auction_params.clone(),
        });
        Ok(stage)
    }

    pub async fn new_spot_auction_stage(params: LongPPParams) -> Result<LongPPExecutorStage> {
        // pass current time as start_sec to avoid querying the option expiry (which is not known yet)
        // spot auction always start after AwaitSettlement and it will ensure to wait for spot_auction_delay
        let auction = LimitOrderAuction::new(
            params.spot_instrument_name(),
            chrono::Utc::now().timestamp(),
            params.spot_auction_params.auction_sec,
            params.spot_auction_params.price_change_tolerance.clone(),
        )
        .await?;
        let stage = SpotAuction(LimitOrderAuctionExecutor {
            auction,
            strategy: params.spot_auction_params.clone(),
        });
        Ok(stage)
    }

    pub async fn select_new_spread_until_success(&self) -> Vec<LegUnpriced> {
        loop {
            match select_new_spread(&self.params).await {
                Ok(legs) => return legs,
                Err(e) => {
                    info!("select_new_spread failed with {:#}, waiting for 60s", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            }
        }
    }

    async fn await_option_auction_start(&self) -> Result<()> {
        let option_name = self.select_new_spread_until_success().await;
        let option_name = option_name[0].instrument_name.clone();
        let option_expiry = get_option_expiry(&option_name).await?;
        let start_sec = self.params.option_auction_start(option_expiry);
        sleep_till(start_sec).await;
        Ok(())
    }

    pub async fn next(&mut self) -> Result<()> {
        self.stage = match &self.stage {
            SpotOnly(_) => {
                let legs = select_new_spread(&self.params).await;
                match legs {
                    Ok(_) => {
                        self.await_option_auction_start().await?;
                        let legs = self.select_new_spread_until_success().await;
                        LongPPExecutor::new_option_stage(self.params.clone(), legs).await?
                    }
                    Err(e) => {
                        info!("select_new_spread failed with {:#}, re-entering spot only stage", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                        SpotOnly(TSACollateralOnly::new().await?)
                    }
                }
            }
            OptionAuction(ref s) => {
                let legs = s.auction.unit_legs.clone();
                LongPPExecutor::new_settlement_stage(self.params.clone(), legs).await?
            }
            AwaitSettlement(_) => {
                LongPPExecutor::new_spot_auction_stage(self.params.clone()).await?
            }
            SpotAuction(_) => SpotOnly(TSACollateralOnly::new().await?),
        };
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            info!("Stage {:?} entered", self.stage);
            match self.stage {
                SpotOnly(ref mut stage) => stage.run_with_reconnect().await?,
                OptionAuction(ref mut stage) => stage.run_with_reconnect().await?,
                AwaitSettlement(ref mut stage) => stage.run_with_reconnect().await?,
                SpotAuction(ref mut stage) => stage.run_with_reconnect().await?,
            }
            info!("Stage {:?} completed", self.stage);
            self.next().await?;
        }
    }
}
