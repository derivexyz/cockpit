use ethers::prelude::{LocalWallet, Signer, EthAbiCodec, Address, U256, I256, EthAbiType};
use bigdecimal::BigDecimal;
use orderbook_types::types::private_order::{
    OrderType, OrderStatus, TimeInForce, Direction, LiquidityRole, PrivateOrderParamsSchema, OrderResponseSchema
};
use crate::utils::{decimal_to_u256, decimal_to_i256};
use anyhow::{Result};
use ethers::abi::AbiEncode;
use ethers::utils::hex;
use orderbook_types::types::channel_ticker_instrument_name_interval::InstrumentTickerSchema;
use orderbook_types::types::public_get_ticker::PublicGetTickerResultSchema;

pub type OrderParams = PrivateOrderParamsSchema;
/// Subset of ticker info required for order signing.
pub trait OrderTicker {
    fn get_name(&self) -> String;
    fn get_max_fee(&self) -> BigDecimal;
    fn get_sub_id(&self) -> Result<U256>;
    fn get_address(&self) -> Result<Address>;
}

impl OrderTicker for &InstrumentTickerSchema {
    fn get_name(&self) -> String {
        self.instrument_name.clone()
    }
    fn get_max_fee(&self) -> BigDecimal {
        BigDecimal::from(3) * &self.taker_fee_rate * &self.index_price
    }
    fn get_sub_id(&self) -> Result<U256> {
        Ok(self.base_asset_sub_id.parse::<u128>()?.into())
    }
    fn get_address(&self) -> Result<Address> {
        Ok(self.base_asset_address.parse()?)
    }
}

impl OrderTicker for &PublicGetTickerResultSchema {
    fn get_name(&self) -> String {
        self.instrument_name.clone()
    }
    fn get_max_fee(&self) -> BigDecimal {
        BigDecimal::from(3) * &self.taker_fee_rate * &self.index_price
    }
    fn get_sub_id(&self) -> Result<U256> {
        Ok(self.base_asset_sub_id.parse::<u128>()?.into())
    }
    fn get_address(&self) -> Result<Address> {
        Ok(self.base_asset_address.parse()?)
    }
}

pub struct OrderArgs {
    pub amount: BigDecimal,
    pub limit_price: BigDecimal,
    pub direction: Direction,
    pub time_in_force: TimeInForce,
    pub order_type: OrderType,
    pub label: String,
    pub mmp: bool
}

pub trait Order {
    fn new_params(
        signer: &LocalWallet,
        ticker: impl OrderTicker,
        subaccount_id: i64,
        args: OrderArgs
    ) -> Result<PrivateOrderParamsSchema>;

    fn sign(&mut self, signer: &LocalWallet, ticker: impl OrderTicker) -> Result<()>;
}


#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
struct TradeData {
    address: Address,
    sub_id: U256,
    limit_price: I256,
    amount: I256,
    max_fee: U256,
    subaccount_id: U256,
    is_bid: bool,
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

impl Order for OrderParams {
    fn new_params(
        signer: &LocalWallet,
        ticker: impl OrderTicker,
        subaccount_id: i64,
        args: OrderArgs
    ) -> Result<OrderParams> {
        let max_fee = ticker.get_max_fee();
        let now = chrono::Utc::now();
        let nonce = now.timestamp_nanos_opt().unwrap();
        let reject_timestamp = (now + chrono::Duration::seconds(5)).timestamp_millis();
        let signature_expiry_sec = (now + chrono::Duration::seconds(345)).timestamp();
        let mut params = OrderParams {
            instrument_name: ticker.get_name(),
            subaccount_id,
            amount: args.amount,
            limit_price: args.limit_price,
            direction: args.direction,
            time_in_force: args.time_in_force,
            order_type: args.order_type,
            mmp: args.mmp,
            max_fee,
            label: args.label,
            nonce,
            reject_timestamp,
            signature_expiry_sec,
            signer: hex::encode_prefixed(signer.address()),
            reduce_only: false,
            replaced_order_id: None,
            referral_code: "".to_string(),
            signature: "".to_string(),
        };
        params.sign(signer, ticker)?;
        Ok(params)
    }

    fn sign(&mut self, signer: &LocalWallet, ticker: impl OrderTicker) -> Result<()> {
        let trade_data = TradeData {
            address: ticker.get_address()?,
            sub_id: ticker.get_sub_id()?,
            limit_price: decimal_to_i256(self.limit_price.clone())?,
            amount: decimal_to_i256(self.amount.clone())?,
            max_fee: decimal_to_u256(self.max_fee.clone())?,
            subaccount_id: self.subaccount_id.into(),
            is_bid: self.direction == Direction::Buy,
        };
        let encoded_data = trade_data.encode();
        let hashed_data = ethers::utils::keccak256(&encoded_data);
        // env var
        let action_typehash = std::env::var("ACTION_TYPEHASH").expect("ACTION_TYPEHASH must be set");
        let action_typehash = hex::const_decode_to_array::<32>(action_typehash.as_bytes())?;
        let domain_sep = std::env::var("DOMAIN_SEPARATOR").expect("DOMAIN_SEPARATOR must be set");
        let domain_sep = hex::decode(domain_sep)?;
        let prefix = hex::decode("1901")?;
        let trade_module = std::env::var("TRADE_ADDRESS").expect("TRADE_ADDRESS must be set");
        let action_data = ActionData {
            action_typehash,
            subaccount_id: self.subaccount_id.into(),
            nonce: self.nonce.into(),
            module: trade_module.parse()?,
            data: hashed_data,
            expiry: self.signature_expiry_sec.into(),
            owner: signer.address(),
            signer: signer.address(),
        };
        let action_hash = ethers::utils::keccak256(&action_data.encode());
        let typed_data_hash = ethers::utils::keccak256(&[prefix, domain_sep, action_hash.into()].concat());
        let signature = signer.sign_hash(typed_data_hash.into())?;
        self.signature = signature.to_string();
        Ok(())
    }
}
