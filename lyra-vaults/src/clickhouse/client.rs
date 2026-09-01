use anyhow::{bail, Error, Result};
use log::{debug, info};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::time::Duration;

const DEFAULT_TIMEOUT_SEC: u64 = 60;
/// AWS parameter store path holding `KEY_ID:KEY_SECRET`.
const DEFAULT_KEY_PARAM: &str = "/api-keys/clickhouse";

/// Minimal client for the ClickHouse Cloud query API.
///
/// The endpoint takes a `{"sql": ...}` body and is authenticated with an organization API key,
/// so no database user is involved. Rows come back as `JSONEachRow`, one JSON object per line.
pub struct ClickhouseClient {
    http: reqwest::Client,
    url: String,
    key_id: String,
    key_secret: String,
}

/// An error response, which the endpoint can return with either a 4xx or a 200 status.
#[derive(Deserialize)]
struct ErrorBody {
    error: String,
}

impl ClickhouseClient {
    pub fn new(
        url: String,
        key_id: String,
        key_secret: String,
        timeout: Duration,
    ) -> Result<Self> {
        let http = reqwest::Client::builder().timeout(timeout).build()?;
        Ok(Self { http, url, key_id, key_secret })
    }

    /// Validates that the API key is in the environment, or loads it from the AWS parameter
    /// store, where it is stored as one `KEY_ID:KEY_SECRET` secret. Panics if neither is set,
    /// matching [lyra_client::setup::ensure_session_key].
    pub async fn ensure_keys() {
        if std::env::var("CLICKHOUSE_KEY_ID").is_ok()
            && std::env::var("CLICKHOUSE_KEY_SECRET").is_ok()
        {
            return;
        }
        let param = std::env::var("CLICKHOUSE_KEY_PARAM")
            .unwrap_or(DEFAULT_KEY_PARAM.to_string());
        info!("No clickhouse key in env, loading {} from AWS", param);
        let secret = lyra_client::aws::get_secret(&param, None).await;
        let (key_id, key_secret) = secret
            .trim()
            .split_once(':')
            .unwrap_or_else(|| panic!("{param} must be stored as KEY_ID:KEY_SECRET"));
        std::env::set_var("CLICKHOUSE_KEY_ID", key_id);
        std::env::set_var("CLICKHOUSE_KEY_SECRET", key_secret);
    }

    /// Reads `CLICKHOUSE_QUERY_URL` (the bare `.../run` endpoint), `CLICKHOUSE_KEY_ID` and
    /// `CLICKHOUSE_KEY_SECRET`, with an optional `CLICKHOUSE_TIMEOUT_SEC`.
    pub fn from_env() -> Result<Self> {
        let var = |name: &str| {
            std::env::var(name).map_err(|_| Error::msg(format!("{name} is not set")))
        };
        let timeout = match std::env::var("CLICKHOUSE_TIMEOUT_SEC") {
            Ok(secs) => secs.parse()?,
            Err(_) => DEFAULT_TIMEOUT_SEC,
        };
        Self::new(
            var("CLICKHOUSE_QUERY_URL")?,
            var("CLICKHOUSE_KEY_ID")?,
            var("CLICKHOUSE_KEY_SECRET")?,
            Duration::from_secs(timeout),
        )
    }

    /// Runs a query and deserializes every returned row.
    pub async fn query<T: DeserializeOwned>(&self, sql: &str) -> Result<Vec<T>> {
        debug!("clickhouse query: {}", sql);
        let res = self
            .http
            .post(&self.url)
            .query(&[("format", "JSONEachRow")])
            .basic_auth(&self.key_id, Some(&self.key_secret))
            .json(&serde_json::json!({ "sql": sql }))
            .send()
            .await?;

        let status = res.status();
        let body = res.text().await?;
        if !status.is_success() {
            bail!("clickhouse query failed with {}: {}", status, truncate(&body));
        }
        // e.g. a query that exceeds the endpoint's own limit answers 200 with {"error": "Timeout error."}
        if let Ok(err) = serde_json::from_str::<ErrorBody>(&body) {
            bail!("clickhouse query failed: {}", err.error);
        }

        let mut rows = Vec::new();
        for line in body.lines().filter(|line| !line.trim().is_empty()) {
            let row = serde_json::from_str(line)
                .map_err(|e| Error::msg(format!("unexpected row {}: {e}", truncate(line))))?;
            rows.push(row);
        }
        Ok(rows)
    }
}

fn truncate(body: &str) -> String {
    let body = body.trim();
    match body.char_indices().nth(500) {
        Some((idx, _)) => format!("{}...", &body[..idx]),
        None => body.to_string(),
    }
}

/// ClickHouse quotes 64-bit integers as strings in JSON by default, so accept either form.
pub(crate) fn de_i64<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Flexible {
        Int(i64),
        Str(String),
    }
    match Flexible::deserialize(deserializer)? {
        Flexible::Int(value) => Ok(value),
        Flexible::Str(value) => value.parse().map_err(serde::de::Error::custom),
    }
}

/// Rejects anything that is not a plain currency symbol, since queries interpolate it into SQL.
pub(crate) fn validate_currency(currency: &str) -> Result<()> {
    let is_valid = !currency.is_empty()
        && currency.len() <= 16
        && currency.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    match is_valid {
        true => Ok(()),
        false => bail!("invalid currency {:?}", currency),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Debug, PartialEq)]
    struct Row {
        #[serde(deserialize_with = "de_i64")]
        hour: i64,
    }

    #[test]
    fn i64_columns_parse_quoted_or_bare() {
        let quoted: Row = serde_json::from_str(r#"{"hour":"1784156400"}"#).unwrap();
        let bare: Row = serde_json::from_str(r#"{"hour":1784156400}"#).unwrap();
        assert_eq!(quoted, bare);
        assert_eq!(quoted.hour, 1784156400);
        assert!(serde_json::from_str::<Row>(r#"{"hour":"not a number"}"#).is_err());
    }

    #[test]
    fn currencies_are_restricted_to_plain_symbols() {
        assert!(validate_currency("ETH").is_ok());
        assert!(validate_currency("XAUT").is_ok());
        assert!(validate_currency("").is_err());
        assert!(validate_currency("ETH'; DROP TABLE default.raw_spot_feed; --").is_err());
        assert!(validate_currency("ETH-PERP").is_err());
    }
}
