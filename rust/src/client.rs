use std::{env, time::Duration};

use dcl_rpc::{
    client::RpcClient,
    transports::web_socket::{WebSocketClient, WebSocketTransport},
};

use tokio::time::sleep;

use social_client::{
    friendship_event_payload, AcceptPayload, CancelPayload, DeletePayload, FriendshipEventPayload,
    FriendshipsServiceClient, FriendshipsServiceClientDefinition, Payload, RejectPayload,
    RequestPayload, UpdateFriendshipPayload, User,
};

// Define different flows
enum Flow {
    Flow1,
    Flow2,
    Flow3,
}

impl Flow {
    fn from_str(s: &str) -> Option<Flow> {
        match s {
            "flow1" => Some(Flow::Flow1),
            "flow2" => Some(Flow::Flow2),
            "flow3" => Some(Flow::Flow3),
            _ => None,
        }
    }

    async fn execute(
        &self,
        module: &FriendshipsServiceClient<WebSocketTransport>,
        token_user_a: &str,
        token_user_b: &str,
        user_a_address: &str,
        user_b_address: &str,
    ) {
        match self {
            Flow::Flow1 => {
                // Implement Flow 1: Request A-B, Cancel A-B
                request_a_to_b(module, token_user_a, user_b_address).await;
                cancel_a_to_b(module, token_user_a, user_b_address).await;
            }
            Flow::Flow2 => {
                // Implement Flow 2: Request A-B, Accept B-A, Delete A-B
                request_a_to_b(module, token_user_a, user_b_address).await;
                accept_b_to_a(module, token_user_b, user_a_address).await;
                delete_a_to_b(module, token_user_a, user_b_address).await;
            }
            Flow::Flow3 => {
                // Implement Flow 3: Request A-B, Reject B-A
                request_a_to_b(module, token_user_a, user_b_address).await;
                reject_b_to_a(module, token_user_b, user_a_address).await;
            }
        }
    }
}

async fn request_a_to_b(
    module: &FriendshipsServiceClient<WebSocketTransport>,
    token_user_a: &str,
    user_b_address: &str,
) {
    let request_payload = RequestPayload {
        user: Some(User {
            address: user_b_address.to_string(),
        }),
        message: Some("A message from userA to userB".to_string()),
    };
    let request_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Request(request_payload)),
    };
    let request_a_to_b_response = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(request_event),
            auth_token: Some(Payload {
                synapse_token: Some(token_user_a.to_string()),
            }),
        })
        .await;
    match request_a_to_b_response {
        Ok(request_a_to_b_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Request::A->B {request_a_to_b_response:?}"
            );
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }

    // The state resolution from synapse takes some time
    sleep(Duration::from_secs(5)).await;
}

async fn cancel_a_to_b(
    module: &FriendshipsServiceClient<WebSocketTransport>,
    token_user_a: &str,
    user_b_address: &str,
) {
    let cancel_payload = CancelPayload {
        user: Some(User {
            address: user_b_address.to_string(),
        }),
    };
    let cancel_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Cancel(cancel_payload)),
    };
    let a = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(cancel_event),
            auth_token: Some(Payload {
                synapse_token: Some(token_user_a.to_string()),
            }),
        })
        .await;
    match a {
        Ok(cancel_a_to_b_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Cancel::A->B {cancel_a_to_b_response:?}"
            );
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }
}

async fn accept_b_to_a(
    module: &FriendshipsServiceClient<WebSocketTransport>,
    token_user_b: &str,
    user_a_address: &str,
) {
    let accept_payload = AcceptPayload {
        user: Some(User {
            address: user_a_address.to_string(),
        }),
    };
    let accept_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Accept(accept_payload)),
    };
    let accept_b_to_a_response = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(accept_event),
            auth_token: Some(Payload {
                synapse_token: Some(token_user_b.to_string()),
            }),
        })
        .await;
    match accept_b_to_a_response {
        Ok(accept_b_to_a_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Accept::B->A {accept_b_to_a_response:?}"
            );
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }

    // The state resolution from synapse takes some time
    sleep(Duration::from_secs(5)).await;
}

async fn reject_b_to_a(
    module: &FriendshipsServiceClient<WebSocketTransport>,
    token_user_b: &str,
    user_a_address: &str,
) {
    let reject_payload = RejectPayload {
        user: Some(User {
            address: user_a_address.to_string(),
        }),
    };
    let reject_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Reject(reject_payload)),
    };
    let reject_b_to_a_response = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(reject_event),
            auth_token: Some(Payload {
                synapse_token: Some(token_user_b.to_string()),
            }),
        })
        .await;
    match reject_b_to_a_response {
        Ok(reject_b_to_a_response) => {
            println!(
              "> Server Unary > Response > UpdateFrienshipResponse Reject::B->A {reject_b_to_a_response:?}"
          );
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }
}

async fn delete_a_to_b(
    module: &FriendshipsServiceClient<WebSocketTransport>,
    token_user_a: &str,
    user_b_address: &str,
) {
    let delete_payload = DeletePayload {
        user: Some(User {
            address: user_b_address.to_string(),
        }),
    };
    let delete_event = FriendshipEventPayload {
        body: Some(friendship_event_payload::Body::Delete(delete_payload)),
    };
    let delete_a_to_b_response = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(delete_event),
            auth_token: Some(Payload {
                synapse_token: Some(token_user_a.to_string()),
            }),
        })
        .await;
    match delete_a_to_b_response {
        Ok(delete_a_to_b_response) => {
            println!(
                "> Server Unary > Response > UpdateFrienshipResponse Delete::A->B {delete_a_to_b_response:?}"
            );
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }
}

#[tokio::main]
async fn main() {
    let token_user_a = "";
    let token_user_b = "";

    let user_a_address = "";
    let user_b_address = "";

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

    // 1. Get Friends message
    let friends_response = module
        .get_friends(Payload {
            synapse_token: Some(token_user_a.to_string()),
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
        }
        Err(err) => {
            panic!("{err:?}")
        }
    }

    // 2. Get Requests Events message
    let friendship_request_events = module
        .get_request_events(Payload {
            synapse_token: Some(token_user_a.to_string()),
        })
        .await;
    match friendship_request_events {
        Ok(friendship_request_events) => {
            println!(
                "> Server Unary > Response > GetRequestsResponse {friendship_request_events:?}"
            );
        }
        Err(err) => {
            panic!("{err:?}");
        }
    }

    // 3. Update Friendship Events message
    // Get the flow to execute from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a flow to execute");
    }
    let flow = Flow::from_str(&args[1]).expect("Invalid flow");

    flow.execute(
        &module,
        token_user_a,
        token_user_b,
        user_a_address,
        user_b_address,
    )
    .await;
}
