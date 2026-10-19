//! Deployment wiring for the mock signal-driven covered call vault.
//!
//! The runtime, market refresh and execution all come from [crate::signals::vault]; this module
//! only supplies the mock's params and its signal source. It is the testnet canary: the same
//! runtime the real strategies use, driven by a signal that flips on a fixed schedule.

use crate::signals::strategies::mock::{MockCCParams, MOCK_SIGNAL_POLL_INTERVAL};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct MockCCVaultParams {
    pub env: String,        // Environment name (e.g. staging, prod)
    pub vault_name: String, // used for logging and as the default key name
    /// The subaccount the vault trades on. Passed as a param since a vault without a TSA has no
    /// contract to read it from.
    pub subaccount_id: i64,
    /// Name of the session key and its owner, i.e. `/session_keys/{env}/{key_name}` and
    /// `/owners/{env}/{key_name}` in the AWS parameter store. Defaults to the lowercased
    /// vault name. Ignored if `SESSION_PRIVATE_KEY` / `OWNER_PUBLIC_KEY` are set in the env.
    pub key_name: Option<String>,
    /// Decision cadence in seconds, defaults to [MOCK_SIGNAL_POLL_INTERVAL].
    pub decision_interval_sec: Option<u64>,

    pub strategy_params: MockCCParams,
}

impl MockCCVaultParams {
    pub fn key_name(&self) -> String {
        self.key_name.clone().unwrap_or(self.vault_name.to_lowercase())
    }

    pub fn decision_interval(&self) -> Duration {
        self.decision_interval_sec.map(Duration::from_secs).unwrap_or(MOCK_SIGNAL_POLL_INTERVAL)
    }
}
