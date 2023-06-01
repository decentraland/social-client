use dcl_rpc::transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport};

use std::time::Duration;
use tokio::time::sleep;

use crate::{
    credentials::AuthUser, friendship_event_payload, AcceptPayload, CancelPayload, DeletePayload,
    FriendshipEventPayload, FriendshipsServiceClient, FriendshipsServiceClientDefinition, Payload,
    RejectPayload, RequestPayload, UpdateFriendshipPayload, User,
};

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

// Define different flows
#[derive(Clone)]
pub enum Flow {
    Flow1,
    Flow2,
    Flow3,
    Flow4,
}

impl Flow {
    pub fn from_str(s: &str) -> Option<Flow> {
        match s {
            "flow1" => Some(Flow::Flow1),
            "flow2" => Some(Flow::Flow2),
            "flow3" => Some(Flow::Flow3),
            "flow4" => Some(Flow::Flow4),
            _ => None,
        }
    }

    pub async fn execute(
        &self,
        module: &FriendshipsServiceClient<Transport>,
        user_a: AuthUser,
        user_b: AuthUser,
    ) {
        match self {
            Flow::Flow1 => {
                // Implement Flow 1: Request A-B, Cancel A-B
                request(module, &user_a.token, &user_b.address).await;
                cancel(module, &user_a.token, &user_b.address).await;
            }
            Flow::Flow2 => {
                // Implement Flow 2: Request A-B, Accept B-A, Delete A-B
                request(module, &user_a.token, &user_b.address).await;
                accept(module, &user_b.token, &user_a.address).await;
                delete(module, &user_a.token, &user_b.address).await;
            }
            Flow::Flow3 => {
                // Implement Flow 3: Request A-B, Reject B-A
                request(module, &user_a.token, &user_b.address).await;
                reject(module, &user_b.token, &user_a.address).await;
            }
            Flow::Flow4 => {
                // Implement Flow 4: Request A-B, Accept B-A, Delete B-A
                request(module, &user_a.token, &user_b.address).await;
                accept(module, &user_b.token, &user_a.address).await;
                delete(module, &user_b.token, &user_a.address).await;
            }
        }
    }
}

async fn request(module: &FriendshipsServiceClient<Transport>, token: &str, user_address: &str) {
    let request_payload = RequestPayload {
        user: Some(User {
            address: user_address.to_string(),
        }),
        message: Some("A message".to_string()),
    };

    update_friendship_event(
        module,
        token,
        friendship_event_payload::Body::Request(request_payload),
    )
    .await;
}

async fn cancel(module: &FriendshipsServiceClient<Transport>, token: &str, user_address: &str) {
    let cancel_payload = CancelPayload {
        user: Some(User {
            address: user_address.to_string(),
        }),
    };

    update_friendship_event(
        module,
        token,
        friendship_event_payload::Body::Cancel(cancel_payload),
    )
    .await;
}

async fn accept(module: &FriendshipsServiceClient<Transport>, token: &str, user_address: &str) {
    let accept_payload = AcceptPayload {
        user: Some(User {
            address: user_address.to_string(),
        }),
    };

    update_friendship_event(
        module,
        token,
        friendship_event_payload::Body::Accept(accept_payload),
    )
    .await;
}

async fn reject(module: &FriendshipsServiceClient<Transport>, token: &str, user_address: &str) {
    let reject_payload = RejectPayload {
        user: Some(User {
            address: user_address.to_string(),
        }),
    };

    update_friendship_event(
        module,
        token,
        friendship_event_payload::Body::Reject(reject_payload),
    )
    .await;
}

async fn delete(module: &FriendshipsServiceClient<Transport>, token: &str, user_address: &str) {
    let delete_payload = DeletePayload {
        user: Some(User {
            address: user_address.to_string(),
        }),
    };

    update_friendship_event(
        module,
        token,
        friendship_event_payload::Body::Delete(delete_payload),
    )
    .await;
}

async fn update_friendship_event(
    module: &FriendshipsServiceClient<Transport>,
    token: &str,
    body: friendship_event_payload::Body,
) {
    let event_payload = FriendshipEventPayload { body: Some(body) };
    let response = module
        .update_friendship_event(UpdateFriendshipPayload {
            event: Some(event_payload),
            auth_token: Some(Payload {
                synapse_token: Some(token.to_string()),
            }),
        })
        .await;
    match response {
        Ok(response) => {
            println!("> Server Unary > Response > UpdateFrienshipResponse:: {response:?}");
        }
        Err(err) => {
            panic!("{err:?}");
        }
    }

    // The state resolution from synapse takes some time
    sleep(Duration::from_secs(5)).await;
}
