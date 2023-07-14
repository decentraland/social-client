use dcl_rpc::transports::web_sockets::tungstenite::WebSocketClient;
use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport},
};
use social_client::{FriendshipsServiceClientDefinition, MutualFriendsPayload, User, Payload};
use social_client::credentials::AuthUser;
use social_client::friendship_procedures::Flow;
use social_client::{credentials::load_users, FriendshipsServiceClient};

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

const RECONNECT_DELAY: u64 = 10; // seconds

#[tokio::main]
async fn main() {
    // Auth Users
    let [user_a, user_b, user_c] = load_users().await;

    let host = "ws://127.0.0.1:8085";

    match WebSocketClient::connect(host).await {
        Ok(client_connection) => {
            let client_transport = WebSocketTransport::new(client_connection.clone());

            let mut client = RpcClient::new(client_transport).await.unwrap();

            let port = client.create_port("friendships").await.unwrap();

            let module = port
                .load_module::<FriendshipsServiceClient<Transport>>("FriendshipsService")
                .await
                .unwrap();

            println!("C -> B: send request");
            let request = Flow::Request;
            request
                .execute_event(&module, user_c.clone(), user_b.clone())
                .await;

            println!("Waiting for Matrix to update the status...");
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

            println!("B -> C: accept request");
            let accept = Flow::Accept;
            accept
                .execute_event(&module, user_c.clone(), user_b.clone())
                .await;

            println!("A -> C: send request");
            let request = Flow::Request;
            request
                .execute_event(&module, user_a.clone(), user_c.clone())
                .await;

            println!("Waiting for Matrix to update the status...");
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
            
            println!("C -> A: accept request");
            let accept = Flow::Accept;
            accept
                .execute_event(&module, user_a.clone(), user_c.clone())
                .await;

            println!("Waiting for Matrix to update the status...");
            tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;

            get_mutual_friends(&module, &user_a, &user_c).await;
        }
        _ => {
            println!("Failed to connect, retrying in {RECONNECT_DELAY} seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_DELAY)).await;
        }
    }
    
}


/// Get and print the friends of the given user using the given module client.
pub async fn get_mutual_friends(module: &FriendshipsServiceClient<Transport>, user: &AuthUser, other_user: &AuthUser) {
    let friends_response = module
        .get_mutual_friends(MutualFriendsPayload {
            user: Some(User{address: other_user.clone().address}),
            auth_token: Some(Payload{synapse_token: Some(user.clone().token)}),
        })
        .await;
    match friends_response {
        Ok(mut friends_response) => {
            println!(
                "> Server Streams > Response > GetMutualFriendsResponse for {:?}",
                &user.clone().address[user.address.len() - 4..]
            );
            while let Some(friend) = friends_response.next().await {
                println!(
                    "> Server Streams > Response > GetMutualFriendsResponse for {:?}: {:?}",
                    &user.clone().address[user.address.len() - 4..],
                    friend.response
                )
            }
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }
}
