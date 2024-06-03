use aws_secrets::{config_from_env, SSMParamExt};
use aws_secrets::config::SdkConfig;
use serde_json::{to_string, Value};
use anyhow::Result;
use log::info;

pub async fn get_secret(name: &str, config: Option<SdkConfig>) -> String {
    let aws_config = config.unwrap_or(config_from_env().await);
    let value = name.get_secure_string(&aws_config).await.expect("Secret not found");
    value
}
