use crate::json_rpc::{http_rpc, WsClient, WsClientExt};
use crate::orders::OrderArgs;
use anyhow::Result;
use clap::Parser;
use log::info;
use orderbook_types::types::tickers::TickerResponse;
use serde_json::{json, Value};

/// A CLI for interacting with the Lyra API
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CliRpc {
    /// The RPC method to call
    #[arg(short, long)]
    pub method: String,

    #[clap(flatten)]
    pub params: ParamsOrInline,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
pub struct ParamsOrInline {
    /// The path to the params file to read
    #[arg(short, long)]
    pub file: Option<std::path::PathBuf>,

    /// An inline json to use as params, e.g. '{"subaccount_id": 1}'
    #[arg(short, long)]
    pub inline: Option<String>,
}

impl CliRpc {
    async fn params_to_value(&self) -> Result<Value> {
        let params: String = match &self.params.inline {
            Some(s) => s.clone(),
            None => tokio::fs::read_to_string(&self.params.file.clone().unwrap()).await?,
        };
        Ok(serde_json::from_str(&params)?)
    }

    pub async fn call() -> Result<Value> {
        let args = CliRpc::parse();
        info!("Parsed Request {:?}", args);
        let params = args.params_to_value().await?;
        let client = WsClient::new_client().await?;
        client.login().await?;
        match args.method.as_str() {
            "private/order" => {
                let order_args = serde_json::from_value::<OrderArgs>(params.clone())?;
                let ticker = http_rpc::<_, TickerResponse>(
                    "public/get_ticker",
                    json!({ "instrument_name": params["instrument_name"] }),
                    None,
                )
                .await?
                .into_result()?
                .result;
                let subaccount_id: i64 = params["subaccount_id"].as_i64().unwrap();
                let response =
                    client.send_order(&ticker, subaccount_id, order_args).await?.into_result();
                match response {
                    Ok(response) => Ok(serde_json::to_value(response)?),
                    Err(response) => Err(response),
                }
            }
            _ => client.send_rpc::<Value, Value>(&args.method, params).await?.into_result(),
        }
    }
}
