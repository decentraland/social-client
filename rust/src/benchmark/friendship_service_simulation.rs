use async_trait::async_trait;
use dcl_rpc::client::RpcClient;

use super::{
    args::Args,
    client::TestWebSocketTransport,
    simulation::{Client, Context},
};

pub struct TestContext;
pub struct TestClient {
    pub _client: RpcClient<TestWebSocketTransport>,
}

#[async_trait]
impl Context for TestContext {
    async fn init(_args: &Args) -> Self {
        Self {}
    }
}

#[async_trait]
impl Client<TestContext> for TestClient {
    async fn from_rpc_client(_client: RpcClient<TestWebSocketTransport>) -> Self {
        Self { _client }
    }

    async fn act(mut self, _context: &TestContext) -> Self {
        self
    }
}
