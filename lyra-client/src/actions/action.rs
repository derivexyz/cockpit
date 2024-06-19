use crate::actions::helpers::ModuleData;
use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, EthAbiCodec, EthAbiType, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;
use log::debug;
pub use orderbook_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
use serde::Deserialize;

#[derive(Clone, Debug, Default, PartialEq, EthAbiType, EthAbiCodec)]
pub struct ActionData {
    pub action_typehash: [u8; 32],
    pub subaccount_id: U256,
    pub nonce: U256,
    pub module: Address,
    pub data: [u8; 32],
    pub expiry: U256,
    pub owner: Address,
    pub signer: Address,
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
