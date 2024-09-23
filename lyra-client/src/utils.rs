use crate::json_rpc::http_rpc;
use anyhow::Result;
use bigdecimal::{BigDecimal, RoundingMode};
use ethers::prelude::{I256, U256};
use orderbook_types::generated::public_get_transaction::{
    PublicGetTransactionParamsSchema, PublicGetTransactionResponseSchema,
    PublicGetTransactionResultSchema, Status,
};
use std::str::FromStr;
use uuid::Uuid;

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

pub async fn await_tx_settlement(transaction_id: Uuid) -> Result<PublicGetTransactionResultSchema> {
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
            Status::Settled | Status::Reverted => {
                return Ok(tx_res.result);
            }
            _ => {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }
    }
}
