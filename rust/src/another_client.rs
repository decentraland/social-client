use dcl_rpc::{
    client::RpcClient,
    transports::web_socket::{WebSocketClient, WebSocketTransport},
};
use social_client::FriendshipsServiceClientDefinition;
use social_client::{FriendshipsServiceClient, Payload};

#[tokio::main]
async fn main() {
    let token = "";
    // let host = "wss://rpc-social-service.decentraland.zone";
    let host = "ws://127.0.0.1:8085";
    let client_connection = WebSocketClient::connect(host).await.unwrap();

    let client_transport = WebSocketTransport::new(client_connection);

    let mut client = RpcClient::new(client_transport).await.unwrap();

    let port = client.create_port("friendships").await.unwrap();

    let module = port
        .load_module::<FriendshipsServiceClient<WebSocketTransport>>("FriendshipsService")
        .await
        .unwrap();

    // 4. Listen to updates to my address
    let updates_response = module
        .subscribe_friendship_events_updates(Payload {
            synapse_token: Some(token.to_string()),
        })
        .await;
    match updates_response {
        Ok(mut u) => {
            while let Some(update) = u.next().await {
                println!("> Server Streams > Response > Notifications {update:?}")
            }
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }
}
