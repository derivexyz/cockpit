pub use crate::web3::tsa::{Action, TSA};
use anyhow::{Error, Result};
use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::prelude::{Abigen, Http, LocalWallet, MiddlewareBuilder, Provider, Signer};
use log::error;
use lyra_client::auth::load_signer_by_name;
use std::env;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

abigen!(ERC20, "./abi/erc20.json");

pub fn generate_tsa_abi() -> Result<()> {
    let abi_source = "./abi/tsa.json";
    let mod_dir = PathBuf::from_str("./lyra-vaults/src/web3").expect("PathBuf::from_str() failed");
    let abi = Abigen::new("TSA", abi_source).expect("Abigen::new() failed");
    let bindings = abi.generate().expect("abi.generate() failed");
    bindings.write_module_in_dir(&mod_dir).expect("write_module_in_dir() failed");
    Ok(())
}

impl<M: ::ethers::providers::Middleware> TSA<M> {
    ///Calls the covered call's contract's `signActionData` (0xa39cc91b) function
    ///This function is to be called on a "legacy" covered call contract instance
    ///New PP contracts (and future contracts) should use the new sign_action_data function
    pub fn sign_action_data_legacy(
        &self,
        action: Action,
    ) -> ::ethers::contract::builders::ContractCall<M, ()> {
        self.deref()
            .method_hash([116, 165, 190, 45], (action,))
            .expect("method not found (this should never happen)")
    }
    pub fn sign_action(
        &self,
        action: Action,
        extra_data: ::ethers::core::types::Bytes,
    ) -> ::ethers::contract::builders::ContractCall<M, ()> {
        let vault_name = std::env::var("VAULT_NAME").expect("VAULT_NAME is not set");
        let tsa_type =
            env::var(format!("{vault_name}_TSA_SIGNING")).expect("TSA_SIGNING is not set");
        match tsa_type.as_str() {
            "legacy" => self.sign_action_data_legacy(action),
            "extra" => self.sign_action(action, extra_data),
            _ => panic!("Invalid TSA_SIGNING value"),
        }
    }
}

// todo: new abi for the new longpp tsa
// todo: RFQ signing needs to pass encoded legs as extraData
// todo somehow minimize the deposit/withdrawal diffs? enum the two?
// if enum, do impl on the enum to return things like client() and other shared methods w/ same name

pub type ProviderWithSigner = NonceManagerMiddleware<SignerMiddleware<Provider<Http>, LocalWallet>>;

pub async fn get_provider_with_signer(
    signer_name: &str,
) -> anyhow::Result<Arc<ProviderWithSigner>> {
    let provider_url = std::env::var("WEB3_PROVIDER").expect("WEB3_PROVIDER is not set");
    let chain_id: u64 = std::env::var("CHAIN_ID").expect("CHAIN_ID is not set").parse().unwrap();
    let signer = load_signer_by_name(signer_name).await.with_chain_id(chain_id);
    let signer_addr = signer.address();
    let provider =
        Provider::<Http>::try_from(provider_url)?.with_signer(signer).nonce_manager(signer_addr);
    Ok(Arc::new(provider))
}

pub async fn get_tsa_contract(
    vault_name: &str,
    signer_name: &str,
) -> anyhow::Result<TSA<ProviderWithSigner>> {
    let provider = get_provider_with_signer(&signer_name).await?;
    let tsa_address: Address =
        std::env::var(format!("{vault_name}_TSA_ADDRESS")).unwrap().parse()?;
    Ok(TSA::new(tsa_address, provider.clone()))
}

pub const GAS_PRICE: u64 = 200000;
