use crate::json_rpc::http_rpc;
use anyhow::{bail, Result};
use bigdecimal::{BigDecimal, RoundingMode};
use derive_types::generated::public_get_transaction::{
    PublicGetTransactionParamsSchema, PublicGetTransactionResponseSchema,
    PublicGetTransactionResultSchema, Status,
};
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

pub async fn await_tx_settlement(op_uuid: impl AsRef<str>) -> Result<PublicGetTransactionResultSchema> {
    let op_uuid = op_uuid.as_ref().to_string();
    let transaction_id = uuid::Uuid::parse_str(&op_uuid)?;
    loop {
        let tx_params = PublicGetTransactionParamsSchema { transaction_id };
        let tx_res = http_rpc::<_, PublicGetTransactionResponseSchema>(
            "public/get_transaction",
            tx_params,
            None,
        )
        .await?
        .into_result()?;
        match tx_res.result.status {
            Status::Settled => return Ok(tx_res.result),
            Status::Reverted | Status::Ignored | Status::TimedOut => {
                bail!("operation {op_uuid} failed: {}", tx_res.result.status.to_string());
            }
            Status::Requested | Status::Pending => {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}
