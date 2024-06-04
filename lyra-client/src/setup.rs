use crate::aws::get_secret;
use dotenv::dotenv;
use env_logger;
use log::info;

pub async fn ensure_env() {
    let env_name = std::env::var("ENV").expect("ENV must be set");
    match env_name.as_str() {
        "staging" | "prod" => (),
        _ => panic!("Invalid env name"),
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

pub async fn setup_env() {
    dotenv::from_filename(".env").expect("Failed to load .env file");
    ensure_env().await;
    let env_name = std::env::var("ENV").unwrap();
    let env_file = format!(".env.{env_name}");
    dotenv::from_filename(env_file).expect("Failed to load .env.{prod or staging} file");
    env_logger::builder().format_timestamp_millis().init();
    ensure_session_key().await;
    ensure_owner().await;
}
