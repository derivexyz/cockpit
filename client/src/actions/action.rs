use crate::actions::helpers::ModuleData;
use anyhow::Result;
use bigdecimal::BigDecimal;
pub use derive_types::types::orders::{
    Direction, LiquidityRole, OrderParams, OrderResponse, OrderStatus, OrderType, ReplaceParams,
    TimeInForce,
};
use ethers::abi::{AbiDecode, AbiEncode};
use ethers::prelude::{
    Address, EthAbiCodec, EthAbiType, LocalWallet, Signature, Signer, I256, U256,
};
use ethers::utils::hex;
use log::debug;
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
        // API expects UTC nanoseconds within 300s of the server clock.
        let nonce = now.timestamp_nanos_opt().expect("UTC timestamp fits in i64 nanoseconds");
        let signature_expiry_sec = (now + chrono::Duration::seconds(600)).timestamp();
        (nonce, signature_expiry_sec)
    }
    pub fn new<T: AbiEncode + ModuleData>(
        module_data: T,
        subaccount_id: i64,
        signer_address: Address,
    ) -> Result<ActionData> {
        Self::new_with_expiry(module_data, subaccount_id, signer_address, None)
    }

    pub fn new_with_expiry<T: AbiEncode + ModuleData>(
        module_data: T,
        subaccount_id: i64,
        signer_address: Address,
        signature_expiry_sec: Option<i64>,
    ) -> Result<ActionData> {
        let (nonce, default_expiry) = ActionData::get_nonce_and_expiry();
        let signature_expiry_sec = signature_expiry_sec.unwrap_or(default_expiry);
        let module_addr = module_data.address();
        let encoded_data = module_data.encode();
        debug!("encoded_data: {:?}", hex::encode(&encoded_data));
        let hashed_data = ethers::utils::keccak256(&encoded_data);
        debug!("encoded_data_hashed: {:?}", hex::encode(&hashed_data));
        let owner = std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY must be set");
        let action_typehash =
            std::env::var("ACTION_TYPEHASH").expect("ACTION_TYPEHASH must be set");
        let action_typehash = hex::const_decode_to_array::<32>(action_typehash.as_bytes())?;

        // note: if var not set, actions will use the signer of the ws connection sending order
        // alternative signers are used in the vaults where signer = contract
        let signer: Address = match std::env::var("SIGNER_PUBLIC_KEY") {
            Ok(signer) => signer.parse()?,
            Err(_) => signer_address.clone(),
        };
        Ok(ActionData {
            action_typehash,
            subaccount_id: subaccount_id.into(),
            nonce: nonce.into(),
            module: module_addr,
            data: hashed_data,
            expiry: signature_expiry_sec.into(),
            owner: owner.parse()?,
            signer,
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
