use aws_secrets::{config_from_env, SSMParamExt};
use serde_json::{to_string, Value};
use anyhow::Result;
use log::info;

/// Fetches a session key from aws parameter store. Will panic if the parameter is not found.
pub async fn get_session_key(env: &str) -> String {
    let shared_config = config_from_env().await;
    // Retrieve a parameter from AWS SSM Parameter Store
    let param_name = format!("/session_keys/{env}");
    let value = param_name.get_secure_string(&shared_config).await.expect("Session key not found");
    value
}