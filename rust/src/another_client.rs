use dcl_rpc::transports::web_sockets::tungstenite::{TungsteniteWebSocket, WebSocketClient};
use dcl_rpc::{client::RpcClient, transports::web_sockets::WebSocketTransport};
use social_client::{credentials::load_users, FriendshipsServiceClientDefinition};
use social_client::{FriendshipsServiceClient, Payload};

#[tokio::main]
async fn main() {
    // Auth Users
    let (_, user_b) = load_users().await;

    let token = user_b.token;

    // let host = "wss://rpc-social-service.decentraland.org";
    let host = "ws://localhost:8085";

    loop {
        handle_connection(host, &token).await;
    }
}

async fn handle_connection(host: &str, token: &str) {
    loop {
        match WebSocketClient::connect(host).await {
            Ok(client_connection) => {
                let client_transport = WebSocketTransport::new(client_connection.clone());

                let mut client = RpcClient::new(client_transport).await.unwrap();

                let port = client.create_port("friendships").await.unwrap();

                let module = port
                  .load_module::<FriendshipsServiceClient<WebSocketTransport<TungsteniteWebSocket, ()>>>(
                      "FriendshipsService",
                  )
                  .await
                  .unwrap();

                // 4. Listen to updates to my address
                let updates_response = tokio::time::timeout(
                    tokio::time::Duration::from_secs(10),
                    module.subscribe_friendship_events_updates(Payload {
                        synapse_token: Some(token.to_string()),
                    }),
                )
                .await;

                match updates_response {
                    Ok(Ok(mut u)) => {
                        while let Ok(Some(update)) =
                            tokio::time::timeout(tokio::time::Duration::from_secs(10), u.next())
                                .await
                        {
                            println!("> Server Streams > Response > Notifications {update:?}");
                        }
                        println!("Timeout when waiting for response, reconnecting...");
                    }
                    Ok(Err(err)) => {
                        println!("Error handling connection: {err:?}, reconnecting...");
                        break;
                    }
                    Err(_) => {
                        println!("Timeout when waiting for response, reconnecting...");
                        break;
                    }
                }
            }
            Err(err) => {
                println!("Failed to connect, retrying in 5 seconds...");
                println!("Error: {err:?}");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
