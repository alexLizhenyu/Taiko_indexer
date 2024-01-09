use clap::{Parser, Subcommand};

/// Simple programvscode-file://vscode-app/Applications/Visual%20Studio%20Code.app/Contents/Resources/app/out/vs/code/electron-sandbox/workbench/workbench.html to greet a person
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Synchronize data from endpoint and save it to a database.
    Sync {
        /// Substreams gRPC endpoint url
        #[arg(short, long, default_value_t = String::from("http://localhost:18015"))]
        endpoint_url: String,

        /// Start block to stream from
        #[arg(short, long)]
        start_block: u64,

        /// Start block to stream from
        #[arg(short = 't', long, default_value_t = u64::MAX)]
        end_block: u64,
    },
    /// Start query service.
    Serve {
        /// Port to listen on
        #[arg(short = 'P', long, default_value_t = 8080)]
        port: u16,

        /// Host ip to listen on
        #[arg(short = 'H', long, default_value_t = String::from("127.0.0.1"))]
        host: String,
    },

    /// Start both `sync` and `serve` services simultaneously
    All {
        /// Substreams gRPC endpoint url
        #[arg(short, long, default_value_t = String::from("http://localhost:18015"))]
        endpoint_url: String,

        /// Start block to stream from
        #[arg(short, long)]
        start_block: u64,

        /// Start block to stream from
        #[arg(short = 't', long, default_value_t = u64::MAX)]
        end_block: u64,

        /// Port to listen on
        #[arg(short = 'P', long, default_value_t = 8080)]
        port: u16,

        /// Host ip to listen on
        #[arg(short = 'H', long, default_value_t = String::from("127.0.0.1"))]
        host: String,
    },
}
