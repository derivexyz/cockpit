use anyhow::Result;
use bigdecimal::{BigDecimal, RoundingMode};
use ethers::prelude::{I256, U256};
use std::str::FromStr;

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

pub fn u256_to_decimal_with_prec(u256: U256, prec: u32) -> Result<BigDecimal> {
    let u256str = u256.to_string();
    Ok(BigDecimal::from_str(&u256str)?
        / BigDecimal::from(10u128.pow(prec)).with_scale_round(prec as i64, RoundingMode::Down))
}

pub fn i256_to_decimal_with_prec(i256: I256, prec: u32) -> Result<BigDecimal> {
    let i256str = i256.to_string();
    Ok(BigDecimal::from_str(&i256str)?
        / BigDecimal::from(10u128.pow(prec)).with_scale_round(prec as i64, RoundingMode::Down))
}

pub fn u256_to_decimal(u256: U256) -> Result<BigDecimal> {
    u256_to_decimal_with_prec(u256, 18)
}

pub fn i256_to_decimal(i256: I256) -> Result<BigDecimal> {
    i256_to_decimal_with_prec(i256, 18)
}
