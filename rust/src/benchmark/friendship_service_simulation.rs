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
    pub auth_users: Vec<AuthUser>,
    pub timeout: Duration,
}

pub struct TestClient {
    pub client: RpcClient<TestWebSocketTransport>,
    pub service: FriendshipsServiceClient<TestWebSocketTransport>,
}

#[async_trait]
impl Context for TestContext {
    async fn init(args: &Args) -> Self {
        let mut auth_users = vec![];
        let (auth_user_a, auth_user_b) = load_users().await;

        // TODO: Populate addresses with addresses.
        // Each client is associated with a different user.
        for _ in 0..args.clients {
            auth_users.push(auth_user_a.clone());
            auth_users.push(auth_user_b.clone());
        }

        Self {
            auth_users,
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
        let auth_user = context.auth_users.clone().pop().expect("Can pop auth user");

        get_friends(&self.service, &auth_user).await;
        get_request_events(&self.service, &auth_user).await;

        self
    }
}

impl TestClient {}
