use ethers::prelude::{LocalWallet, Signer};
use ethers::utils::hex;
use log::info;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use crate::aws::get_session_key;

use orderbook_types::generated::public_login::PublicLoginParamsSchema;

pub async fn load_signer() -> LocalWallet {
    info!("Loading signer from env");
    let mut pk_str = std::env::var("SESSION_PRIVATE_KEY");
    if pk_str.is_err() {
        info!("No signer in env, loading signer from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        pk_str = Ok(get_session_key(&env).await);
    }
    let wallet = pk_str.expect("Session Key").parse::<LocalWallet>().unwrap();
    wallet
}

async fn sign_auth_params(wallet: &LocalWallet) -> (String, String, String) {
    let timestamp = chrono::Utc::now().timestamp_millis().to_string();
    let signature = wallet.sign_message(&timestamp).await.unwrap();
    (
        std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY must be set"),
        timestamp,
        signature.to_string(),
    )
}

pub async fn sign_auth_header(wallet: &LocalWallet) -> HeaderMap {
    let mut headers = HeaderMap::new();
    let (address, timestamp, signature) = sign_auth_params(wallet).await;
    headers.insert("X-LyraWallet", address.parse().unwrap());
    headers.insert("X-LyraTimestamp", timestamp.parse().unwrap());
    headers.insert("X-LyraSignature", signature.parse().unwrap());
    headers
}

pub async fn sign_auth_msg(wallet: &LocalWallet) -> PublicLoginParamsSchema {
    let (address, timestamp, signature) = sign_auth_params(wallet).await;
    PublicLoginParamsSchema {
        wallet: address,
        timestamp,
        signature,
    }
}
