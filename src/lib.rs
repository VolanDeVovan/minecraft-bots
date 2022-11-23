use anyhow::Context;
use azalea::{
    pathfinder::{self, State},
    plugins, Account, Client, Event,
};
use azalea_protocol::ServerAddress;
use futures::{stream::FuturesUnordered, StreamExt};
use tracing::{error, info};

pub async fn run_bots(host: String, port: u16, count: usize, prefix: String) -> anyhow::Result<()> {
    let mut tasks = FuturesUnordered::new();

    for i in 1..count + 1 {
        let host = host.clone();
        let prefix = prefix.clone();

        let task = tokio::spawn(async move {
            let username = format!("{}_{}", prefix, i);
            let account = Account::offline(&username);

            azalea::start(azalea::Options {
                account,
                address: ServerAddress {
                    host: host.clone(),
                    port,
                },
                plugins: plugins![pathfinder::Plugin::default()],
                state: State::default(),
                handle,
            })
            .await
            .with_context(|| format!("{} failed to connect", username.clone()))
            .and(Ok(username))
        });

        tasks.push(task);
    }

    while let Some(task) = tasks.next().await {
        match task? {
            Ok(username) => {
                info!("{}: disconnected", username)
            }
            Err(err) => {
                error!("{:#}", err);
            }
        }
    }

    Ok(())
}

async fn handle(bot: Client, event: Event, _state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            info!("{}: connected", bot.profile.name);

            bot.send_chat_packet("hello world").await?;
        }

        Event::Chat(m) => {
            info!("{}: {}", bot.profile.name, m.message().to_ansi(None));
        }

        _ => {}
    }

    Ok(())
}
