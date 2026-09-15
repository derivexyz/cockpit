use crate::json_rpc::WsClient;
use derive_types::generated::public_login::PublicLoginParamsSchema;
use ethers::prelude::{LocalWallet, Signer};
use ethers::utils::hex;
use log::info;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};

pub async fn load_signer() -> LocalWallet {
    load_signer_by_name("SESSION").await
}

pub async fn load_signer_by_name(name: &str) -> LocalWallet {
    let env_name = format!("{name}_PRIVATE_KEY");
    info!("Loading signer from env {}", &env_name);
    let mut pk_str = std::env::var(env_name).unwrap();
    let wallet = pk_str.parse::<LocalWallet>().unwrap();
    wallet
}

async fn sign_auth_params(wallet: &LocalWallet) -> (String, String, String) {
    let timestamp = chrono::Utc::now().timestamp_millis().to_string();
    let signature = wallet.sign_message(&timestamp).await.unwrap();
    (std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY"), timestamp, signature.to_string())
}

pub async fn sign_auth_header(wallet: &LocalWallet) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let (address, timestamp, signature) = sign_auth_params(wallet).await;
    // v3 REST auth reads X-Derive*; X-Lyra* is the v2 name and is ignored.
    let wallet_value: HeaderValue = address.parse().unwrap();
    let timestamp_value: HeaderValue = timestamp.parse().unwrap();
    let signature_value: HeaderValue = signature.parse().unwrap();
    headers.insert("X-DeriveWallet", wallet_value);
    headers.insert("X-DeriveTimestamp", timestamp_value);
    headers.insert("X-DeriveSignature", signature_value);
    headers
}

pub async fn get_auth_headers() -> HeaderMap {
    let wallet = load_signer().await;
    sign_auth_header(&wallet).await
}

pub async fn sign_auth_msg(wallet: &LocalWallet) -> PublicLoginParamsSchema {
    let (address, timestamp, signature) = sign_auth_params(wallet).await;
    PublicLoginParamsSchema { wallet: address, timestamp, signature }
}
