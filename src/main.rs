mod server;
mod client;
mod message;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "broadcast-server")]
#[command(about = "A simple WebSocket broadcast server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Start {
        #[arg(short, long, default_value = "9001")]
        port: u16,
    },

    /// Connect as a client
    Connect {
        #[arg(short, long, default_value = "127.0.0.1:9001")]
        addr: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { port } => server::run_server(port).await,
        Commands::Connect { addr } => client::run_client(addr).await,
    }
}
