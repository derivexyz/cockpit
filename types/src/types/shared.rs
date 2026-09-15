use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// Action nonces are UTC nanoseconds encoded as JSON decimal strings.
/// Decode still accepts integers so older payloads keep working.
pub mod serde_nonce {
    use super::*;

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = i64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a nonce as a decimal string or integer")
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<i64, E> {
                Ok(value)
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<i64, E> {
                i64::try_from(value).map_err(E::custom)
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<i64, E> {
                value.parse().map_err(E::custom)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

/// v3 returns some former scalar strings as JSON arrays (e.g. subaccount `currency`).
pub mod serde_string_or_vec {
    use super::*;

    pub fn serialize<S>(value: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        value.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or an array of strings")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Vec<String>, E> {
                Ok(vec![value.to_string()])
            }

            fn visit_string<E: de::Error>(self, value: String) -> Result<Vec<String>, E> {
                Ok(vec![value])
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Vec<String>, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut out = Vec::new();
                while let Some(item) = seq.next_element::<String>()? {
                    out.push(item);
                }
                Ok(out)
            }
        }
        deserializer.deserialize_any(Visitor)
    }
}

pub mod serde_option_nonce {
    use super::*;

    pub fn serialize<S>(value: &Option<i64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(nonce) => serializer.serialize_some(&nonce.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<serde_json::Value>::deserialize(deserializer)? {
            None | Some(serde_json::Value::Null) => Ok(None),
            Some(serde_json::Value::String(s)) => s.parse().map(Some).map_err(de::Error::custom),
            Some(serde_json::Value::Number(n)) => n
                .as_i64()
                .map(Some)
                .ok_or_else(|| de::Error::custom("nonce out of i64 range")),
            Some(other) => Err(de::Error::custom(format!("invalid nonce: {other}"))),
        }
    }
}

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
pub struct RPCError {
    pub code: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RPCErrorResponse {
    /// HTTP JSON-RPC errors from v3 often omit `id` (e.g. auth failures).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RPCId>,
    pub error: RPCError,
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

#[cfg(test)]
mod tests {
    use super::serde_nonce;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Deserialize, Serialize)]
    struct NonceWrapper {
        #[serde(with = "serde_nonce")]
        nonce: i64,
    }

    #[test]
    fn nonce_serializes_as_decimal_string() {
        let encoded = serde_json::to_value(NonceWrapper { nonce: 1789133536619202 }).unwrap();
        assert_eq!(encoded, json!({ "nonce": "1789133536619202" }));
    }

    #[test]
    fn nonce_deserializes_from_string_or_integer() {
        let from_string: NonceWrapper =
            serde_json::from_value(json!({ "nonce": "1789133536619202" })).unwrap();
        let from_int: NonceWrapper =
            serde_json::from_value(json!({ "nonce": 1789133536619202i64 })).unwrap();
        assert_eq!(from_string.nonce, 1789133536619202);
        assert_eq!(from_int.nonce, 1789133536619202);
    }

    #[test]
    fn rpc_error_response_parses_without_id() {
        let parsed: super::RPCErrorResponse = serde_json::from_value(json!({
            "error": {
                "code": -32602,
                "message": "Invalid params",
                "data": "Missing wallet in header"
            }
        }))
        .unwrap();
        assert!(parsed.id.is_none());
        assert_eq!(parsed.error.code, -32602);
        assert_eq!(parsed.error.message, "Invalid params");
    }

    #[test]
    fn string_or_vec_accepts_string_or_array() {
        #[derive(Debug, Deserialize, Serialize)]
        struct Wrapper {
            #[serde(with = "super::serde_string_or_vec")]
            currency: Vec<String>,
        }
        let from_array: Wrapper =
            serde_json::from_value(json!({ "currency": ["BTC", "ETH"] })).unwrap();
        let from_string: Wrapper = serde_json::from_value(json!({ "currency": "ETH" })).unwrap();
        assert_eq!(from_array.currency, vec!["BTC", "ETH"]);
        assert_eq!(from_string.currency, vec!["ETH"]);
    }
}
