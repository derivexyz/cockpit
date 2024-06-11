use anyhow::Result;
use bigdecimal::BigDecimal;
use ethers::prelude::{I256, U256};

pub fn decimal_to_u256(decimal: BigDecimal) -> Result<U256> {
    decimal_to_u256_with_prec(decimal, 18)
}

pub fn decimal_to_i256(decimal: BigDecimal) -> Result<I256> {
    decimal_to_i256_with_prec(decimal, 18)
}

pub fn decimal_to_u256_with_prec(decimal: BigDecimal, prec: u32) -> Result<U256> {
    let u256str = (decimal * BigDecimal::from(10u128.pow(prec))).round(0).to_string();
    Ok(U256::from_dec_str(&u256str)?)
}

pub fn decimal_to_i256_with_prec(decimal: BigDecimal, prec: u32) -> Result<I256> {
    let i256str = (decimal * BigDecimal::from(10u128.pow(prec))).round(0).to_string();
    Ok(I256::from_dec_str(&i256str)?)
}
