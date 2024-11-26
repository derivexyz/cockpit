use anyhow::Result;
use aws_secrets::config::SdkConfig;
use aws_secrets::{config_from_env, SSMParamExt};
use log::info;
use serde_json::{to_string, Value};

pub async fn get_secret(name: &str, config: Option<SdkConfig>) -> String {
    let aws_config = config.unwrap_or(config_from_env().await);
    let value = name.get_secure_string(&aws_config).await.expect("Secret not found");
    value
}

pub async fn maybe_get_secret(name: &str, config: Option<SdkConfig>) -> Option<String> {
    let aws_config = config.unwrap_or(config_from_env().await);
    let value = name.get_secure_string(&aws_config).await;
    match value {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
