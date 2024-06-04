use crate::aws::get_secret;
use dotenv::dotenv;
use env_logger;
use log::{info, warn};

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
    info!("{}", std::env::var("ENV").unwrap());
    ensure_env().await;
    let env_name = std::env::var("ENV").unwrap();
    let env_consts = format!(".env.constants.{env_name}");
    let env_keys = format!(".env.keys.{env_name}");
    println!("{}", env_consts);
    println!("{}", env_keys);

    dotenv::from_filename(env_consts).expect("Failed to load .env.constants.{} file");
    let key_loaded = dotenv::from_filename(env_keys);
    if key_loaded.is_err() {
        println!("No keys file found for env, expecting them to be in AWS");
    }
    env_logger::builder().format_timestamp_millis().init();
    ensure_session_key().await;
    ensure_owner().await;
}
