use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum PerformanceResolution {
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "24h")]
    TwentyFourHours,
    #[serde(rename = "1wk")]
    OneWeek,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum VaultAction {
    Create = 0,
    Deposit = 1,
    Withdraw = 2,
    Cancel = 3,
    MintShares = 4,
    BurnShares = 5,
}
