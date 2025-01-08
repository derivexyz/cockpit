use crate::actions::QuoteArgs;
use crate::actions::{new_quote_params, OrderArgs};
use crate::json_rpc::{http_rpc, Notification, WsClient, WsClientExt};
use anyhow::{format_err, Result};
use bigdecimal::RoundingMode::Down;
use bigdecimal::{BigDecimal, One, Zero};
use clap::{Args, Parser, Subcommand};
use comfy_table::Table;
use futures::{future::FutureExt, StreamExt};

use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, Clear, ClearType};
use crossterm::ExecutableCommand;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream, KeyCode},
    execute,
};

use crate::utils::await_tx_settlement;
use crossterm::event::KeyEvent;
use log::{error, info, warn};
use orderbook_types::generated::channel_orderbook_instrument_name_group_depth::OrderbookInstrumentNameGroupDepthPublisherDataSchema;
use orderbook_types::generated::private_get_subaccount::{
    PrivateGetSubaccount, PrivateGetSubaccountParamsSchema, PrivateGetSubaccountResponseSchema,
};
use orderbook_types::generated::public_login::PublicLoginResponseSchema;
use orderbook_types::types::liquidations::{
    AuctionState, AuctionsWatchData, AuctionsWatchResultSchema,
};
use orderbook_types::types::rfqs::{PollQuotesResponse, PollQuotesResult, QuoteResultPublic};
use orderbook_types::types::tickers::{InstrumentTicker, TickerResponse};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::stdout;
use std::io::{stdin, BufRead, BufReader};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

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
    Auctions(CliAuctions),
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
pub struct CliSub {
    pub channels: String,
}

impl CliSub {
    pub async fn subscribe(&self) -> Result<()> {
        info!("Starting market task");
        let channels = serde_json::from_str::<Vec<String>>(&self.channels)?;
        let client = WsClient::new_client().await?;
        let login_res = client.login().await;
        if let Err(e) = login_res {
            error!("Error logging in: {:?}", e);
        }
        client
            .subscribe(channels, |d: Notification<Value>| async move {
                info!("{}", serde_json::to_string_pretty(&d)?);
                Ok(())
            })
            .await?;
        Ok(())
    }
}

#[derive(Args, Debug)]
pub struct CliAuctions {
    #[arg(short, long)]
    pub subaccount_id: i64,
}

type AuctionMap = Arc<Mutex<HashMap<i64, AuctionsWatchResultSchema>>>;

impl CliAuctions {
    pub async fn render_table(
        auctions_map: AuctionMap,
        mut pause_rx: mpsc::Receiver<(i64, bool)>,
    ) -> Result<()> {
        // renders table until a pause rx is received
        // resumes if it gets received again
        let mut selected_subaccount: i64 = 0;
        let mut is_paused = false;
        loop {
            let signal = pause_rx.recv().now_or_never();
            if let Some(Some((id, pause))) = signal {
                selected_subaccount = id;
                is_paused = pause;
            }
            if is_paused {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                continue;
            }
            if selected_subaccount != 0 {
                Self::render_subaccount_once(auctions_map.clone(), selected_subaccount).await?;
            } else {
                Self::render_auctions_once(auctions_map.clone(), None).await?;
            }
            println!("Press ENTER to pause and enable input");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
    pub async fn render_subaccount_once(
        auctions_map: AuctionMap,
        subaccount_id: i64,
    ) -> Result<()> {
        let mut table = Table::new();
        table.set_header(vec!["Asset", "Balance"]);
        let map_lock = auctions_map.lock().await.clone();
        let auction = map_lock.get(&subaccount_id);
        if auction.is_none() {
            error!("No auction found for subaccount_id: {}", subaccount_id);
            return Ok(());
        }
        let auction = auction.unwrap().clone();
        let details = auction.details.unwrap();
        for (asset, balance) in details.subaccount_balances.iter() {
            table.add_row(vec![asset.to_string(), balance.to_string()]);
        }
        Self::render_auctions_once(auctions_map, Some(subaccount_id)).await.unwrap();
        let mut stdout = stdout();
        stdout.execute(Print(table))?.execute(Print("\n"))?;
        Ok(())
    }
    pub async fn render_auctions_once(
        auctions_map: AuctionMap,
        filter_id: Option<i64>,
    ) -> Result<()> {
        let mut table = Table::new();
        table.set_header(vec![
            "Subaccount ID",
            "Discount PnL",
            "Oracle Price",
            "Bid Price",
            "Discount",
            "Liquidatable",
            "Currency",
        ]);
        let map_lock = auctions_map.lock().await.clone();
        for (subaccount_id, auction) in map_lock.into_iter() {
            if let Some(id) = filter_id {
                if id != subaccount_id {
                    continue;
                }
            }
            let details = auction.details.unwrap();
            let disc = if (details.estimated_mtm > BigDecimal::zero()
                && details.estimated_bid_price > BigDecimal::zero())
            {
                let pct = (&details.estimated_bid_price / &details.estimated_mtm
                    * BigDecimal::from(100))
                .with_scale_round(1, Down);
                format!("{}%", pct)
            } else {
                "-".to_string()
            };
            table.add_row(vec![
                subaccount_id.to_string(),
                details.estimated_discount_pnl.to_string(),
                details.estimated_mtm.to_string(),
                details.estimated_bid_price.to_string(),
                disc,
                format!(
                    "{}%",
                    (details.estimated_percent_bid * BigDecimal::from(100))
                        .with_scale_round(1, Down)
                ),
                details.currency.clone().unwrap_or("-".to_string()),
            ]);
        }
        let mut stdout = stdout();
        stdout
            .execute(Clear(ClearType::Purge))?
            .execute(MoveTo(0, 0))?
            .execute(Print(table))?
            .execute(Print("\n"))?;
        Ok(())
    }
    pub async fn listen_for_input(
        subaccount_id: i64,
        client: WsClient,
        auctions_map: AuctionMap,
        pause_tx: mpsc::Sender<(i64, bool)>,
    ) -> Result<()> {
        let mut reader = EventStream::new();
        let mut is_buffer_active = false;
        let mut liquidated_id = 0;
        let mut buffer = String::new();

        while let mut event = reader.next().fuse().await {
            if event.is_none() {
                error!("Null event received");
                break;
            }
            let mut event = event.unwrap();
            if event.is_err() {
                error!("Error reading event: {:?}", event);
            }
            let mut event = event.unwrap();

            if let Event::Key(KeyEvent { code, .. }) = event {
                match code {
                    KeyCode::Enter => {
                        is_buffer_active = !is_buffer_active;
                        if is_buffer_active {
                            let _ = pause_tx.send((liquidated_id, true)).await;
                            if liquidated_id == 0 {
                                println!("Press ENTER to continue or type subaccount id and ENTER to view in detail");
                            } else {
                                println!("Press ENTER to continue or type `exec` + ENTER to execute auction");
                            }
                            buffer = String::new();
                            continue;
                        }
                        match (buffer.as_str(), liquidated_id) {
                            ("back", _) => {
                                liquidated_id = 0;
                            }
                            ("exec", 0) => {
                                error!("No subaccount_id selected");
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                            }
                            ("exec", id) => {
                                println!("Executing auction for subaccount_id: {}", id);
                                let lock = auctions_map.lock().await;
                                let auction = lock.get(&id);
                                if auction.is_none() {
                                    error!("No auction found for subaccount_id: {}", id);
                                    continue;
                                }
                                let auction = auction.unwrap().clone();
                                let details = auction.details.unwrap();
                                let liquidate_res = client
                                    .send_liquidate(subaccount_id, id, BigDecimal::one(), &details)
                                    .await?
                                    .into_result();
                                match liquidate_res {
                                    Ok(v) => {
                                        info!("Liquidation successful: {:?}", v);
                                        let tx_id = v.result.transaction_id;
                                        let tx_res = await_tx_settlement(tx_id).await?;
                                        info!("Transaction settled: {:?}", tx_res);
                                    }
                                    Err(e) => {
                                        error!("Error liquidating: {:?}", e);
                                        // sleep for 10 seconds
                                    }
                                }
                                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                                liquidated_id = 0;
                            }
                            (buffer, _) => {
                                liquidated_id = buffer.parse::<i64>().unwrap_or(0);
                            }
                        }
                        let _ = pause_tx.send((liquidated_id, false)).await;
                    }
                    KeyCode::Char(c) => {
                        if is_buffer_active {
                            buffer.push(c);
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        info!("Starting market task");
        let channels = vec!["auctions.watch".to_string()];
        let sub_client = WsClient::new_client().await?;
        let client = WsClient::new_client().await?;
        let login_res = client.login().await;
        if let Err(e) = login_res {
            error!("Error logging in: {:?}", e);
        }
        let ping_task = client.ping_interval(15);
        let mut auctions_map: AuctionMap = Arc::new(Mutex::new(HashMap::new()));
        let map_ref = auctions_map.clone();
        let sub_handle =
            sub_client.subscribe(channels, |d: Notification<AuctionsWatchData>| async {
                let mut map_lock = map_ref.lock().await;
                for auction in d.params.data.into_iter() {
                    let subaccount_id = auction.subaccount_id;
                    match auction.state {
                        AuctionState::Ended => {
                            map_lock.remove(&subaccount_id);
                        }
                        AuctionState::Ongoing => {
                            map_lock.insert(subaccount_id, auction.clone());
                        }
                    }
                }
                Ok(())
            });
        let (pause_tx, pause_rx) = mpsc::channel::<(i64, bool)>(64);
        let render_handle = tokio::spawn(Self::render_table(auctions_map.clone(), pause_rx));

        // Create a task to listen for user input
        let input_task: JoinHandle<Result<()>> = tokio::spawn(Self::listen_for_input(
            self.subaccount_id,
            client.clone(),
            auctions_map.clone(),
            pause_tx,
        ));

        tokio::select! {
            _ = ping_task => {
                error!("Ping task ended");
            }
            _ = sub_handle => {
                error!("Subscription ended");
            }
            _ = render_handle => {
                error!("Render task ended");
            }
            _ = input_task => {
                error!("Input task ended");
            }
        }

        Ok(())
    }
}

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
                let time = chrono::Utc::now().timestamp_millis();
                let latency = time - d.params.data.timestamp;
                out.push_str(&format!("Latency: {}ms\n", latency));
                out.push_str("~~~~~~~~~~~~~~~~~~~~");
                info!("{}", out);
                if latency > 1000 {
                    warn!("Latency is high: {}", latency);
                    panic!("Latency is high");
                }
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
            Command::Sub(sub) => sub.subscribe().await,
            Command::Auctions(a) => a.start().await,
            Command::Orderbook(ob) => ob.subscribe().await,
        }
    }

    pub async fn call(args: CliRpc) -> Result<()> {
        let params = args.params_to_value().await?;
        let client = WsClient::new_client().await?;
        if args.method.starts_with("private") {
            client.login().await?.into_result()?;
            client.set_cancel_on_disconnect(false).await?.into_result()?;
        }
        let start = chrono::Utc::now().timestamp_millis();
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
            "private/deposit" => {
                let subaccount_id: i64 = params["subaccount_id"].as_i64().unwrap();
                let amount: BigDecimal = params["amount"].as_str().unwrap().parse()?;
                let asset_name: String = params["asset_name"].as_str().unwrap().to_string();
                let subacc = client
                    .send_rpc::<_, PrivateGetSubaccountResponseSchema>(
                        "private/get_subaccount",
                        PrivateGetSubaccountParamsSchema { subaccount_id },
                    )
                    .await?
                    .into_result()?;
                let response = client
                    .deposit(subaccount_id, amount, asset_name, subacc.result.margin_type)
                    .await?
                    .into_result();
                match response {
                    Ok(response) => Ok(serde_json::to_value(response)?),
                    Err(response) => Err(response),
                }
            }
            "private/withdraw" => {
                let subaccount_id: i64 = params["subaccount_id"].as_i64().unwrap();
                let amount: BigDecimal = params["amount"].as_str().unwrap().parse()?;
                let asset_name: String = params["asset_name"].as_str().unwrap().to_string();
                let response =
                    client.withdraw(subaccount_id, amount, asset_name).await?.into_result();
                match response {
                    Ok(response) => Ok(serde_json::to_value(response)?),
                    Err(response) => Err(response),
                }
            }
            "private/send_quote" => {
                let quote_args = serde_json::from_value::<QuoteArgs>(params.clone())?;
                let mut tickers = HashMap::<String, InstrumentTicker>::new();
                for leg in quote_args.legs.iter() {
                    let ticker = http_rpc::<_, TickerResponse>(
                        "public/get_ticker",
                        json!({ "instrument_name": leg.instrument_name }),
                        None,
                    )
                    .await?
                    .into_result()?;
                    tickers.insert(leg.instrument_name.clone(), ticker.result);
                }
                let subaccount_id: i64 = params["subaccount_id"].as_i64().unwrap();
                client.send_quote(&tickers, subaccount_id, quote_args).await?.into_result()
            }
            "private/execute_quote" => {
                let subaccount_id: i64 = params["subaccount_id"].as_i64().unwrap();
                let poll_params = json!({
                    "subaccount_id": subaccount_id,
                    "quote_id": params["quote_id"],
                });
                let quote = client
                    .send_rpc::<_, PollQuotesResponse>("private/poll_quotes", poll_params)
                    .await?
                    .into_result()?;
                let quote = &quote.result.quotes[0];
                let mut tickers = HashMap::<String, InstrumentTicker>::new();
                for leg in quote.legs.iter() {
                    let ticker = http_rpc::<_, TickerResponse>(
                        "public/get_ticker",
                        json!({ "instrument_name": leg.instrument_name }),
                        None,
                    )
                    .await?
                    .into_result()?;
                    tickers.insert(leg.instrument_name.clone(), ticker.result);
                }
                client.send_execute(&tickers, subaccount_id, quote).await?.into_result()
            }
            _ => client.send_rpc::<Value, Value>(&args.method, params).await?.into_result(),
        };
        match res {
            Ok(r) => info!("{}", serde_json::to_string_pretty(&r)?),
            Err(e) => error!("Error: {:?}", e),
        };
        let end = chrono::Utc::now().timestamp_millis();
        info!("RPC time taken: {}ms", end - start);
        Ok(())
    }
}
