use clap::{command, Parser};

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
        default_value_t = 100,
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
