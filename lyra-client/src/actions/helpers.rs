use ethers::prelude::Address;
pub use orderbook_types::generated::private_get_subaccount::MarginType;

pub trait ModuleData {
    fn address(&self) -> Address;
}

// deposit & withdrawal helpers
pub fn get_asset_address(asset_name: &str) -> String {
    let asset_prefix = match asset_name {
        "USDC" => "CASH".to_string(),
        _ => asset_name.to_uppercase() + "_BASE",
    };
    let erc20_env = format!("{}_ADDRESS", asset_prefix);
    std::env::var(erc20_env.clone()).expect(&format!("{} must be set", erc20_env))
}

pub fn get_asset_decimals(asset_name: &str) -> u32 {
    match asset_name {
        "USDC" => 6,
        "BTC" => 8,
        "LBTC" => 8,
        _ => 18,
    }
}

pub fn get_manager_address(asset_name: &str, margin_type: MarginType) -> String {
    let manager_prefix = match margin_type {
        MarginType::Pm => asset_name.to_uppercase() + "_PMRM",
        MarginType::Sm => "SRM".to_string(),
        MarginType::Pm2 => asset_name.to_uppercase() + "_PMRM2",
    };
    let manager_env = format!("{}_ADDRESS", manager_prefix);
    std::env::var(manager_env.clone()).expect(&format!("{} must be set", manager_env))
}
