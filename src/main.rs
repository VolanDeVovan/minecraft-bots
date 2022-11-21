use std::env;

use azalea::{plugins, Account, Client, Event, pathfinder::State};
use azalea_protocol::ServerAddress;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT").unwrap_or("25565".to_string()).parse()?;

    let count: usize = env::var("COUNT").unwrap_or("10".to_string()).parse()?;
    let prefix = env::var("PREFIX").unwrap_or("bot_".to_string());

    let task = tokio::spawn(async move {
        let username = format!("{}_{}", prefix, 1);
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
        .unwrap_or_else(|err| {
            println!("{}: {}", username, err);
        });
    });


    task.await?;

    Ok(())
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            println!("{} logged in", bot.profile.name);

            bot.send_chat_packet("hello world").await?;
        }

        Event::Chat(m) => {
            println!("{}: {}", bot.profile.name, m.message().to_ansi(None));
        }

        _ => {}
    }

    Ok(())
}
