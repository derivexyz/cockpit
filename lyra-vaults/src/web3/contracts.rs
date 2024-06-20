use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::middleware::{NonceManagerMiddleware, SignerMiddleware};
use ethers::prelude::{Http, LocalWallet, MiddlewareBuilder, Provider, Signer};
use lyra_client::auth::load_signer_by_name;
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
