use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum InstrumentType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
}

impl From<&InstrumentType> for InstrumentType {
    fn from(value: &InstrumentType) -> Self {
        value.clone()
    }
}

impl ToString for InstrumentType {
    fn to_string(&self) -> String {
        match *self {
            Self::Erc20 => "erc20".to_string(),
            Self::Option => "option".to_string(),
            Self::Perp => "perp".to_string(),
        }
    }
}

impl std::str::FromStr for InstrumentType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "erc20" => Ok(Self::Erc20),
            "option" => Ok(Self::Option),
            "perp" => Ok(Self::Perp),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for InstrumentType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TickerInterval {
    #[serde(rename = "100")]
    _100,
    #[serde(rename = "1000")]
    _1000,
}

impl From<&TickerInterval> for TickerInterval {
    fn from(value: &TickerInterval) -> Self {
        value.clone()
    }
}

impl ToString for TickerInterval {
    fn to_string(&self) -> String {
        match *self {
            Self::_100 => "100".to_string(),
            Self::_1000 => "1000".to_string(),
        }
    }
}

impl std::str::FromStr for TickerInterval {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "100" => Ok(Self::_100),
            "1000" => Ok(Self::_1000),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for TickerInterval {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for TickerInterval {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for TickerInterval {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum OptionType {
    C,
    P,
}

impl OptionType {
    pub fn is_call(&self) -> bool {
        match self {
            Self::C => true,
            _ => false,
        }
    }
}

impl From<&OptionType> for OptionType {
    fn from(value: &OptionType) -> Self {
        value.clone()
    }
}

impl ToString for OptionType {
    fn to_string(&self) -> String {
        match *self {
            Self::C => "C".to_string(),
            Self::P => "P".to_string(),
        }
    }
}

impl std::str::FromStr for OptionType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "C" => Ok(Self::C),
            "P" => Ok(Self::P),
            _ => Err("invalid value"),
        }
    }
}

impl std::convert::TryFrom<&str> for OptionType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<&String> for OptionType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}

impl std::convert::TryFrom<String> for OptionType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
