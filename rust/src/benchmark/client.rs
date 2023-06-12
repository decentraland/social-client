use std::time::{Duration, Instant};

use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{
        tungstenite::{TungsteniteWebSocket, WebSocketClient},
        WebSocket, WebSocketTransport,
    },
};

use super::args::Args;

pub type TestWebSocketTransport = WebSocketTransport<TungsteniteWebSocket, ()>;

#[derive(Debug)]
pub enum ClientCreationError {
    Connection,
    Transport,
}

pub async fn handle_client(
    args: Args,
) -> Result<(RpcClient<TestWebSocketTransport>, u128, u128), ClientCreationError> {
    let Args { rpc_host, .. } = args;

    let whole_connection = Instant::now();

    let ws = WebSocketClient::connect(&rpc_host).await.map_err(|e| {
        log::error!("Couldn't connect to ws: {e:?}");
        ClientCreationError::Connection
    })?;

    ws.clone().ping_every(Duration::from_secs(30)).await;

    let transport = WebSocketTransport::new(ws);

    let client_connection = Instant::now();
    let client = RpcClient::new(transport)
        .await
        .map_err(|_| ClientCreationError::Transport)?;
    let client_creation_elapsed = client_connection.elapsed().as_millis();
    let whole_connection = whole_connection.elapsed().as_millis();

    Ok((client, whole_connection, client_creation_elapsed))
}
