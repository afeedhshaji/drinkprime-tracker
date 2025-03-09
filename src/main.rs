mod fetcher;
mod reporter;
mod config;

use clap::{Parser, Subcommand};
use config::load_config;

#[derive(Parser)]
#[command(name = "drinkprime-tracker")]
#[command(about = "Track and report DrinkPrime water usage")]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, default_value = "config.json")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch water usage data from the device
    Fetch,
    /// Report daily water usage to Discord
    Report,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let config = match load_config(&cli.config) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            return;
        }
    };

    match cli.command {
        Commands::Fetch => {
            fetcher::fetch_and_store(&config).await;
        }
        Commands::Report => {
            reporter::report_usage(&config).await;
        }
    }
}
