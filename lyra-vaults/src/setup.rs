use crate::aws::get_secret;
use dotenv::dotenv;
use env_logger;
use log::info;

/// These are currently copied from lyra-maker - will need to be changed
/// for the vaults logic

pub async fn ensure_env() {
    let args: Vec<String> = std::env::args().collect();
    let env_name = args.get(1);
    match env_name {
        None => std::env::set_var("ENV", "local"),
        Some(env_name) => match env_name.as_str() {
            "local" => std::env::set_var("ENV", "local"),
            "staging" => std::env::set_var("ENV", "staging"),
            "prod" => std::env::set_var("ENV", "prod"),
            _ => panic!("Invalid env name"),
        },
    }
}

pub async fn ensure_session_key() {
    let mut pk_str = std::env::var("SESSION_PRIVATE_KEY");
    if pk_str.is_err() {
        info!("No signer in env, loading signer from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        let name = format!("/session_keys/{env}");
        std::env::set_var("SESSION_PRIVATE_KEY", get_secret(&name, None).await);
    }
}

pub async fn ensure_owner() {
    let mut pk_str = std::env::var("OWNER_PUBLIC_KEY");
    if pk_str.is_err() {
        info!("No owner in env, loading owner from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        let name = format!("/owners/{env}");
        std::env::set_var("OWNER_PUBLIC_KEY", get_secret(&name, None).await);
    }
}

pub async fn ensure_subaccount() {
    let mut subaccount_str = std::env::var("SUBACCOUNT_ID");
    if subaccount_str.is_err() {
        info!("No subaccount in env, loading owner from AWS");
        let env = std::env::var("ENV").expect("ENV must be set");
        let name = format!("/subaccounts/{env}");
        std::env::set_var("SUBACCOUNT_ID", get_secret(&name, None).await);
    }
}

pub async fn setup_env() {
    ensure_env().await;
    let env_name = std::env::var("ENV").unwrap();
    dotenv::from_filename(format!(".env.{env_name}")).expect("Failed to load .env file");
    env_logger::builder().format_timestamp_millis().init();
    ensure_session_key().await;
    ensure_owner().await;
    ensure_subaccount().await;
}
