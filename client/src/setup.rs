use crate::aws::{get_secret, maybe_get_secret};
use log::info;

/// Map derive-py `DERIVE_*` names onto the cockpit's existing env vars.
fn apply_derive_env_aliases() {
    if let Ok(v) = std::env::var("DERIVE_SESSION_KEY") {
        if std::env::var("SESSION_PRIVATE_KEY").is_err() {
            std::env::set_var("SESSION_PRIVATE_KEY", v);
        }
    }
    if let Ok(v) = std::env::var("DERIVE_WALLET") {
        if std::env::var("OWNER_PUBLIC_KEY").is_err() {
            std::env::set_var("OWNER_PUBLIC_KEY", v);
        }
    }
    if std::env::var("ENV").is_err() {
        if let Ok(chain) = std::env::var("DERIVE_ETH_CHAIN") {
            let env = match chain.to_uppercase().as_str() {
                "SEPOLIA" | "901" => "staging",
                "ETHEREUM" | "1" | "957" => "prod",
                other => panic!(
                    "Invalid DERIVE_ETH_CHAIN '{other}': expected SEPOLIA or ETHEREUM"
                ),
            };
            std::env::set_var("ENV", env);
        }
    }
    if let Ok(v) = std::env::var("DERIVE_ETH_RPC_ENDPOINTS") {
        if let Some(first) = v.split(',').map(str::trim).find(|s| !s.is_empty()) {
            if std::env::var("MAINNET_PROVIDER").is_err() {
                std::env::set_var("MAINNET_PROVIDER", first);
            }
        }
    }
}

pub async fn ensure_env() {
    let env_name = std::env::var("ENV").expect("ENV must be set");
    match env_name.as_str() {
        "staging" | "prod" => (),
        _ => panic!("Invalid env name"),
    }
}

/// Validates that the session private key is set in the environment or loads it from AWS if not.
/// Will panic if neither is set.
pub async fn ensure_session_key() {
    if std::env::var("SESSION_PRIVATE_KEY").is_err() {
        if let Ok(v) = std::env::var("DERIVE_SESSION_KEY") {
            std::env::set_var("SESSION_PRIVATE_KEY", v);
        }
    }
    if std::env::var("SESSION_PRIVATE_KEY").is_err() {
        info!("No signer in env, loading signer from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        let name = std::env::var("SESSION_KEY_NAME").expect("SESSION_KEY_NAME must be set");
        let aws_param_name = format!("/session_keys/{env}/{name}");
        std::env::set_var("SESSION_PRIVATE_KEY", get_secret(&aws_param_name, None).await);
    }
}

/// Validates that the owner public key is set in the environment or loads it from AWS if not.
/// Will panic if neither is set.
pub async fn ensure_owner() {
    if std::env::var("OWNER_PUBLIC_KEY").is_err() {
        if let Ok(v) = std::env::var("DERIVE_WALLET") {
            std::env::set_var("OWNER_PUBLIC_KEY", v);
        }
    }
    if std::env::var("OWNER_PUBLIC_KEY").is_err() {
        info!("No owner in env, loading owner from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        let name = std::env::var("OWNER_KEY_NAME").expect("OWNER_KEY_NAME must be set");
        let aws_param_name = format!("/owners/{env}/{name}");
        std::env::set_var("OWNER_PUBLIC_KEY", get_secret(&aws_param_name, None).await);
    }
}

pub async fn setup_env() {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    apply_derive_env_aliases();
    info!("{}", std::env::var("ENV").unwrap());
    ensure_env().await;
    let env_name = std::env::var("ENV").unwrap();
    let env_consts = format!(".env.constants.{env_name}");
    let env_keys = format!(".env.keys.{env_name}");

    // println!("env_consts: {}", env_consts);
    // println!("env_keys: {}", env_keys);
    // println!("env_name: {}", env_name);
    // println!("std::env::var(\"SESSION_PRIVATE_KEY\"): {}", std::env::var("SESSION_PRIVATE_KEY").unwrap());
    // println!("std::env::var(\"OWNER_PUBLIC_KEY\"): {}", std::env::var("OWNER_PUBLIC_KEY").unwrap());    

    dotenv::from_filename(env_consts).expect("Failed to load .env.constants.{} file");
    let key_loaded = dotenv::from_filename(env_keys);
    if key_loaded.is_err() {
        println!("No keys file found for env, expecting them to be in AWS");
    }
    env_logger::builder().format_timestamp_millis().init();
}

pub async fn setup_ws_endpoint() {
    let env = std::env::var("ENV").expect("ENV must be set");
    let aws_param_name = format!("/ws/{env}");
    let maybe_dedicated_ws = maybe_get_secret(&aws_param_name, None).await;
    if let Some(dedicated_ws) = maybe_dedicated_ws {
        std::env::set_var("WEBSOCKET_ADDRESS", dedicated_ws);
        return;
    }
}
