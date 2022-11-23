use clap::Parser;

#[derive(Parser, Debug)]
pub struct Config {
    /// Server address where bots will connect
    #[clap(long, default_value = "localhost")]
    pub host: String,

    /// Server port where bots will connect
    #[clap(long, default_value = "25565")]
    pub port: u16,

    /// Count of bots which will connected to server
    #[clap(long, default_value = "10")]
    pub count: usize,

    /// Prefix for username
    /// Username based on prefix and id
    /// Example: bot_1
    #[clap(long, default_value = "bot")]
    pub prefix: String,
}
