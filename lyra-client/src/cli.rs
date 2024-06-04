use crate::json_rpc::{http_rpc, Notification, WsClient, WsClientExt};
use crate::orders::OrderArgs;
use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use log::{error, info};
use orderbook_types::generated::channel_orderbook_instrument_name_group_depth::OrderbookInstrumentNameGroupDepthPublisherDataSchema;
use orderbook_types::types::tickers::TickerResponse;
use serde_json::{json, Value};
pub type OrderbookData = OrderbookInstrumentNameGroupDepthPublisherDataSchema;

/// A CLI for interacting with the Lyra API
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Rpc(CliRpc),
    Sub(CliSub),
    Orderbook(CliOrderbook),
}

#[derive(Args, Debug)]
pub struct CliRpc {
    /// The RPC method to call
    #[arg(short, long)]
    pub method: String,

    #[clap(flatten)]
    pub params: ParamsOrInline,
}

#[derive(Args, Debug)]
pub struct CliSub {}

#[derive(Args, Debug)]
pub struct CliOrderbook {
    /// The instrument name to get the orderbook for
    #[arg(short, long)]
    pub instrument: String,
}

impl CliOrderbook {
    pub async fn subscribe(&self) -> Result<()> {
        info!("Starting market task");
        let channels: Vec<String> = vec![format!("orderbook.{}.1.10", self.instrument)];
        let client = WsClient::new_client().await?;
        client
            .subscribe(channels, |mut d: Notification<OrderbookData>| async move {
                // print the orderbook in a nice format
                let mut out = String::new();
                out.push_str("\n~~~~~~~~~~~~~~~~~~~~\n");
                for tick_v in d.params.data.asks.iter().rev() {
                    let tick = tick_v[0].to_string();
                    let tick = format!("{: <10}", tick);
                    out.push_str(&format!("{} | {}\n", tick, tick_v[1]));
                }
                out.push_str("---------------------\n");
                for tick_v in d.params.data.bids.iter() {
                    let tick = tick_v[0].to_string();
                    let tick = format!("{: <10}", tick);
                    out.push_str(&format!("{} | {}\n", tick, tick_v[1]));
                }
                out.push_str("~~~~~~~~~~~~~~~~~~~~");
                info!("{}", out);
                Ok(())
            })
            .await?;
        Ok(())
    }
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

    pub async fn execute() -> Result<()> {
        let args = Cli::parse();
        info!("Parsed Request {:?}", args);
        match args.command {
            Command::Rpc(rpc) => Self::call(rpc).await,
            Command::Sub(_) => Ok(()),
            Command::Orderbook(ob) => ob.subscribe().await,
        }
    }

    pub async fn call(args: CliRpc) -> Result<()> {
        let params = args.params_to_value().await?;
        let client = WsClient::new_client().await?;
        if args.method.starts_with("private") {
            client.login().await?;
        }
        let res = match args.method.as_str() {
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
        };
        match res {
            Ok(r) => info!("{}", serde_json::to_string_pretty(&r)?),
            Err(e) => error!("Error: {:?}", e),
        };
        Ok(())
    }
}
