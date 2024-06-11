use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, EthAbiCodec, EthAbiType, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;
pub use orderbook_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
use uuid::Uuid;

use crate::utils::{decimal_to_i256, decimal_to_u256, decimal_to_u256_with_prec};
use log::debug;
use orderbook_types::generated::channel_subaccount_id_orders;
use orderbook_types::generated::private_deposit::PrivateDepositParamsSchema;
use orderbook_types::generated::private_withdraw::PrivateWithdrawParamsSchema;

use orderbook_types::types::tickers::InstrumentTicker;
use serde::Deserialize;
use std::str::FromStr;

pub use orderbook_types::generated::private_get_subaccount::MarginType;
pub type DepositParams = PrivateDepositParamsSchema;
pub type WithdrawParams = PrivateWithdrawParamsSchema;

#[derive(Debug, Deserialize)]
pub struct OrderArgs {
    pub amount: BigDecimal,
    pub limit_price: BigDecimal,
    pub direction: Direction,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub label: String,
    pub mmp: bool,
}

trait ModuleData {
    fn address(&self) -> Address;
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct TradeData {
    asset_address: Address,
    sub_id: U256,
    limit_price: I256,
    amount: I256,
    max_fee: U256,
    subaccount_id: U256,
    is_bid: bool,
}

impl ModuleData for TradeData {
    fn address(&self) -> Address {
        let addr = std::env::var("TRADE_ADDRESS").expect("TRADE_ADDRESS must be set");
        addr.parse().expect("TRADE_ADDRESS must be a valid module address")
    }
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct DepositData {
    erc20_amount: U256,
    asset_address: Address,
    manager_address: Address,
}

impl ModuleData for DepositData {
    fn address(&self) -> Address {
        let addr = std::env::var("DEPOSIT_ADDRESS").expect("DEPOSIT_ADDRESS must be set");
        addr.parse().expect("DEPOSIT_ADDRESS must be a valid module address")
    }
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct WithdrawalData {
    asset_address: Address,
    erc20_amount: U256,
}

impl ModuleData for WithdrawalData {
    fn address(&self) -> Address {
        let addr = std::env::var("WITHDRAWAL_ADDRESS").expect("WITHDRAWAL_ADDRESS must be set");
        addr.parse().expect("WITHDRAWAL_ADDRESS must be a valid module address")
    }
}

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct ActionData {
    action_typehash: [u8; 32],
    subaccount_id: U256,
    nonce: U256,
    module: Address,
    data: [u8; 32],
    expiry: U256,
    owner: Address,
    signer: Address,
}

impl ActionData {
    fn get_nonce_and_expiry() -> (i64, i64) {
        let now = chrono::Utc::now();
        let nonce = now.timestamp_micros();
        let signature_expiry_sec = (now + chrono::Duration::seconds(600)).timestamp();
        (nonce, signature_expiry_sec)
    }

    pub fn new<T: AbiEncode + ModuleData>(
        module_data: T,
        subaccount_id: i64,
        signer_address: Address,
    ) -> Result<ActionData> {
        let (nonce, signature_expiry_sec) = ActionData::get_nonce_and_expiry();
        let module_addr = module_data.address();
        let encoded_data = module_data.encode();
        debug!("encoded_data: {:?}", hex::encode(&encoded_data));
        let hashed_data = ethers::utils::keccak256(&encoded_data);
        debug!("encoded_data_hashed: {:?}", hex::encode(&hashed_data));
        let owner = std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY must be set");
        let action_typehash =
            std::env::var("ACTION_TYPEHASH").expect("ACTION_TYPEHASH must be set");
        let action_typehash = hex::const_decode_to_array::<32>(action_typehash.as_bytes())?;
        Ok(ActionData {
            action_typehash,
            subaccount_id: subaccount_id.into(),
            nonce: nonce.into(),
            module: module_addr,
            data: hashed_data,
            expiry: signature_expiry_sec.into(),
            owner: owner.parse()?,
            signer: signer_address,
        })
    }

    fn action_hash(self) -> [u8; 32] {
        let action_hash = ethers::utils::keccak256(self.encode());
        debug!("action_hash: {:?}", hex::encode(&action_hash));
        action_hash
    }

    pub fn hash(self) -> [u8; 32] {
        let domain_sep = std::env::var("DOMAIN_SEPARATOR").expect("DOMAIN_SEPARATOR must be set");
        let domain_sep = hex::decode(domain_sep).expect("hex::decode failed for DOMAIN_SEPARATOR");
        let prefix = hex::decode("1901").expect("hex::decode failed for prefix");
        let action_hash = self.action_hash();
        let hash = ethers::utils::keccak256(&[prefix, domain_sep, action_hash.into()].concat());
        debug!("typed_data_hash: {:?}", hex::encode(&hash));
        hash
    }
}

pub fn new_order_action(
    subaccount_id: i64,
    limit_price: BigDecimal,
    amount: BigDecimal,
    is_bid: bool,
    max_fee: BigDecimal,
    signer: &LocalWallet,
    ticker: &InstrumentTicker,
) -> Result<ActionData> {
    let trade_data = TradeData {
        asset_address: ticker.base_asset_address.parse()?,
        sub_id: ticker.base_asset_sub_id.parse::<u128>()?.into(),
        limit_price: decimal_to_i256(limit_price)?,
        amount: decimal_to_i256(amount)?,
        max_fee: decimal_to_u256(max_fee)?,
        subaccount_id: subaccount_id.into(),
        is_bid,
    };
    let action_data = ActionData::new(trade_data, subaccount_id, signer.address())?;
    Ok(action_data)
}

pub fn new_order_params(
    signer: &LocalWallet,
    ticker: &InstrumentTicker,
    subaccount_id: i64,
    args: OrderArgs,
) -> Result<OrderParams> {
    let max_fee = ticker.get_max_fee();
    let order_action = new_order_action(
        subaccount_id,
        args.limit_price.clone(),
        args.amount.clone(),
        args.direction == Direction::Buy,
        max_fee.clone(),
        signer,
        ticker,
    )?;
    let params = OrderParams {
        instrument_name: ticker.instrument_name.clone(),
        subaccount_id,
        amount: args.amount,
        limit_price: args.limit_price,
        direction: args.direction,
        time_in_force: args.time_in_force,
        order_type: args.order_type,
        mmp: args.mmp,
        max_fee,
        label: args.label,
        nonce: order_action.nonce.as_u64() as i64,
        reject_timestamp: (chrono::Utc::now() + chrono::Duration::seconds(5)).timestamp_millis(),
        signature_expiry_sec: order_action.expiry.as_u64() as i64,
        signer: hex::encode_prefixed(signer.address()),
        reduce_only: false,
        replaced_order_id: None,
        referral_code: "".to_string(),
        signature: signer.sign_hash(order_action.hash().into())?.to_string(),
    };
    Ok(params)
}

pub fn new_replace_params(
    signer: &LocalWallet,
    ticker: &InstrumentTicker,
    subaccount_id: i64,
    order_id_to_cancel: Uuid,
    args: OrderArgs,
) -> Result<ReplaceParams> {
    let max_fee = ticker.get_max_fee();
    let order_action = new_order_action(
        subaccount_id,
        args.limit_price.clone(),
        args.amount.clone(),
        args.direction == Direction::Buy,
        max_fee.clone(),
        signer,
        ticker,
    )?;
    let params = ReplaceParams {
        instrument_name: ticker.instrument_name.clone(),
        subaccount_id,
        amount: args.amount,
        limit_price: args.limit_price,
        direction: args.direction,
        time_in_force: args.time_in_force,
        order_type: args.order_type,
        mmp: args.mmp,
        max_fee,
        label: args.label,
        nonce: order_action.nonce.as_u64() as i64,
        reject_timestamp: (chrono::Utc::now() + chrono::Duration::seconds(5)).timestamp_millis(),
        signature_expiry_sec: order_action.expiry.as_u64() as i64,
        signer: hex::encode_prefixed(signer.address()),
        reduce_only: false,
        replaced_order_id: None,
        referral_code: "".to_string(),
        signature: signer.sign_hash(order_action.hash().into())?.to_string(),
        expected_filled_amount: None,
        nonce_to_cancel: None,
        order_id_to_cancel: Some(order_id_to_cancel),
    };
    Ok(params)
}

fn get_asset_address(asset_name: &str) -> String {
    let asset_prefix = match asset_name {
        "USDC" => "CASH".to_string(),
        _ => asset_name.to_uppercase() + "_BASE",
    };
    let erc20_env = format!("{}_ADDRESS", asset_prefix);
    std::env::var(erc20_env.clone()).expect(&format!("{} must be set", erc20_env))
}

fn get_asset_decimals(asset_name: &str) -> u32 {
    match asset_name {
        "USDC" => 6,
        "BTC" => 8,
        _ => 18,
    }
}

fn get_manager_address(asset_name: &str, margin_type: MarginType) -> String {
    let manager_prefix = match margin_type {
        MarginType::Pm => asset_name.to_uppercase() + "_PMRM",
        MarginType::Sm => "SRM".to_string(),
    };
    let manager_env = format!("{}_ADDRESS", manager_prefix);
    std::env::var(manager_env.clone()).expect(&format!("{} must be set", manager_env))
}

pub fn new_deposit_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    amount: BigDecimal,
    asset_name: String,
    margin_type: MarginType,
) -> Result<DepositParams> {
    // todo when erc20 details are added - can use them
    let asset_address = get_asset_address(&asset_name);
    let asset_decimals = get_asset_decimals(&asset_name);
    let manager_address = get_manager_address(&asset_name, margin_type);

    let deposit_data = DepositData {
        erc20_amount: decimal_to_u256_with_prec(amount.clone(), asset_decimals)?,
        asset_address: asset_address.parse()?,
        manager_address: manager_address.parse()?,
    };
    let action_data = ActionData::new(deposit_data, subaccount_id, signer.address())?;
    let params = DepositParams {
        subaccount_id,
        amount,
        asset_name,
        nonce: action_data.nonce.as_u64() as i64,
        signature_expiry_sec: action_data.expiry.as_u64() as i64,
        signer: hex::encode_prefixed(signer.address()),
        signature: signer.sign_hash(action_data.hash().into())?.to_string(),
    };
    Ok(params)
}

pub fn new_withdraw_params(
    signer: &LocalWallet,
    subaccount_id: i64,
    amount: BigDecimal,
    asset_name: String,
) -> Result<WithdrawParams> {
    // todo when erc20 details are added - can use them
    let asset_address = get_asset_address(&asset_name);
    let asset_decimals = get_asset_decimals(&asset_name);

    let withdrawal_data = WithdrawalData {
        asset_address: asset_address.parse()?,
        erc20_amount: decimal_to_u256_with_prec(amount.clone(), asset_decimals)?,
    };
    let action_data = ActionData::new(withdrawal_data, subaccount_id, signer.address())?;
    let params = WithdrawParams {
        subaccount_id,
        amount,
        asset_name,
        nonce: action_data.nonce.as_u64() as i64,
        signature_expiry_sec: action_data.expiry.as_u64() as i64,
        signer: hex::encode_prefixed(signer.address()),
        signature: signer.sign_hash(action_data.hash().into())?.to_string(),
    };
    Ok(params)
}
