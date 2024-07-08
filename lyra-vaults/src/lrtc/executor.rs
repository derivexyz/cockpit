use crate::helpers::{fetch_ticker, get_option_expiry, sync_subaccount};
use crate::lrtc::params::LRTCParams;
use crate::lrtc::selector::{maybe_select_from_positions, select_new_option};
use crate::lrtc::stages::LRTCExecutorStage::{
    AwaitSettlement, OptionAuction, SpotAuction, SpotOnly,
};
use crate::lrtc::stages::{LRTCAwaitSettlement, LRTCExecutorStage, LRTCSpotOnly, LRTCStage};
use crate::market::new_market_state;
use crate::shared::auction::{LimitOrderAuction, LimitOrderAuctionExecutor};
use anyhow::Result;
use bigdecimal::{BigDecimal, Zero};
use log::info;

pub struct LRTCExecutor {
    params: LRTCParams,
    stage: LRTCExecutorStage,
}

impl LRTCExecutor {
    /// Create a new LRTCExecutor inferring the state from the positions / market
    /// Cues for the state:
    /// - Spot Only MUST have no options and USDC < threshold and USDC >= 0
    /// - Option Auction has USDC >= 0 and # of options > 0 and expiry >= auction len x 2
    /// - Await Settlement has USDC >= 0 and # of options > 0 and expiry < auction len x 2
    /// - Spot Auction has no options and USDC < 0 or USDC > threshold
    /// Usually the executor will start in the Spot Only state, the other states are meant for
    /// recovery from hard crashes during e.g. spot or option auction
    pub async fn new(params: LRTCParams) -> Result<Self> {
        let market = new_market_state();
        let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID").unwrap().parse()?;
        sync_subaccount(market.clone(), subaccount_id, vec![]).await?;

        let option_name = maybe_select_from_positions(&market).await?;
        info!("Current option position: {:?}", option_name);

        let reader = market.read().await;
        let cash_bal = reader.get_amount(&params.cash_name);
        drop(reader);

        let cash_threshold = &params.spot_auction_params.max_cash;
        let is_cash_within_threshold = cash_bal >= -cash_threshold && &cash_bal < cash_threshold;

        if option_name.is_none() && is_cash_within_threshold {
            info!("Starting in Spot Only stage");
            return Ok(Self { params, stage: SpotOnly(LRTCSpotOnly::new().await?) });
        } else if option_name.is_none() && !is_cash_within_threshold {
            info!("Starting in Spot Auction stage");
            let stage = LRTCExecutor::new_spot_stage(params.clone()).await?;
            return Ok(Self { params, stage });
        }
        let option_name = option_name.unwrap();

        fetch_ticker(market.clone(), &option_name).await?;
        let reader = market.read().await;
        let option_expiry =
            reader.get_ticker(&option_name).unwrap().option_details.as_ref().unwrap().expiry;

        // in case of an executor restart during an auction, we will continue the auction
        // if it is likely to still be ongoing
        let now = chrono::Utc::now().timestamp();
        let approx_auction_start = params.option_auction_start(option_expiry);
        let is_still_ongoing =
            now < approx_auction_start + params.option_auction_params.auction_sec;
        let is_expiry_still_valid = option_expiry > now + params.min_expiry_sec();

        return if is_still_ongoing && is_expiry_still_valid {
            info!("Starting in Option Auction stage");
            let stage = LRTCExecutor::new_option_stage(params.clone(), option_name).await?;
            Ok(Self { params, stage })
        } else {
            info!("Starting in Await Settlement stage");
            let stage = LRTCExecutor::new_settlement_stage(params.clone(), option_name).await?;
            Ok(Self { params, stage })
        };
    }

    pub async fn new_settlement_stage(
        params: LRTCParams,
        option_name: String,
    ) -> Result<LRTCExecutorStage> {
        Ok(AwaitSettlement(LRTCAwaitSettlement::new(params, option_name).await?))
    }

    pub async fn new_option_stage(
        params: LRTCParams,
        option_name: String,
    ) -> Result<LRTCExecutorStage> {
        let option_expiry = get_option_expiry(&option_name).await?;
        let auction = LimitOrderAuction::new(
            option_name,
            params.option_auction_start(option_expiry),
            params.option_auction_params.auction_sec,
            params.option_auction_params.price_change_tolerance.clone(),
        )
        .await?;
        let stage = OptionAuction(LimitOrderAuctionExecutor {
            auction,
            strategy: params.option_auction_params.clone(),
        });
        Ok(stage)
    }

    pub async fn new_spot_stage(params: LRTCParams) -> Result<LRTCExecutorStage> {
        // pass current time as start_sec to avoid querying the option expiry (which is not known yet)
        // spot auction always start after AwaitSettlement and it will ensure to wait for spot_auction_delay
        let auction = LimitOrderAuction::new(
            format!("{}-{}", params.spot_name.clone(), params.cash_name.clone()),
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

    pub async fn select_new_option_until_success(&self) -> String {
        loop {
            match select_new_option(&self.params).await {
                Ok(option_name) => return option_name,
                Err(e) => {
                    info!("select_new_option failed with {:#}, waiting for 60s", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                }
            }
        }
    }

    async fn await_option_auction_start(&self) -> Result<()> {
        let option_name = self.select_new_option_until_success().await;
        let option_expiry = get_option_expiry(&option_name).await?;
        let start_sec = self.params.option_auction_start(option_expiry);
        let sleep_sec = start_sec - chrono::Utc::now().timestamp();
        if sleep_sec > 0 {
            info!("Executor await_option_auction_start sleep for {} sec", sleep_sec);
            tokio::time::sleep(tokio::time::Duration::from_secs(sleep_sec as u64)).await;
        }
        Ok(())
    }

    pub async fn next(&mut self) -> Result<()> {
        self.stage = match &self.stage {
            SpotOnly(_) => {
                let option_name = select_new_option(&self.params).await;
                match option_name {
                    Ok(_) => {
                        self.await_option_auction_start().await?;
                        let option_name = self.select_new_option_until_success().await;
                        LRTCExecutor::new_option_stage(self.params.clone(), option_name).await?
                    }
                    Err(e) => {
                        info!("select_new_option failed with {:#}, re-entering spot only stage", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                        SpotOnly(LRTCSpotOnly::new().await?)
                    }
                }
            }
            OptionAuction(ref s) => {
                let option_name = s.auction.instrument_name.clone();
                LRTCExecutor::new_settlement_stage(self.params.clone(), option_name).await?
            }
            AwaitSettlement(_) => LRTCExecutor::new_spot_stage(self.params.clone()).await?,
            SpotAuction(_) => SpotOnly(LRTCSpotOnly::new().await?),
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
