pub use crate::generated::private_get_subaccount::MarginType;
use serde::{Deserialize, Serialize};

/// Payload `state` for `auctions.watch`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AuctionState {
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "ended")]
    Ended,
}

impl From<&AuctionState> for AuctionState {
    fn from(value: &AuctionState) -> Self {
        value.clone()
    }
}

impl std::fmt::Display for AuctionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Ongoing => write!(f, "ongoing"),
            Self::Ended => write!(f, "ended"),
        }
    }
}

impl std::str::FromStr for AuctionState {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "ongoing" => Ok(Self::Ongoing),
            "ended" => Ok(Self::Ended),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for AuctionState {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AuctionState {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

/// Auction phase on `public/get_liquidation_history` entries.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AuctionType {
    #[serde(rename = "solvent")]
    Solvent,
    #[serde(rename = "insolvent")]
    Insolvent,
}

impl From<&AuctionType> for AuctionType {
    fn from(value: &AuctionType) -> Self {
        value.clone()
    }
}

impl std::fmt::Display for AuctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Solvent => write!(f, "solvent"),
            Self::Insolvent => write!(f, "insolvent"),
        }
    }
}

impl std::str::FromStr for AuctionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "solvent" => Ok(Self::Solvent),
            "insolvent" => Ok(Self::Insolvent),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for AuctionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for AuctionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
