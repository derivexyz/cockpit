use crate::web3;
use crate::web3::{get_provider_with_signer, get_tsa_contract, ERC20};
use anyhow::Error;
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use ethers::prelude::Middleware;
use log::info;
use lyra_client::actions::OrderArgs;
use lyra_client::auth::{load_signer_by_name, sign_auth_header};
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use lyra_client::utils::decimal_to_u256;
use orderbook_types::types::orders::{Direction, OrderType, TimeInForce};
use orderbook_types::types::tickers::TickerResponse;
use serde_json::{json, Value};
use std::str::FromStr;

pub async fn test_order() -> anyhow::Result<()> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let tsa_contract = get_tsa_contract("RSWETH", "SESSION").await?;
    let order_args = OrderArgs {
        amount: BigDecimal::from_str("1")?,
        limit_price: BigDecimal::from_str("9")?,
        direction: Direction::Sell,
        time_in_force: TimeInForce::Gtc,
        order_type: OrderType::Limit,
        label: String::from("test-vault"),
        mmp: false,
    };
    let ticker = http_rpc::<_, TickerResponse>(
        "public/get_ticker",
        json!({ "instrument_name": "ETH-20240621-3700-C" }),
        None,
    )
    .await?
    .into_result()?
    .result;
    let action_data = web3::sign_order(&tsa_contract, &ticker, &order_args).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION").await;
    let order = action_data.to_order_params(&session_signer, &ticker, order_args)?;
    let res = client.send_rpc::<_, Value>("private/order", order).await?;
    info!("Order response: {:?}", res);
    Ok(())
}

pub async fn test_mint() -> anyhow::Result<()> {
    let asset_name = String::from("RSWETH");
    let erc20_addr: Address = std::env::var(format!("{asset_name}_ADDRESS"))?.parse()?;
    let provider = get_provider_with_signer("KEEPER").await?;
    let erc20_contract = ERC20::new(erc20_addr, provider);
    let mint_to: Address = Address::from_str("0xb94dCcaDf0c72E4A472f6ccf07595Ba27B49e033")?;
    let amount = BigDecimal::from_str("500")?;
    let keeper_addr = erc20_contract.client().default_sender().unwrap();
    info!("Minting {} to {} by {}", amount, mint_to, keeper_addr);
    let call = erc20_contract.mint(mint_to, decimal_to_u256(amount)?);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
    info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
    let tx = erc20_contract.client().get_transaction(receipt.transaction_hash).await?;
    info!("Mint tx: {:?}", tx);
    Ok(())
}

pub async fn test_deposit() -> anyhow::Result<()> {
    let asset_name = String::from("RSWETH");
    let tsa_contract = get_tsa_contract(&asset_name, "SESSION").await?;
    let amount = BigDecimal::from_str("1")?;
    let action_data = web3::sign_deposit(&tsa_contract, &asset_name, &amount).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION").await;
    let deposit = action_data.to_deposit_params(&session_signer, amount, asset_name)?;
    let res = client.send_rpc::<_, Value>("private/deposit", deposit).await?;
    info!("Deposit response: {:?}", res);
    Ok(())
}

pub async fn test_withdrawal() -> anyhow::Result<()> {
    let asset_name = String::from("RSWETH");
    let tsa_contract = get_tsa_contract(&asset_name, "SESSION").await?;
    let amount = BigDecimal::from_str("1")?;
    let action_data = web3::sign_withdrawal(&tsa_contract, &asset_name, &amount).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION").await;
    let deposit = action_data.to_withdraw_params(&session_signer, amount, asset_name)?;
    let res = client.send_rpc::<_, Value>("private/withdraw", deposit).await?;
    info!("Withdrawal response: {:?}", res);

    Ok(())
}

pub async fn test_header() -> anyhow::Result<()> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let wallet = load_signer_by_name("KEEPER").await;
    let header = sign_auth_header(&wallet).await;
    info!("Header: {:?}", header);
    http_rpc::<Value, Value>(
        "private/get_subaccount",
        json!({"subaccount_id": subaccount_id}),
        Some(header),
    )
    .await?;
    Ok(())
}

pub async fn test_initiate_deposit() -> anyhow::Result<()> {
    let asset_name = String::from("RSWETH");
    let amount = BigDecimal::from_str("69")?;
    let tsa_contract = get_tsa_contract(&asset_name, "SESSION").await?;

    let addr: Address = tsa_contract.client().default_sender().unwrap();
    let call = tsa_contract.initiate_deposit(decimal_to_u256(amount)?, addr);
    let static_call = call.call().await?;
    info!("Initiate deposit call: {:?}", static_call);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
    info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
    let tx = tsa_contract.client().get_transaction(receipt.transaction_hash).await?;
    info!("Initiate deposit tx: {:?}", tx);
    Ok(())
}

pub async fn test_initiate_withdrawal() -> anyhow::Result<()> {
    let asset_name = String::from("RSWETH");
    let amount = BigDecimal::from_str("33.331231322")?;
    let tsa_contract = get_tsa_contract(&asset_name, "SESSION").await?;

    let call = tsa_contract.request_withdrawal(decimal_to_u256(amount)?);
    let static_call = call.call().await?;
    info!("Initiate wd call: {:?}", static_call);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
    info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
    let tx = tsa_contract.client().get_transaction(receipt.transaction_hash).await?;
    info!("Initiate withdrawal tx: {:?}", tx);
    Ok(())
}
