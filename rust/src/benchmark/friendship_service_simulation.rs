use std::time::Duration;

use async_trait::async_trait;
use dcl_rpc::client::RpcClient;

use crate::FriendshipsServiceClient;

use super::{
    args::Args,
    client::TestWebSocketTransport,
    simulation::{Client, Context},
};

pub struct TestContext {
    pub addresses: Vec<String>,
    pub timeout: Duration,
}

pub struct TestClient {
    pub client: RpcClient<TestWebSocketTransport>,
    pub service: FriendshipsServiceClient<TestWebSocketTransport>,
}

#[async_trait]
impl Context for TestContext {
    async fn init(args: &Args) -> Self {
        let mut addresses = vec![];

        // TODO: Populate addresses with addresses.
        // Each client is associated with a different user.
        for _ in 0..args.clients {
            addresses.push("".to_string());
        }

        Self {
            addresses,
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
            .load_module::<FriendshipsServiceClient<_>>("FrienshipService")
            .await
            .expect("Can create quests service");

        Self { client, service }
    }

    async fn act(mut self, _context: &TestContext) -> Self {
        self
    }
}

impl TestClient {}
