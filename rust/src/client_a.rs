use dcl_rpc::transports::web_sockets::tungstenite::WebSocketClient;
use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport},
};
use social_client::friendship_procedures::Flow;
use social_client::{credentials::load_users, FriendshipsServiceClient};

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

const RECONNECT_DELAY: u64 = 10; // seconds

#[tokio::main]
async fn main() {
    // Auth Users
    let [user_a, user_b, _] = load_users().await;

    let host = "ws://127.0.0.1:8085";

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

                println!("Running Client A...");

                println!("Sending request to Client B...");
                let request = Flow::Request;
                request
                    .execute_event(&module, user_a.clone(), user_b.clone())
                    .await;

                println!("Waiting for Client B to accept the request...");
                tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;

                println!("Deleting friendship...");
                let delete = Flow::Delete;
                delete
                    .execute_event(&module, user_a.clone(), user_b.clone())
                    .await;

                println!("Sending request to Client B...");
                let request = Flow::Request;
                request
                    .execute_event(&module, user_a.clone(), user_b.clone())
                    .await;

                println!("Waiting before canceling the request...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                println!("Canceling request to Client B...");
                let cancel = Flow::Cancel;
                cancel
                    .execute_event(&module, user_a.clone(), user_b.clone())
                    .await;

                break;
            }
            _ => {
                println!("Failed to connect, retrying in {RECONNECT_DELAY} seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_DELAY)).await;
            }
        }
    }
}
