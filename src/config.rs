use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
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

    /// Tick rate for cli
    /// Affects how often the cli ui will be updated.
    /// Specify in milis
    #[clap(long, value_parser=parse_duration, default_value = "250")]
    pub rate: Duration,

    /// Amount of worker threads in runtime for bots
    /// The default value is the number of cores available to the system.
    #[clap(long, default_value_t = num_cpus::get())]
    pub threads: usize,
}

fn parse_duration(arg: &str) -> Result<std::time::Duration, std::num::ParseIntError> {
    let seconds = arg.parse()?;
    Ok(std::time::Duration::from_millis(seconds))
}
