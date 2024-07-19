use crate::shared::auction::LimitOrderAuctionExecutor;
use crate::shared::params::OptionRFQParams;
use crate::shared::params::SpotAuctionParams;
use crate::shared::rfq::RFQAuctionExecutor;
use crate::shared::stages::{TSACollateralOnly, TSAWaitForSettlement};
use std::fmt::Debug;

#[derive(Debug)]
pub enum LongPPExecutorStage {
    SpotOnly(TSACollateralOnly),
    OptionAuction(RFQAuctionExecutor<OptionRFQParams>),
    AwaitSettlement(TSAWaitForSettlement),
    SpotAuction(LimitOrderAuctionExecutor<SpotAuctionParams>),
}
