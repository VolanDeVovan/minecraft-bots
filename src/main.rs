use std::env;

use anyhow::Context;
use azalea::{pathfinder::State, plugins, Account, Client, Event};
use azalea_protocol::ServerAddress;
use futures::{future::try_join_all, stream::FuturesUnordered, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT").unwrap_or("25565".to_string()).parse()?;

    let count: usize = env::var("COUNT").unwrap_or("10".to_string()).parse()?;
    let prefix = env::var("PREFIX").unwrap_or("bot".to_string());

    // let mut tasks = Vec::with_capacity(count);

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
                plugins: plugins![],
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
                println!("{}: disconnected", username)
            },
            Err(err) => {
                println!("{:#}", err);
            }
        }
    }

    Ok(())
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            println!("{}: connected", bot.profile.name);

            bot.send_chat_packet("hello world").await?;
        }

        Event::Chat(m) => {
            println!("{}: {}", bot.profile.name, m.message().to_ansi(None));
        }

        _ => {}
    }

    Ok(())
}
