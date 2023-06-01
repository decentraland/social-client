use dcl_rpc::transports::web_sockets::tungstenite::WebSocketClient;
use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport},
};
use social_client::friendship_flow::Flow;
use social_client::{
    credentials::{load_users, AuthUser},
    FriendshipsServiceClient, FriendshipsServiceClientDefinition, Payload,
};
use std::env;

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

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
    let (user_a, user_b) = load_users().await;

    // let host = "wss://rpc-social-service.decentraland.org";
    // let host = "wss://rpc-social-service.decentraland.org";
    let host = "ws://127.0.0.1:8085";

    loop {
        handle_connection(host, user_a.clone(), user_b.clone(), flow.clone()).await;
    }
}

async fn handle_connection(host: &str, user_a: AuthUser, user_b: AuthUser, flow: Option<Flow>) {
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

                // 1. Get Friends message
                let friends_response = module
                    .get_friends(Payload {
                        synapse_token: Some(user_a.clone().token),
                    })
                    .await;
                match friends_response {
                    Ok(mut friends_response) => {
                        println!(
                            "> Server Streams > Response > GetAllFriendsResponse for {:?}",
                            user_a.clone().address
                        );
                        while let Some(friend) = friends_response.next().await {
                            println!(
                                "> Server Streams > Response > GetAllFriendsResponse {:?}",
                                friend.response
                            )
                        }
                    }
                    Err(err) => {
                        panic!("{err:?}")
                    }
                }

                // 2. Get Requests Events message
                let friendship_request_events = module
                    .get_request_events(Payload {
                        synapse_token: Some(user_a.clone().token),
                    })
                    .await;
                match friendship_request_events {
                    Ok(friendship_request_events) => {
                        println!(
                        "> Server Unary > Response > GetRequestsResponse for {:?}: {friendship_request_events:?}", user_a.clone().address
                    );
                    }
                    Err(err) => {
                        panic!("{err:?}");
                    }
                }

                // 3. Update Friendship Events message
                if let Some(flow) = flow.clone() {
                    flow.execute(&module, user_a.clone(), user_b.clone()).await;
                } else {
                    // Do nothing
                }
            }
            Err(_) => {
                println!("Failed to connect, retrying in 5 seconds...");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
