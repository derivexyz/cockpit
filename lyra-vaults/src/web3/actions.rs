use crate::shared::fetch_ticker;
use anyhow::{Error, Result};
use bigdecimal::BigDecimal;
use ethers::abi::{AbiEncode, Address};
use ethers::middleware::MiddlewareBuilder;
use ethers::prelude::{
    abigen, Abigen, LocalWallet, Middleware, NonceManagerMiddleware, Signer, SignerMiddleware,
    Wallet,
};
use ethers::prelude::{ProviderExt, U256};
use ethers::providers::{Http, Provider};
use log::info;
use lyra_client::actions::{
    ActionData, DepositData, DepositParams, MarginType, ModuleData, OrderArgs, TradeData,
    WithdrawParams, WithdrawalData,
};
use lyra_client::auth::{load_signer_by_name, sign_auth_header};
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use lyra_client::utils::{decimal_to_u256, decimal_to_u256_with_prec};
use orderbook_types::types::orders::{Direction, OrderType, TimeInForce};
use orderbook_types::types::tickers::{InstrumentTicker, TickerResponse};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

abigen!(ERC20, "./abi/erc20.json");
abigen!(TSA, "./abi/tsa.json");
abigen!(
    LRTBase,
    r#"[
        function setSubAccountWL(uint accountId, bool isWhitelisted) external
        ]"#
);

pub type ProviderWithSigner = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub async fn get_provider_with_signer(signer_name: &str) -> Result<Arc<ProviderWithSigner>> {
    let provider_url = std::env::var("WEB3_PROVIDER").expect("WEB3_PROVIDER is not set");
    let chain_id: u64 = std::env::var("CHAIN_ID").expect("CHAIN_ID is not set").parse().unwrap();
    let signer = load_signer_by_name(signer_name).await.with_chain_id(chain_id);
    let signer_addr = signer.address();
    let provider =
        Provider::<Http>::try_from(provider_url)?.with_signer(signer).nonce_manager(signer_addr);
    Ok(Arc::new(provider))
}

pub async fn get_tsa_contract(vault_name: &String) -> Result<TSA<ProviderWithSigner>> {
    let provider = get_provider_with_signer("KEEPER_PRIVATE_KEY").await?;
    let tsa_address: Address =
        std::env::var(format!("{vault_name}_TSA_ADDRESS")).unwrap().parse()?;
    Ok(TSA::new(tsa_address, provider.clone()))
}

pub async fn sign_action<T: AbiEncode + ModuleData + Clone>(
    tsa: &TSA<ProviderWithSigner>,
    data: T,
) -> Result<ActionData> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let action_data = ActionData::new(data.clone(), subaccount_id, tsa.address())?;
    let action = tsa::Action {
        subaccount_id: action_data.subaccount_id,
        nonce: action_data.nonce,
        module: action_data.module,
        data: data.encode().into(),
        expiry: action_data.expiry,
        owner: action_data.owner,
        signer: action_data.signer,
    };
    let call = tsa.sign_action_data(action.clone());
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
    info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
    let tx = tsa.client().get_transaction(receipt.transaction_hash).await?;
    info!("Sent tx: {}\n", serde_json::to_string(&tx)?);
    Ok(action_data)
}

pub async fn sign_deposit(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
    amount: &BigDecimal,
) -> Result<ActionData> {
    let deposit_data = DepositData::new(&amount, &asset_name, MarginType::Sm)?;
    info!("Deposit data: {:?}", deposit_data);
    let action_data = sign_action(tsa, deposit_data.clone()).await?;
    Ok(action_data)
}

pub async fn sign_withdrawal(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
    amount: &BigDecimal,
) -> Result<ActionData> {
    let withdrawal_data = WithdrawalData::new(&amount, &asset_name)?;
    info!("Withdrawal data: {:?}", withdrawal_data);
    let action_data = sign_action(tsa, withdrawal_data.clone()).await?;
    Ok(action_data)
}

pub async fn sign_order(
    tsa: &TSA<ProviderWithSigner>,
    ticker: &InstrumentTicker,
    args: &OrderArgs,
) -> Result<ActionData> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let order_data = TradeData::new(
        ticker,
        subaccount_id,
        args.limit_price.clone(),
        args.amount.clone(),
        args.direction.is_bid(),
    )?;
    info!("Order data: {:?}", order_data);
    let action_data = sign_action(tsa, order_data.clone()).await?;
    Ok(action_data)
}

pub async fn test_order() -> Result<()> {
    let asset_name = String::from("RSWETH");
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let tsa_contract = get_tsa_contract(&asset_name).await?;
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
    let action_data = sign_order(&tsa_contract, &ticker, &order_args).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION_PRIVATE_KEY").await;
    let order = action_data.to_order_params(&session_signer, &ticker, subaccount_id, order_args)?;
    let res = client.send_rpc::<_, Value>("private/order", order).await?;
    info!("Order response: {:?}", res);
    Ok(())
}

pub async fn test_mint() -> Result<()> {
    let asset_name = String::from("RSWETH");
    let erc20_addr: Address = std::env::var(format!("{asset_name}_ADDRESS"))?.parse()?;
    let provider = get_provider_with_signer("KEEPER_PRIVATE_KEY").await?;
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

pub async fn test_deposit() -> Result<()> {
    let asset_name = String::from("RSWETH");
    let tsa_contract = get_tsa_contract(&asset_name).await?;
    let amount = BigDecimal::from_str("1")?;
    let action_data = sign_deposit(&tsa_contract, &asset_name, &amount).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION_PRIVATE_KEY").await;
    let deposit = action_data.to_deposit_params(&session_signer, amount, asset_name)?;
    let res = client.send_rpc::<_, Value>("private/deposit", deposit).await?;
    info!("Deposit response: {:?}", res);
    Ok(())
}

pub async fn test_withdrawal() -> Result<()> {
    let asset_name = String::from("RSWETH");
    let tsa_contract = get_tsa_contract(&asset_name).await?;
    let amount = BigDecimal::from_str("1")?;
    let action_data = sign_withdrawal(&tsa_contract, &asset_name, &amount).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION_PRIVATE_KEY").await;
    let deposit = action_data.to_withdraw_params(&session_signer, amount, asset_name)?;
    let res = client.send_rpc::<_, Value>("private/withdraw", deposit).await?;
    info!("Withdrawal response: {:?}", res);

    Ok(())
}

pub async fn test_header() -> Result<()> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let wallet = load_signer_by_name("KEEPER_PRIVATE_KEY").await;
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
