use crate::json_rpc::WsClient;
use ethers::prelude::{LocalWallet, Signer};
use ethers::utils::hex;
use log::info;
use orderbook_types::generated::public_login::PublicLoginParamsSchema;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

pub async fn load_signer() -> LocalWallet {
    load_signer_by_name("SESSION_PRIVATE_KEY").await
}

pub async fn load_signer_by_name(name: &str) -> LocalWallet {
    info!("Loading signer from env");
    let mut pk_str = std::env::var(name).unwrap();
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
    headers.insert("X-LyraWallet", address.parse().unwrap());
    headers.insert("X-LyraTimestamp", timestamp.parse().unwrap());
    headers.insert("X-LyraSignature", signature.parse().unwrap());
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
