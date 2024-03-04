use anyhow::{Result};
use ethers::prelude::{U256, I256};
use bigdecimal::BigDecimal;

pub fn decimal_to_u256(decimal: BigDecimal) -> Result<U256> {
    let u256str = (decimal * BigDecimal::from(10u128.pow(18))).round(0).to_string();
    Ok(U256::from_dec_str(&u256str)?)
}

pub fn decimal_to_i256(decimal: BigDecimal) -> Result<I256> {
    let i256str = (decimal * BigDecimal::from(10u128.pow(18))).round(0).to_string();
    Ok(I256::from_dec_str(&i256str)?)
}
