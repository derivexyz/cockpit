use anyhow::{Error, Result};
use ethers::prelude::{LocalWallet, Signer};
use ethers::utils::hex;
use futures_util::{FutureExt, SinkExt, StreamExt};
use log::{debug, error, info, warn};
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::time::Instant;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream};
use uuid::Uuid;

use orderbook_types::generated::private_cancel::{
    PrivateCancelParamsSchema, PrivateCancelResponseSchema,
};
use orderbook_types::generated::private_cancel_all::{
    PrivateCancelAll, PrivateCancelAllParamsSchema, PrivateCancelAllResponseSchema,
};
use orderbook_types::generated::private_set_cancel_on_disconnect::{
    PrivateSetCancelOnDisconnectParamsSchema, PrivateSetCancelOnDisconnectResponseSchema,
};
use orderbook_types::generated::public_login::PublicLoginResponseSchema;
use orderbook_types::generated::subscribe::{SubscribeParamsSchema, SubscribeResponseSchema};
use orderbook_types::types::orders::{ReplaceResponse, SendOrderResponse};
use orderbook_types::types::tickers::InstrumentTicker;
use orderbook_types::types::RPCErrorResponse;

use crate::auth::{load_signer, sign_auth_msg};
use crate::orders::{new_order_params, new_replace_params, OrderArgs, OrderParams, ReplaceParams};

type SocketError = tungstenite::error::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Response<T> {
    Success(T),
    Error(RPCErrorResponse),
}

impl<T> Response<T> {
    pub fn into_result(self) -> Result<T> {
        match self {
            Response::Success(s) => Ok(s),
            Response::Error(e) => Err(Error::new(e)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification<D> {
    // method is always "subscription"
    pub method: String,
    pub params: NotificationParams<D>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotificationParams<D> {
    pub channel: String,
    pub data: D,
}

pub struct WsClientState {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    messages: HashMap<Uuid, Value>,
    notifications: Vec<Value>,
    owner: String,
    signer: Option<LocalWallet>,
}

/// A "shareable" (thread safe) lyra websocket client.
pub type WsClient = Arc<Mutex<WsClientState>>;

/// An interface for the wrapped / shared lyra websocket client.
#[allow(async_fn_in_trait)]
pub trait WsClientExt
where
    Self: Sized,
{
    async fn new_client() -> Result<Self>;
    async fn get_owner(&self) -> String;
    async fn get_signer(&self) -> String;
    async fn close(&self) -> Result<()>;
    async fn ping(&self) -> Result<()>;
    async fn ping_interval(&self, interval_sec: u64) -> Result<()>;
    async fn send_rpc<P, R>(&self, method: &str, params: P) -> Result<Response<R>>
    where
        P: Serialize + Debug + Clone,
        R: for<'de> Deserialize<'de> + Debug + Serialize + Clone;
    async fn login(&self) -> Result<Response<PublicLoginResponseSchema>>;
    async fn enable_cancel_on_disconnect(
        &self,
    ) -> Result<Response<PrivateSetCancelOnDisconnectResponseSchema>>;
    async fn send_order(
        &self,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        args: OrderArgs,
    ) -> Result<Response<SendOrderResponse>>;
    async fn send_replace(
        &self,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        to_cancel: Uuid,
        args: OrderArgs,
    ) -> Result<Response<ReplaceResponse>>;
    async fn cancel_all(
        &self,
        subaccount_id: i64,
    ) -> Result<Response<PrivateCancelAllResponseSchema>>;
    async fn subscribe<Fut, Data>(
        &self,
        channels: Vec<String>,
        handler: impl FnMut(Data) -> Fut,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<()>>,
        Data: for<'de> Deserialize<'de> + Debug;
}

impl WsClientExt for WsClient {
    async fn new_client() -> Result<Self> {
        let client = WsClientState::new().await?;
        Ok(Arc::new(Mutex::new(client)))
    }
    async fn get_owner(&self) -> String {
        self.lock().await.owner.clone()
    }
    async fn get_signer(&self) -> String {
        hex::encode_prefixed(self.lock().await.signer.clone().unwrap().address())
    }
    async fn close(&self) -> Result<()> {
        self.lock().await.socket.close(None).await?;
        Ok(())
    }
    async fn ping(&self) -> Result<()> {
        WsClientState::ping(self).await
    }
    async fn ping_interval(&self, interval_sec: u64) -> Result<()> {
        WsClientState::ping_interval(self.clone(), interval_sec).await
    }
    async fn send_rpc<P, R>(&self, method: &str, params: P) -> Result<Response<R>>
    where
        P: Serialize + Debug + Clone,
        R: for<'de> Deserialize<'de> + Debug + Serialize + Clone,
    {
        info!(
            "Sending: {}, params: {}",
            method,
            serde_json::to_string_pretty(&params).unwrap_or("could not serialize".into())
        );
        let this_id = WsClientState::send_to_socket(&self, method, params).await?;
        let res = WsClientState::listen_and_wait_for::<R>(&self, this_id).await;
        if let Ok(res) = &res {
            match res {
                Response::Success(s) => info!("Received RPC result"),
                Response::Error(e) => error!("Received error: {:?}", e),
            }
        } else {
            error!("Error decoding response {:?}", res);
        }
        res
    }
    async fn login(&self) -> Result<Response<PublicLoginResponseSchema>> {
        let wallet = load_signer().await;
        let owner = std::env::var("OWNER_PUBLIC_KEY").expect("OWNER_PUBLIC_KEY must be set");
        let login_params = sign_auth_msg(&wallet).await;
        WsClientState::set_signer(self, wallet).await;
        WsClientState::set_owner(self, owner).await;
        self.send_rpc("public/login", login_params).await
    }
    async fn enable_cancel_on_disconnect(
        &self,
    ) -> Result<Response<PrivateSetCancelOnDisconnectResponseSchema>> {
        self.send_rpc(
            "private/set_cancel_on_disconnect",
            PrivateSetCancelOnDisconnectParamsSchema {
                enabled: true,
                wallet: self.get_owner().await,
            },
        )
        .await
    }
    async fn send_order(
        &self,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        args: OrderArgs,
    ) -> Result<Response<SendOrderResponse>> {
        let order_params =
            WsClientState::new_signed_order(self, ticker, subaccount_id, args).await?;
        self.send_rpc("private/order", order_params).await
    }
    async fn send_replace(
        &self,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        to_cancel: Uuid,
        args: OrderArgs,
    ) -> Result<Response<ReplaceResponse>> {
        let replace_params =
            WsClientState::new_signed_replace(self, ticker, subaccount_id, to_cancel, args).await?;
        self.send_rpc("private/replace", replace_params).await
    }
    async fn cancel_all(
        &self,
        subaccount_id: i64,
    ) -> Result<Response<PrivateCancelAllResponseSchema>> {
        let cancel_params = PrivateCancelAllParamsSchema { subaccount_id };
        self.send_rpc("private/cancel_all", cancel_params).await
    }
    async fn subscribe<Fut, Data>(
        &self,
        channels: Vec<String>,
        handler: impl FnMut(Data) -> Fut,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<()>>,
        Data: for<'de> Deserialize<'de> + Debug,
    {
        let sub_params = SubscribeParamsSchema { channels };
        let sub_res = self.send_rpc::<_, SubscribeResponseSchema>("subscribe", sub_params).await;
        match sub_res {
            Ok(Response::Success(success)) => {
                for (channel, status) in success.result.status.iter() {
                    if status != "ok" {
                        return Err(Error::msg(format!("Subscription error: {channel}")));
                    }
                }
                WsClientState::listen_forever(&self, handler).await
            }
            Ok(Response::Error(e)) => {
                Err(Error::msg(format!("RPC error while subscribing: {:?}", e)))
            }
            Err(e) => Err(e),
        }
    }
}

/// Private methods for WsClientState, used by the extension trait method implementations.
impl WsClientState {
    async fn new() -> Result<Self> {
        let url = std::env::var("WEBSOCKET_ADDRESS").expect("WEBSOCKET_ADDRESS must be set");
        let (socket, _) = connect_async(&url).await?;
        info!("Connected to {}", &url);
        Ok(WsClientState {
            socket,
            messages: HashMap::new(),
            notifications: Vec::new(),
            owner: String::new(),
            signer: None,
        })
    }

    async fn set_signer(client: &WsClient, signer: LocalWallet) {
        let mut client_guard = client.lock().await;
        client_guard.signer = Some(signer);
    }

    async fn set_owner(client: &WsClient, owner: String) {
        let mut client_guard = client.lock().await;
        client_guard.owner = owner;
    }

    async fn new_signed_order(
        client: &WsClient,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        args: OrderArgs,
    ) -> Result<OrderParams> {
        let client_guard = client.lock().await;
        if let Some(signer) = &client_guard.signer {
            Ok(new_order_params(signer, ticker, subaccount_id, args)?)
        } else {
            Err(Error::msg("Not logged in or signer not set"))
        }
    }

    async fn new_signed_replace(
        client: &WsClient,
        ticker: &InstrumentTicker,
        subaccount_id: i64,
        to_cancel: Uuid,
        args: OrderArgs,
    ) -> Result<ReplaceParams> {
        let client_guard = client.lock().await;
        if let Some(signer) = &client_guard.signer {
            Ok(new_replace_params(signer, ticker, subaccount_id, to_cancel, args)?)
        } else {
            Err(Error::msg("Not logged in or signer not set"))
        }
    }

    async fn send_to_socket<P>(client: &WsClient, method: &str, params: P) -> Result<Uuid>
    where
        P: Serialize,
    {
        let this_id = Uuid::new_v4();
        let payload = json!({
            "method": method,
            "params": params,
            "id": this_id
        });
        let item = Message::Text(payload.to_string());
        let mut client_guard = client.lock().await;
        client_guard.socket.send(item).await?;
        Ok(this_id)
    }

    async fn ping(client: &WsClient) -> Result<()> {
        let mut client_guard = client.lock().await;
        client_guard.socket.send(Message::Ping(Vec::new())).await?;
        Ok(())
    }

    async fn listen_and_wait_for<R>(client: &WsClient, id: Uuid) -> Result<Response<R>>
    where
        R: for<'de> Deserialize<'de>,
    {
        let wait_handle = WsClientState::wait_for(client.clone(), id);
        let listen_handle = WsClientState::listen(client.clone());
        tokio::select! {
            val = wait_handle => {
                let val = val?;
                info!("Received: {}", serde_json::to_string_pretty(&val).unwrap_or("could not serialize".into()));
                Ok(serde_json::from_value::<Response<R>>(val)?)
            }
            _ = listen_handle => {
                Err(Error::msg("LyraWsClient::listen() exited before receiving reply"))
            }
        }
    }

    async fn listen(client: WsClient) {
        loop {
            let mut client_guard = client.lock().await;
            let msg = client_guard.socket.next().now_or_never();
            if let Some(Some(msg)) = msg {
                let result = WsClientState::decode_and_insert(msg, &mut client_guard);
                if let Err(e) = result {
                    warn!("decode_and_insert error: {:?}", e);
                }
            } else {
                drop(client_guard);
                tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
            }
        }
    }

    async fn ping_interval(client: WsClient, interval_sec: u64) -> Result<()> {
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(interval_sec)).await;
            WsClientState::ping(&client).await?
        }
    }

    async fn wait_for(client: WsClient, id: Uuid) -> Result<Value> {
        let start = Instant::now();
        loop {
            let mut client_guard = client.lock().await;
            if let Some(json) = client_guard.messages.remove(&id) {
                return Ok(json);
            } else {
                drop(client_guard);
                tokio::time::sleep(tokio::time::Duration::from_micros(100)).await;
            }
            // TODO timeout env var or param
            if start.elapsed().as_secs() > 5 {
                return Err(Error::msg(format!("Timeout waiting for msg id: {id}")));
            }
        }
    }

    async fn listen_forever<Fut, Data>(
        client: &WsClient,
        handler: impl FnMut(Data) -> Fut,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<()>>,
        Data: for<'de> Deserialize<'de> + Debug,
    {
        let listen_handle = WsClientState::listen(client.clone());
        let notification_handle = WsClientState::handle_notifications(client.clone(), handler);
        let ping_handle = WsClientState::ping_interval(client.clone(), 15);
        tokio::select! {
            _ = listen_handle => { Err(Error::msg("listen() exited")) }
            _ = notification_handle => { Err(Error::msg("handle_notifications() exited")) }
            _ = ping_handle => { Err(Error::msg("ping_interval() exited")) }
        }
    }

    async fn handle_notifications<Fut, Data>(
        client: WsClient,
        mut handler: impl FnMut(Data) -> Fut,
    ) -> Result<()>
    where
        Fut: Future<Output = Result<()>>,
        Data: for<'de> Deserialize<'de> + Debug,
    {
        loop {
            let mut client_guard = client.lock().await;
            for v in client_guard.notifications.drain(..) {
                let notification = serde_json::from_value::<Data>(v);
                match notification {
                    Ok(notification) => {
                        handler(notification).await?;
                    }
                    Err(e) => {
                        // todo can consider to put the unhandled values back for other handlers
                        // to handle, but for now can assume the same client will not be used for
                        // multiple handlers at the same time
                        error!("Error in serde_json::from_value::<Notification<Data>>: {:?}", e);
                    }
                }
            }
            drop(client_guard);
            tokio::time::sleep(tokio::time::Duration::from_micros(1000)).await;
        }
    }

    fn decode_and_insert(
        msg: Result<Message, SocketError>,
        state: &mut WsClientState,
    ) -> Result<()> {
        let msg = msg?;
        match msg {
            Message::Ping(_) => {
                debug!("Received: PING");
                return Ok(());
            }
            Message::Pong(_) => {
                debug!("Received: PONG");
                return Ok(());
            }
            Message::Close(_) => {
                warn!("Received: CLOSE");
                return Ok(());
            }
            Message::Text(_) => {}
            Message::Binary(_) => {}
        }
        let json = WsClientState::decode_to_value(msg)?;
        let id_value = json.get("id");
        // TODO max size for # of messages and notifications
        if let Some(id_value) = id_value {
            let id = Uuid::deserialize(id_value)?;
            state.messages.insert(id, json);
        } else {
            let channel = &json["params"]["channel"];
            let channel = channel.as_str();
            if let Some(channel) = channel {
                debug!("Received: {}", channel);
                state.notifications.push(json);
            } else {
                warn!("Received unrecognized message format: {:?}", json);
            }
        }
        Ok(())
    }

    fn decode_to_value(msg: Message) -> Result<Value> {
        let msg_text = msg.to_text()?;
        let json = serde_json::from_str::<Value>(msg_text);
        match json {
            Ok(json) => Ok(json),
            Err(e) => Err(Error::msg(format!("Error in serde_json::from_str::<Value>: {:?}", e))),
        }
    }
}

// TODO a bit ugly to pass two types here, can use one trait but the stub generator needs to be updated
pub async fn http_rpc<P, R>(
    method: &str,
    params: P,
    headers: Option<HeaderMap>,
) -> Result<Response<R>>
where
    P: Serialize + Debug,
    R: for<'de> Deserialize<'de>,
{
    let headers = headers.unwrap_or_default();
    let root = std::env::var("HTTP_ADDRESS").expect("HTTP_ADDRESS must be set");
    let url = format!("{root}/{method}");
    let client = Client::new();
    info!("HTTP Request: {} with {:?}", url, params);
    let response = client.post(url).json(&params).headers(headers).send().await?;
    let response_text = response.text().await?;
    debug!("HTTP Response: {response_text}");
    let parsed_response: Response<R> = serde_json::from_str(&response_text)?;
    Ok(parsed_response)
}
