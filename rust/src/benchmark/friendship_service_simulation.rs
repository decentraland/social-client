use std::time::Duration;

use async_trait::async_trait;
use dcl_rpc::client::RpcClient;

use crate::{
    credentials::{load_users, AuthUser},
    friendship_procedures::{get_friends, get_request_events},
    FriendshipsServiceClient,
};

use super::{
    args::Args,
    client::TestWebSocketTransport,
    simulation::{Client, Context},
};

pub struct TestContext {
    pub auth_user: AuthUser,
    pub timeout: Duration,
}

pub struct TestClient {
    pub client: RpcClient<TestWebSocketTransport>,
    pub service: FriendshipsServiceClient<TestWebSocketTransport>,
}

#[async_trait]
impl Context for TestContext {
    async fn init(args: &Args) -> Self {
        let (auth_user_a, auth_user_b) = load_users().await;

        let random_user = if rand::random() {
            auth_user_a
        } else {
            auth_user_b
        };

        Self {
            auth_user: random_user,
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

        Self { client, service }
    }

    async fn act(mut self, context: &TestContext) -> Self {
        get_friends(&self.service, &context.auth_user).await;
        get_request_events(&self.service, &context.auth_user).await;

        self
    }
}

impl TestClient {}
