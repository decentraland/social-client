use dcl_rpc::transports::web_sockets::{tungstenite::TungsteniteWebSocket, WebSocketTransport};

use std::time::Duration;
use tokio::time::sleep;

use crate::{
    credentials::AuthUser, friendship_event_payload, AcceptPayload, CancelPayload, DeletePayload,
    FriendshipEventPayload, FriendshipsServiceClient, FriendshipsServiceClientDefinition, Payload,
    RejectPayload, RequestPayload, UpdateFriendshipPayload, User,
};

type Transport = WebSocketTransport<TungsteniteWebSocket, ()>;

const DELAY: u64 = 5; // seconds

// Define different flows
#[derive(Clone)]
pub enum Flow {
    /// Request A-B, Cancel A-B
    Flow1,
    /// Request A-B, Accept B-A, Delete A-B
    Flow2,
    /// Request A-B, Reject B-A
    Flow3,
    /// Request A-B, Accept B-A, Delete B-A
    Flow4,
    Request,
    Accept,
    Reject,
    Delete,
    Cancel,
}

impl Flow {
    /// Get the flow from a string.
    pub fn from_str(s: &str) -> Option<Flow> {
        match s {
            "flow1" => Some(Flow::Flow1),
            "flow2" => Some(Flow::Flow2),
            "flow3" => Some(Flow::Flow3),
            "flow4" => Some(Flow::Flow4),
            "request" => Some(Flow::Request),
            "accept" => Some(Flow::Accept),
            "reject" => Some(Flow::Reject),
            "delete" => Some(Flow::Delete),
            "cancel" => Some(Flow::Cancel),
            _ => None,
        }
    }

    /// Execute the flow with the given users and module clients for A and B respectively.
    /// Executing a flow means sending friendship event updates to the server.
    pub async fn execute_flow(
        &self,
        module_a: &FriendshipsServiceClient<Transport>,
        module_b: &FriendshipsServiceClient<Transport>,
        user_a: AuthUser,
        user_b: AuthUser,
    ) {
        match self {
            Flow::Flow1 => {
                // Implement Flow 1: Request A-B, Cancel A-B
                request(module_a, &user_a.token, &user_b.address).await;
                cancel(module_a, &user_a.token, &user_b.address).await;
            }
            Flow::Flow2 => {
                // Implement Flow 2: Request A-B, Accept B-A, Delete A-B
                request(module_a, &user_a.token, &user_b.address).await;
                accept(module_b, &user_b.token, &user_a.address).await;
                delete(module_a, &user_a.token, &user_b.address).await;
            }
            Flow::Flow3 => {
                // Implement Flow 3: Request A-B, Reject B-A
                request(module_a, &user_a.token, &user_b.address).await;
                reject(module_b, &user_b.token, &user_a.address).await;
            }
            Flow::Flow4 => {
                // Implement Flow 4: Request A-B, Accept A-B, Delete B-A
                request(module_a, &user_a.token, &user_b.address).await;
                accept(module_b, &user_b.token, &user_a.address).await;
                delete(module_b, &user_b.token, &user_a.address).await;
            }
            _ => {
                // Do nothing for other Flow variants
            }
        }
    }

    /// Execute a friendship event update with the given users and module client.
    pub async fn execute_event(
        &self,
        module: &FriendshipsServiceClient<Transport>,
        user_a: AuthUser,
        user_b: AuthUser,
    ) {
        match self {
            Flow::Request => {
                // Implement Request A-B
                request(module, &user_a.token, &user_b.address).await;
            }
            Flow::Accept => {
                // Implement Accept B-A
                accept(module, &user_b.token, &user_a.address).await;
            }
            Flow::Reject => {
                // Implement Reject B-A
                reject(module, &user_b.token, &user_a.address).await;
            }
            Flow::Delete => {
                // Implement Delete A-B
                delete(module, &user_a.token, &user_b.address).await;
            }
            Flow::Cancel => {
                // Implement Cancel A-B
                cancel(module, &user_a.token, &user_b.address).await;
            }
            _ => {
                // Do nothing for other Flow variants
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

/// Update the friendship event of the given user using the given module client.
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
    sleep(Duration::from_secs(DELAY)).await;
}

/// Get and print the friends of the given user using the given module client.
pub async fn get_friends(module: &FriendshipsServiceClient<Transport>, user: &AuthUser) {
    let friends_response = module
        .get_friends(Payload {
            synapse_token: Some(user.clone().token),
        })
        .await;
    match friends_response {
        Ok(mut friends_response) => {
            println!(
                "> Server Streams > Response > GetAllFriendsResponse for {:?}",
                &user.clone().address[user.address.len() - 4..]
            );
            while let Some(friend) = friends_response.next().await {
                println!(
                    "> Server Streams > Response > GetAllFriendsResponse for {:?}: {:?}",
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

/// Get and print the friendship request events of the given user using the given module client.
pub async fn get_request_events(module: &FriendshipsServiceClient<Transport>, user: &AuthUser) {
    let friendship_request_events = module
        .get_request_events(Payload {
            synapse_token: Some(user.clone().token),
        })
        .await;
    match friendship_request_events {
        Ok(friendship_request_events) => {
            println!(
                "> Server Unary > Response > GetRequestsResponse for {:?}: {:?}",
                &user.clone().address[user.address.len() - 4..],
                friendship_request_events
            );
        }
        Err(err) => {
            panic!("{err:?}");
        }
    }
}
