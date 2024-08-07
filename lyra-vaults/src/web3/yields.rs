use anyhow::{Error, Result};
use bigdecimal::{BigDecimal, One};
use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::prelude::{
    Abigen, Http, LocalWallet, Middleware, MiddlewareBuilder, Provider, Signer, U256,
};
use log::{debug, error, info};
use lyra_client::json_rpc::Response;
use lyra_client::utils::u256_to_decimal;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::sync::Arc;

abigen!(
    ERC4626,
    r#"[
        function convertToAssets(uint256 shares) public view virtual override returns (uint256)
    ]"#,
);

/// Returns price of an interest-bearing token in units of the underlying stable asset.
/// For example, in units of USDe for sUSDe, or in # of ETH for weETH / LRTs.
pub async fn get_price_at_timestamp(base: &str, quote: &str, timestamp: i64) -> Result<BigDecimal> {
    if base == quote {
        return Ok(BigDecimal::one());
    }

    match (base, quote) {
        ("SUSDE", "USDE") => get_susde_price_at_timestamp(timestamp).await,
        ("USDE", "USDC") => Ok(BigDecimal::one()),
        _ => Err(Error::msg(format!("Pair {}-{} not supported", base, quote))),
    }
}

/// Returns the estimated $ growth for a given collateral balance in units of CASH_NAME (USDC)
/// Used by PP vaults to determine the budget for premiums.
/// For example, if sUSDe = $1.08 a week ago and $1.09 now, with 10,000,000 sUSDe TVL,
/// the position would earn 10,000,000 * ($1.09 - $1.08) = $100,000.
/// The growth is then converted to CASH_NAME units using quote -> cash price.
pub async fn get_growth_between(
    base: &str,
    quote: &str,
    balance: &BigDecimal,
    from: i64,
    to: i64,
) -> Result<BigDecimal> {
    let cash_name = env::var("CASH_NAME").unwrap();
    let collat_price_now = get_price_at_timestamp(base, quote, to).await?;
    info!("Collat price now: {}", collat_price_now);
    let collat_price_last = get_price_at_timestamp(base, quote, from).await?;
    info!("Collat price last: {}", collat_price_last);
    let unit_growth_in_quote = (&collat_price_now - &collat_price_last);
    info!("Growth per base in quote units: {}", unit_growth_in_quote);
    let growth_in_quote = balance * unit_growth_in_quote;
    info!("Growth in quote units: {}", growth_in_quote);
    let quote_to_cash = get_price_at_timestamp(quote, &cash_name, to).await?;
    info!("Quote to Cash: {}", quote_to_cash);
    let growth_in_cash = growth_in_quote * quote_to_cash;
    info!("Growth in Cash: {}", growth_in_cash);
    Ok(growth_in_cash)
}

async fn get_susde_price_at_timestamp(timestamp: i64) -> Result<BigDecimal> {
    let provider_url = env::var("MAINNET_PROVIDER")?;
    let address: Address = env::var("SUSDE_MAINNET_ADDRESS")?.parse()?;
    let provider = Provider::<Http>::try_from(provider_url)?;
    let provider = Arc::new(provider);
    let contract = ERC4626::new(address, provider);

    let now = chrono::Utc::now().timestamp();
    let block = match timestamp > now {
        true => contract.client().get_block_number().await?.as_u64(),
        false => {
            let base_url = env::var("BLOCK_ENDPOINT").expect("BLOCK_ENDPOINT is not set");
            let url = format!("{base_url}/{timestamp}");
            let response = http_get(url).await?;
            response["height"].as_u64().ok_or(Error::msg("height not found"))?
        }
    };
    info!("Block at timestamp {}: {}", timestamp, block);
    let one = U256::from(1e18 as u64);
    let price = contract.convert_to_assets(one).block(block).call().await?;
    u256_to_decimal(price)
}

async fn http_get(url: String) -> Result<Value> {
    let client = Client::new();
    info!("HTTP GET: {}", url);
    let response = client.get(url).send().await?;
    let response_text = response.text().await?;
    debug!("HTTP GET Response: {response_text}");
    let parsed_response: Value = serde_json::from_str(&response_text)?;
    Ok(parsed_response)
}
