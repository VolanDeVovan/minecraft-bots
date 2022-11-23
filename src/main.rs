use clap::Parser;
use config::Config;
use minecraft_bots::run_bots;
use tracing::{info, Level};

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let Config {
        host,
        port,
        count,
        prefix,
    } = Config::parse();

    info!(
        "host: {}, port: {}, count: {}, prefix: {}",
        host, port, count, prefix
    );

    run_bots(host, port, count, prefix).await
}
