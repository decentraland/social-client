use dcl_rpc::{
    client::RpcClient,
    transports::web_socket::{WebSocketClient, WebSocketTransport},
};

use social_client::{
    friendship_event_payload, FriendshipEventPayload, FriendshipsServiceClient,
    FriendshipsServiceClientDefinition, Payload, RequestPayload, CancelPayload, UpdateFriendshipPayload, User,
};

#[tokio::main]
async fn main() {
    let token = "";
    let user_b_address = "";
    // let host = "wss://rpc-social-service.decentraland.zone";
    let host = "ws://127.0.0.1:8085";

    let client_connection = WebSocketClient::connect(host)
        .await
        .unwrap();

    let client_transport = WebSocketTransport::new(client_connection);

    let mut client = RpcClient::new(client_transport).await.unwrap();

    let port = client.create_port("friendships").await.unwrap();

    let module = port
        .load_module::<FriendshipsServiceClient<WebSocketTransport>>("FriendshipsService")
        .await
        .unwrap();

    // 1. Get Friends message
    let friends_response = module
        .get_friends(Payload {
            synapse_token: Some(token.to_string()),
        })
        .await;
    match friends_response {
        Ok(mut friends_response) => {
            while let Some(friend) = friends_response.next().await {
                println!(
                    "> Server Streams > Response > GetAllFriendsResponse {:?}",
                    friend.users
                )
            }
        },
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    // 2. Get Requests Events message
    let friendship_request_events = module
        .get_request_events(Payload {
            synapse_token: Some(token.to_string()),
        })
        .await;
    match friendship_request_events {
        Ok(friendship_request_events) => {
            println!(
                "> Server Unary > Response > GetRequestsResponse {:?}",
                friendship_request_events
            );
        },
        Err(err) => {
            panic!("{:?}", err);
        }
    }


    // 3. Update Friendship Events message

    // 3.a. Request A->B
    let request_payload = RequestPayload {
        user: Some(User {
            address: user_b_address.to_string(),
        }),
        message: Some("A message from userA to userB".to_string()),
    };
    let request_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Request(
            request_payload
        )),
    };
    let request_a_to_b_response = module
        .update_friendship_event(
            UpdateFriendshipPayload {
                event: Some(request_event),
                auth_token: Some(Payload {
                    synapse_token: Some(token.to_string()),
                }),
            }
        )
        .await;
    match request_a_to_b_response {
        Ok(request_a_to_b_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Request::A->B {:?}",
                request_a_to_b_response
            );
        },
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    // 3.b. Cancel A->B
    let cancel_payload = CancelPayload {
        user: Some(User {
            address: user_b_address.to_string(),
        })
    };
    let cancel_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Cancel(
            cancel_payload
        )),
    };
    let a = module
        .update_friendship_event(
            UpdateFriendshipPayload {
                event: Some(cancel_event),
                auth_token: Some(Payload {
                    synapse_token: Some(token.to_string()),
                }),
            }
        ).await;
    match a {
        Ok(cancel_a_to_b_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Cancel::A->B {:?}",
                cancel_a_to_b_response
            );
        },
        Err(err) => { panic!("{:?}",err)}
    }

}
