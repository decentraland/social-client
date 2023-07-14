use dcl_rpc::transports::web_sockets::tungstenite::{TungsteniteWebSocket, WebSocketClient};
use dcl_rpc::{client::RpcClient, transports::web_sockets::WebSocketTransport};
use social_client::{credentials::load_users, FriendshipsServiceClientDefinition};
use social_client::{FriendshipsServiceClient, Payload};

const RECONNECT_DELAY: u64 = 10; // seconds
const TIMEOUT_RESPONSE: u64 = 20; // seconds

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

#[tokio::main]
async fn main() {
    // Auth Users
    let [user_a, user_b, _] = load_users().await;

    let which_a = format!("USER_A_{}", &user_a.address[user_a.address.len() - 4..]);
    let which_b = format!("USER_B_{}", &user_b.address[user_b.address.len() - 4..]);

    // Hosts
    let host_a = "ws://localhost:8085";
    let host_b = "ws://localhost:8085";

    let handle_a = tokio::spawn(async move {
        loop {
            handle_connection(host_a, &user_a.token, &which_a).await;
        }
    });

    let handle_b = tokio::spawn(async move {
        loop {
            handle_connection(host_b, &user_b.token, &which_b).await;
        }
    });

    let _ = tokio::try_join!(handle_a, handle_b);
}

async fn handle_connection(host: &str, token: &str, which: &str) {
    loop {
        match WebSocketClient::connect(host).await {
            Ok(client_connection) => {
                let client_transport = WebSocketTransport::new(client_connection.clone());

                let mut client = RpcClient::new(client_transport).await.unwrap();

                let port = client.create_port("friendships").await.unwrap();

                let module = port
                    .load_module::<FriendshipsServiceClient<Transport>>("FriendshipsService")
                    .await
                    .unwrap();

                // 4. Listen to updates to my address
                let updates_response = tokio::time::timeout(
                    tokio::time::Duration::from_secs(TIMEOUT_RESPONSE),
                    module.subscribe_friendship_events_updates(Payload {
                        synapse_token: Some(token.to_string()),
                    }),
                )
                .await;
                match updates_response {
                    Ok(Ok(mut u)) => {
                        println!(
                            "> Server Streams > Response > Notifications > {which} > Listening..."
                        );
                        while let Ok(Some(update)) = tokio::time::timeout(
                            tokio::time::Duration::from_secs(TIMEOUT_RESPONSE),
                            u.next(),
                        )
                        .await
                        {
                            println!("> Server Streams > Response > Notifications > {which} > {update:?}");
                        }
                        println!("Timeout when waiting for response, reconnecting...");
                    }
                    _ => {
                        println!("Error with the connection, reconnecting...");
                        break;
                    }
                }
            }
            Err(err) => {
                println!("Failed to connect, retrying in {RECONNECT_DELAY} seconds...");
                println!("Error: {err:?}");
                tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_DELAY)).await;
            }
        }
    }
}
