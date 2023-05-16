use std::{
    collections::HashSet,
    sync::Arc,
    thread::sleep,
    time::{Duration, Instant},
};

use async_trait::async_trait;
use clap::command;
use dcl_rpc::{
    client::RpcClient,
    transports::web_sockets::{
        tungstenite::{TungsteniteWebSocket, WebSocketClient},
        WebSocketTransport,
    },
};
use futures_util::{stream::FuturesUnordered, StreamExt};
use log::{debug, info};
use rand::{seq::IteratorRandom, thread_rng};
use tokio::sync::Mutex;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "ws://127.0.0.1:8085", help = "Hostname")]
    pub rpc_host: String,

    #[arg(short, long, default_value_t = 50, help = "Parallel")]
    pub parallel: u8,

    #[arg(
        short,
        long,
        default_value_t = 10000,
        help = "Amount of clients to connect"
    )]
    pub clients: usize,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Simulation duration in minutes"
    )]
    pub duration: u8,

    #[arg(short, long, default_value_t = 60, help = "Request timeout")]
    pub timeout: u8,
}

#[async_trait]
pub trait Context {
    async fn init(args: &Args) -> Self;
}

#[async_trait]
pub trait Client<C: Context> {
    async fn from_rpc_client(client: RpcClient<TestWebSocketTransport>) -> Self;
    async fn act(self, context: &C) -> Self;
}

pub struct Simulation;

impl Simulation {
    pub async fn run<SC, C>(args: &Args, rpc_clients: Vec<RpcClient<TestWebSocketTransport>>)
    where
        SC: Context + Send + Sync + 'static,
        C: Client<SC> + Send + Sync + 'static,
    {
        let context = SC::init(args).await;
        let mut clients = vec![];
        for rpc_client in rpc_clients {
            clients.push(C::from_rpc_client(rpc_client).await);
        }

        let clients = Arc::new(Mutex::new(clients));
        let context = Arc::new(context);

        debug!("Simulation > Wait for 10s before start...");
        sleep(Duration::from_secs(10));
        let mut futures = FuturesUnordered::new();
        for worker_id in 0..args.parallel {
            futures.push(tokio::spawn(worker(
                worker_id,
                args.clone(),
                clients.clone(),
                context.clone(),
            )));
        }

        let mut worker_ids = (0..args.parallel).collect::<HashSet<_>>();

        while let Some(worker_result) = futures.next().await {
            match worker_result {
                Ok(worker_id) => {
                    worker_ids.remove(&worker_id);
                    debug!("Remaining active workers {}", worker_ids.len());
                }
                _ => {
                    debug!("Worker failed to join");
                }
            }
        }
    }
}

async fn worker<SC, C>(
    worker_id: u8,
    args: Args,
    clients: Arc<Mutex<Vec<C>>>,
    context: Arc<SC>,
) -> u8
where
    SC: Context + Send + Sync,
    C: Client<SC> + Send + Sync,
{
    let duration = Duration::from_secs(60 * args.duration as u64);
    let start = Instant::now();
    loop {
        debug!("Worker {worker_id} > Locking clients");
        let mut clients_guard = clients.lock().await;
        let i = (0..clients_guard.len()).choose(&mut thread_rng());
        if let Some(i) = i {
            let client = clients_guard.remove(i);

            if start.elapsed() > duration {
                debug!("Worker {worker_id} > No more time to act, dropping client!");
                info!("Worker {worker_id} > Clients left: {}", clients_guard.len());
                continue;
            }
            drop(clients_guard);
            debug!("Worker {worker_id} > Clients guard manually dropped");
            let client = client.act(&context).await;
            debug!("Worker {worker_id} > client {i} acted");
            clients.lock().await.push(client);
        } else {
            debug!("Worker {worker_id} > No more clients!");
            break;
        }

        let millis = 100;
        debug!("Worker {worker_id} > Waiting {millis} ms before next iteration");
        sleep(Duration::from_millis(millis));
    }
    info!("Worker {worker_id} > Returning");
    worker_id
}

pub type TestWebSocketTransport = WebSocketTransport<TungsteniteWebSocket, ()>;

#[derive(Debug)]
pub enum ClientCreationError {
    Authentication,
    Connection,
    Transport,
}

pub async fn handle_client(
    args: Args,
) -> Result<(RpcClient<TestWebSocketTransport>, u128, u128), ClientCreationError> {
    let Args { rpc_host, .. } = args;
    let whole_connection = Instant::now();
    let ws = WebSocketClient::connect(&rpc_host).await.map_err(|e| {
        log::error!("Couldn't connect to ws: {e:?}");
        ClientCreationError::Connection
    })?;

    let transport = WebSocketTransport::new(ws);

    let client_connection = Instant::now();
    let client = RpcClient::new(transport)
        .await
        .map_err(|_| ClientCreationError::Transport)?;
    let client_creation_elapsed = client_connection.elapsed().as_millis();
    let whole_connection = whole_connection.elapsed().as_millis();

    Ok((client, whole_connection, client_creation_elapsed))
}

use clap::Parser;
#[tokio::main]
async fn main() {
    use env_logger::init as initialize_logger;
    initialize_logger();
    let args = Args::parse();

    let test_elapsed_time = Instant::now();
    let mut set = tokio::task::JoinSet::new();

    let mut whole_conns = vec![];
    let mut client_conns = vec![];
    let mut rpc_clients = vec![];

    for i in 0..args.clients {
        set.spawn(handle_client(args.clone()));
        if (i + 1) % args.parallel as usize == 0 {
            while let Some(res) = set.join_next().await {
                match res.unwrap() {
                    Ok((client, whole_conn, client_conn)) => {
                        rpc_clients.push(client);
                        whole_conns.push(whole_conn);
                        client_conns.push(client_conn);

                        info!("Connected clients: {}", rpc_clients.len());
                    }
                    Err(e) => {
                        debug!("Couldn't create client: {e:?}");
                        info!("Ending test as clients can't connect to server");
                        return;
                    }
                }
            }
            sleep(Duration::from_millis(500));
        }
    }

    let test_elapsed_time = test_elapsed_time.elapsed().as_secs();
    let mean_whole = mean(&whole_conns);
    let mean_client_conns = mean(&client_conns);

    info!("Clients Creation >");
    info!("\nCurrent test duration: {} secs", test_elapsed_time);
    info!("\nEntire Connection (mean) {mean_whole} ms");
    info!("\nClient Connection (mean) {mean_client_conns} ms");

    info!(
        "\nSimulation > Started and will run for {} minutes...",
        args.duration
    );
    Simulation::run::<TestContext, TestClient>(&args, rpc_clients).await;
    info!("\nSimulation > Completed");
}

pub fn mean(values: &[u128]) -> u128 {
    values.iter().sum::<u128>() / values.len() as u128
}

struct TestContext;
struct TestClient {
    _client: RpcClient<TestWebSocketTransport>,
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
