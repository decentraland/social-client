use std::time::Duration;

use async_trait::async_trait;
use dcl_rpc::client::RpcClient;
use rand::{thread_rng, Rng};

use crate::{
    credentials::{load_users, AuthUser},
    friendship_procedures::{get_friends, get_request_events, Flow},
    FriendshipsServiceClient,
};

use super::{
    args::Args,
    client::TestWebSocketTransport,
    simulation::{Client, Context},
};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum FriendshipEvent {
    REQUEST, // Send a friendship request
    CANCEL,  // Cancel a friendship request
    ACCEPT,  // Accept a friendship request
    REJECT,  // Reject a friendship request
    DELETE,  // Delete an existing friendship
}

pub struct TestContext {
    pub acting_user: AuthUser,
    pub second_user: AuthUser,
    pub timeout: Duration,
}

pub struct TestClient {
    pub client: RpcClient<TestWebSocketTransport>,
    pub service: FriendshipsServiceClient<TestWebSocketTransport>,
    pub last_event: Option<FriendshipEvent>,
}

#[async_trait]
impl Context for TestContext {
    async fn init(args: &Args) -> Self {
        let (auth_user_a, auth_user_b) = load_users().await;

        // Randomize the assigment of the acting user and the second user
        let acting_user = if thread_rng().gen_bool(0.5) {
            auth_user_a.clone()
        } else {
            auth_user_b.clone()
        };

        let second_user = if acting_user == auth_user_a {
            auth_user_b
        } else {
            auth_user_a
        };

        Self {
            acting_user,
            second_user,
            timeout: Duration::from_secs(args.timeout as u64),
        }
    }
}

#[async_trait]
impl Client<TestContext> for TestClient {
    async fn from_rpc_client(mut client: RpcClient<TestWebSocketTransport>) -> Self {
        let port = client
            .create_port("test-port")
            .await
            .expect("Can create port");

        let service = port
            .load_module::<FriendshipsServiceClient<_>>("FriendshipsService")
            .await
            .expect("Can create frienships service");

        Self {
            client,
            service,
            last_event: None,
        }
    }

    async fn act(mut self, context: &TestContext) -> Self {
        let acting_user = context.acting_user.clone();
        let second_user = context.second_user.clone();

        // Randomize the action to be performed by the client
        if thread_rng().gen_bool(0.5) {
            get_friends(&self.service, &acting_user).await;
            get_request_events(&self.service, &acting_user).await;
        } else {
            get_friends(&self.service, &second_user).await;
            get_request_events(&self.service, &second_user).await;
        };

        // Take action based on self.last_event
        match self.last_event {
            Some(FriendshipEvent::REQUEST) => {
                match rand::random::<u8>() % 3 {
                    0 => {
                        // Randomly choose between accepting, cancelling, and rejecting
                        let accept = Flow::Accept;
                        accept
                            .execute_event(&self.service, second_user, acting_user)
                            .await;
                        self.last_event = Some(FriendshipEvent::ACCEPT);
                    }
                    1 => {
                        let cancel = Flow::Cancel;
                        cancel
                            .execute_event(&self.service, acting_user, second_user)
                            .await;
                        self.last_event = Some(FriendshipEvent::CANCEL);
                    }
                    _ => {
                        let reject = Flow::Reject;
                        reject
                            .execute_event(&self.service, second_user, acting_user)
                            .await;
                        self.last_event = Some(FriendshipEvent::REJECT);
                    }
                }
            }
            Some(FriendshipEvent::CANCEL) => {
                let request = Flow::Request;
                request
                    .execute_event(&self.service, acting_user, second_user)
                    .await;

                self.last_event = Some(FriendshipEvent::REQUEST);
            }
            Some(FriendshipEvent::ACCEPT) => {
                let delete = Flow::Delete;
                delete
                    .execute_event(&self.service, acting_user, second_user)
                    .await;

                self.last_event = Some(FriendshipEvent::DELETE);
            }
            Some(FriendshipEvent::REJECT) => {
                let request = Flow::Request;
                request
                    .execute_event(&self.service, acting_user, second_user)
                    .await;

                self.last_event = Some(FriendshipEvent::REQUEST);
            }
            Some(FriendshipEvent::DELETE) => {
                let request = Flow::Request;
                request
                    .execute_event(&self.service, acting_user, second_user)
                    .await;

                self.last_event = Some(FriendshipEvent::REQUEST);
            }
            None => {
                let request = Flow::Request;
                request
                    .execute_event(&self.service, acting_user, second_user)
                    .await;

                self.last_event = Some(FriendshipEvent::REQUEST);
            }
        }

        self
    }
}

impl TestClient {}
