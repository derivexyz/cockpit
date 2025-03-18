use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::prelude::{Http, JsonRpcClient, Middleware, MiddlewareBuilder, Provider, Signer};
use std::sync::Arc;

abigen!(
    ChainlinkAggregator,
    r#"[
        function latestAnswer() public view virtual override returns (int256)
        function decimals() external vie override returns (uint8)
    ]"#,
);

pub async fn get_chainlink_price(currency: &str) -> Result<BigDecimal> {
    let provider_url = std::env::var("BASE_MAINNET_PROVIDER")?;
    let oracle_addr_name = format!("BASE_{}_FEED", currency);
    let oracle_addr: Address = std::env::var(&oracle_addr_name)?.parse()?;
    let provider = Provider::<Http>::try_from(provider_url)?;
    let provider = Arc::new(provider);
    let contract = ChainlinkAggregator::new(oracle_addr, provider);
    let rate = contract.latest_answer().call().await?;
    let prec = contract.decimals().call().await?;
    lyra_client::utils::i256_to_decimal_with_prec(rate, prec.into())
}
