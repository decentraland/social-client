use dcl_rpc::transports::web_sockets::tungstenite::WebSocketClient;
use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport},
};
use social_client::friendship_procedures::{get_friends, get_request_events, Flow};
use social_client::{credentials::load_users, FriendshipsServiceClient};
use std::env;

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

const RECONNECT_DELAY: u64 = 10; // seconds

#[tokio::main]
async fn main() {
    // Get the flow to execute from command-line arguments
    let args: Vec<String> = env::args().collect();
    let flow = if args.len() >= 2 {
        Flow::from_str(&args[1])
    } else {
        println!("No flow provided");
        None
    };

    // Auth Users
    let [user_a, user_b, _] = load_users().await;

    let host_a = "ws://127.0.0.1:8085";
    let host_b = "ws://127.0.0.1:8086";
    // let host_a = "wss://rpc-social-service.decentraland.zone";

    loop {
        match (
            WebSocketClient::connect(host_a).await,
            WebSocketClient::connect(host_b).await,
        ) {
            (Ok(client_connection_a), Ok(client_connection_b)) => {
                let client_transport_a = WebSocketTransport::new(client_connection_a.clone());
                let client_transport_b = WebSocketTransport::new(client_connection_b.clone());

                let mut client_a = RpcClient::new(client_transport_a).await.unwrap();
                let mut client_b = RpcClient::new(client_transport_b).await.unwrap();

                let port_a = client_a.create_port("friendships").await.unwrap();
                let port_b = client_b.create_port("friendships").await.unwrap();

                let module_a = port_a
                    .load_module::<FriendshipsServiceClient<Transport>>("FriendshipsService")
                    .await
                    .unwrap();
                let module_b = port_b
                    .load_module::<FriendshipsServiceClient<Transport>>("FriendshipsService")
                    .await
                    .unwrap();

                // 1. Get Friends message
                get_friends(&module_a, &user_a).await;

                // 2. Get Friendship Request Events message
                get_request_events(&module_a, &user_a).await;

                // 3. Update Friendship Events message
                if let Some(flow) = flow.clone() {
                    flow.execute_flow(&module_a, &module_b, user_a.clone(), user_b.clone())
                        .await;
                } else {
                    // Do nothing
                }
            }
            _ => {
                println!("Failed to connect, retrying in {RECONNECT_DELAY} seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_DELAY)).await;
            }
        }
    }
}
