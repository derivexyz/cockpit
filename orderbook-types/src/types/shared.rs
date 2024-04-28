use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RPCId {
    Variant0(String),
    Variant1(i64),
}
impl From<&RPCId> for RPCId {
    fn from(value: &RPCId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RPCId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for RPCId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RPCId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RPCId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for RPCId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for RPCId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RPCError
{
    code: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
    message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RPCErrorResponse
{
    id: RPCId,
    error: RPCError,
}

impl fmt::Display for RPCErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RPCError\n{:?}", self)
    }
}

impl std::error::Error for RPCErrorResponse {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PaginationInfoSchema {
    ///Total number of items, across all pages
    pub count: i64,
    ///Number of pages
    pub num_pages: i64,
}