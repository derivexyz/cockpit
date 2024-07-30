use crate::helpers::{fetch_ticker, get_single_balance, sync_subaccount};
use crate::market::new_market_state;
pub use crate::web3::contracts::{
    get_provider_with_signer, get_tsa_contract, ProviderWithSigner, ERC20, TSA,
};
use crate::web3::tsa;
use crate::web3::{process_deposit_events, MAX_TO_PROCESS_PER_CALL};
use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, Zero};
use ethers::abi::{AbiEncode, Address};
use ethers::middleware::MiddlewareBuilder;
use ethers::prelude::{
    abigen, Abigen, Bytes, Event, LocalWallet, Middleware, NonceManagerMiddleware, Signer,
    SignerMiddleware, Wallet, I256, U64,
};
use ethers::prelude::{ProviderExt, U256};
use ethers::providers::{Http, Provider};
use log::{info, warn};
use lyra_client::actions::order::TradeData;
use lyra_client::actions::{
    ActionData, DepositData, DepositParams, ExecuteData, MarginType, ModuleData, OrderArgs,
    QuoteData, WithdrawParams, WithdrawalData,
};
use lyra_client::auth::{load_signer_by_name, sign_auth_header};
use lyra_client::json_rpc::{http_rpc, WsClient, WsClientExt};
use lyra_client::utils::{decimal_to_u256, decimal_to_u256_with_prec, u256_to_decimal};
use orderbook_types::generated::private_deposit::PrivateDepositResponseSchema;
use orderbook_types::generated::private_withdraw::PrivateWithdrawResponseSchema;
use std::collections::HashMap;

use crate::web3::contracts::GAS_PRICE;
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::generated::public_get_transaction::{
    PublicGetTransactionParamsSchema, PublicGetTransactionResponseSchema, Status,
};
use orderbook_types::types::orders::{Direction, OrderType, TimeInForce};
use orderbook_types::types::rfqs::QuoteResultPublic;
use orderbook_types::types::tickers::{InstrumentTicker, TickerResponse};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

const BLOCK_SEC: u64 = 2;
const WITHDRAW_BUFFER_FACTOR: &str = "1.01";

pub async fn sign_action<T: AbiEncode + ModuleData + Clone>(
    tsa: &TSA<ProviderWithSigner>,
    data: T,
    extra_data: Bytes,
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
    let call = tsa.sign_action_data(action.clone(), extra_data).gas_price(GAS_PRICE);
    let pending_tx = call.send().await?;
    let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
    info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
    let tx = tsa.client().get_transaction(receipt.transaction_hash).await?;
    info!("Sent tx: {}\n", serde_json::to_string(&tx)?);
    Ok(action_data)
}

pub async fn get_subaccount_id(vault_name: &String) -> Result<i64> {
    let tsa = get_tsa_contract(vault_name, "SESSION").await?;
    let subaccount_id = tsa.sub_account().call().await?;
    Ok(subaccount_id.as_u64() as i64)
}

pub async fn get_erc20_balance_of_tsa(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
) -> Result<U256> {
    let provider = tsa.client();
    let erc20_address: Address = std::env::var(format!("{asset_name}_ADDRESS")).unwrap().parse()?;
    let erc20_contract = ERC20::new(erc20_address, provider.clone());
    Ok(erc20_contract.balance_of(tsa.address()).call().await?)
}

pub async fn get_balance_to_deposit(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
) -> Result<BigDecimal> {
    let balance = get_erc20_balance_of_tsa(tsa, asset_name).await?;
    let pending_deposits = tsa.total_pending_deposits().call().await?;
    let available_balance = balance - pending_deposits;
    u256_to_decimal(available_balance)
}

pub async fn get_balance_to_withdraw(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
) -> Result<BigDecimal> {
    let balance = get_erc20_balance_of_tsa(tsa, asset_name).await?;
    let pending_deposits = tsa.total_pending_deposits().call().await?;
    let pending_withdrawals = tsa.total_pending_withdrawals().call().await?;
    let tsa_params = tsa.get_tsa_params().call().await?;
    let scale = tsa_params.withdraw_scale;
    let scaled_pending = scale * pending_withdrawals / U256::from(1e18 as i64);
    let shares_value = tsa.get_shares_value(scaled_pending).call().await?;
    let extra_balance_needed = shares_value - (balance - pending_deposits);
    let buffer = BigDecimal::from_str(WITHDRAW_BUFFER_FACTOR)?;
    Ok(u256_to_decimal(extra_balance_needed)? * buffer)
}

pub async fn sign_deposit(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
    amount: &BigDecimal,
) -> Result<ActionData> {
    let deposit_data = DepositData::new(&amount, &asset_name, MarginType::Sm)?;
    info!("Deposit data: {:?}", deposit_data);
    let action_data = sign_action(tsa, deposit_data.clone(), Bytes::new()).await?;
    Ok(action_data)
}

pub async fn process_deposits_once(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: String,
) -> Result<()> {
    process_deposit_events(tsa).await?;

    let balance = get_balance_to_deposit(tsa, &asset_name).await?;
    if balance <= BigDecimal::zero() {
        // todo some magic numbers -> env (e.g. 5 sec wait time here)
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        return Ok(());
    }

    let action_data = sign_deposit(tsa, &asset_name, &balance).await?;
    let client = WsClient::new_client().await?;
    client.login().await?;
    let session_signer = load_signer_by_name("SESSION").await;
    let deposit = action_data.to_deposit_params(&session_signer, balance, asset_name.clone())?;
    let deposit_res = client
        .send_rpc::<_, PrivateDepositResponseSchema>("private/deposit", deposit)
        .await?
        .into_result()?;
    info!("Deposit response: {:?}", deposit_res);
    await_tx_settlement(deposit_res.result.transaction_id).await
}

pub async fn process_deposits_forever(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: String,
) -> Result<()> {
    loop {
        process_deposits_once(tsa, asset_name.clone()).await?;
    }
}

pub async fn sign_withdrawal(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: &String,
    amount: &BigDecimal,
) -> Result<ActionData> {
    let withdrawal_data = WithdrawalData::new(&amount, &asset_name)?;
    info!("Withdrawal data: {:?}", withdrawal_data);
    let action_data = sign_action(tsa, withdrawal_data.clone(), Bytes::new()).await?;
    Ok(action_data)
}

/// clears the total_pending_withdrawals using an existing LRT balance of the vault
async fn process_withdrawals_onchain(
    tsa: &TSA<ProviderWithSigner>,
    asset_name: String,
) -> Result<()> {
    loop {
        let balance = get_erc20_balance_of_tsa(tsa, &asset_name).await?;
        let pending = tsa.total_pending_withdrawals().call().await?;
        if balance == U256::from(0) || pending == U256::from(0) {
            info!("Balance & pending withdrawals for {}: {} & {}", asset_name, balance, pending);
            break;
        }
        let call = tsa
            .process_withdrawal_requests(U256::from(MAX_TO_PROCESS_PER_CALL))
            .gas_price(GAS_PRICE);
        let pending_tx = call.send().await?;
        let receipt = pending_tx.await?.ok_or(Error::msg("Failed"))?;
        info!("Tx receipt: {}", serde_json::to_string(&receipt)?);
        let tx = tsa.client().get_transaction(receipt.transaction_hash).await?;
        info!("Sent tx: {}\n", serde_json::to_string(&tx)?);
    }
    Ok(())
}

pub async fn process_withdrawals(tsa: &TSA<ProviderWithSigner>, asset_name: String) -> Result<()> {
    let subaccount_id: i64 = std::env::var("SUBACCOUNT_ID")?.parse()?;
    let lrt_balance = get_single_balance(subaccount_id, &asset_name).await?;
    info!("Orderbook LRT balance for {}: {:?}", asset_name, lrt_balance);
    if lrt_balance == BigDecimal::zero() {
        warn!("No spot balance found for {}", asset_name);
        return Ok(());
    }

    let want_to_withdraw = get_balance_to_withdraw(tsa, &asset_name).await?;
    info!("Want to withdraw for {}: {:?}", asset_name, want_to_withdraw);

    let can_withdraw = want_to_withdraw.min(lrt_balance.clone());
    if can_withdraw <= BigDecimal::zero() {
        // vault could still have some stray balance it could use to process a few withdrawals
        info!("Can withdraw for {:?} for {}", can_withdraw, asset_name);
        process_withdrawals_onchain(tsa, asset_name).await?;
        return Ok(());
    }

    let action_data = sign_withdrawal(&tsa, &asset_name, &can_withdraw).await?;
    let session_signer = load_signer_by_name("SESSION").await;
    let headers = sign_auth_header(&session_signer).await;
    let withdrawal =
        action_data.to_withdraw_params(&session_signer, can_withdraw, asset_name.clone())?;
    let withdrawal_res =
        http_rpc::<_, PrivateWithdrawResponseSchema>("private/withdraw", withdrawal, Some(headers))
            .await?
            .into_result()?;
    info!("Withdrawal response: {:?}", withdrawal_res);
    await_tx_settlement(withdrawal_res.result.transaction_id).await?;
    process_withdrawals_onchain(tsa, asset_name).await
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
    let action_data = sign_action(tsa, order_data.clone(), Bytes::new()).await?;
    Ok(action_data)
}

pub async fn sign_execute_quote(
    tsa: &TSA<ProviderWithSigner>,
    tickers: &HashMap<String, InstrumentTicker>,
    quote: &QuoteResultPublic,
) -> Result<ActionData> {
    let quote_data = QuoteData::from_quote_result(quote, tickers)?;
    let execute_data = quote_data.clone().into_execute();
    info!("Execute data: {:?}", execute_data);
    let extra_data = Bytes::from(quote_data.encoded_legs());
    let action_data = sign_action(tsa, execute_data.clone(), extra_data).await?;
    Ok(action_data)
}

pub async fn await_tx_settlement(transaction_id: Uuid) -> Result<()> {
    loop {
        let tx_params = PublicGetTransactionParamsSchema { transaction_id };
        let tx_res = http_rpc::<_, PublicGetTransactionResponseSchema>(
            "public/get_transaction",
            tx_params,
            None,
        )
        .await?
        .into_result()?;
        match tx_res.result.status {
            Status::Settled | Status::Reverted => {
                break;
            }
            _ => {
                tokio::time::sleep(tokio::time::Duration::from_secs(BLOCK_SEC)).await;
            }
        }
    }
    Ok(())
}
