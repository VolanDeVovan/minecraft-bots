use std::env;
use minecraft_bots::run_bots;
use tracing::Level;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::DEBUG).init();

    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT").unwrap_or("25565".to_string()).parse()?;

    let count: usize = env::var("COUNT").unwrap_or("10".to_string()).parse()?;
    let prefix = env::var("PREFIX").unwrap_or("bot".to_string());


    run_bots(host, port, count, prefix).await
}
