pub use crate::generated::private_get_subaccount::MarginType;
use serde::{Deserialize, Serialize};

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

impl ToString for AuctionState {
    fn to_string(&self) -> String {
        match *self {
            Self::Ongoing => "ongoing".to_string(),
            Self::Ended => "ended".to_string(),
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
