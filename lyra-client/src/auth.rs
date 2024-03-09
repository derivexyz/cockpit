use ethers::prelude::{LocalWallet, Signer};
use ethers::utils::hex;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};

use orderbook_types::generated::public_login::PublicLoginParamsSchema;

pub fn load_signer() -> LocalWallet {
    // TODO session key instead
    let pk_str = std::env::var("OWNER_PRIVATE_KEY").expect("OWNER_PRIVATE_KEY must be set");
    let wallet = pk_str.parse::<LocalWallet>().unwrap();
    wallet
}

async fn sign_auth_params(wallet: &LocalWallet) -> (String, String, String) {
    let timestamp = chrono::Utc::now().timestamp_millis().to_string();
    let signature = wallet.sign_message(&timestamp).await.unwrap();
    (
        hex::encode_prefixed(wallet.address()),
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
