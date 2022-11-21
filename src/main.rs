use std::env;

use azalea::{pathfinder::State, plugins, Account, Client, Event};
use azalea_protocol::ServerAddress;
use futures::future::join_all;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT").unwrap_or("25565".to_string()).parse()?;

    let count: usize = env::var("COUNT").unwrap_or("10".to_string()).parse()?;
    let prefix = env::var("PREFIX").unwrap_or("bot_".to_string());

    println!(
        "host: {}\nport: {}\ncount: {}\nprefix: {}\n",
        host, port, count, prefix
    );

    let mut tasks = Vec::with_capacity(count);

    for i in 0..count {
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
            .unwrap_or_else(|err| {
                println!("{}: {}", username, err);
            });
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    Ok(())
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            println!("{} logged in", bot.profile.name);

            bot.send_command_packet("register").await?;
        }

        Event::Chat(m) => {
            println!("{}: {}", bot.profile.name, m.message().to_ansi(None));
        }

        _ => {}
    }

    // match event {
    //     Event::Initialize => todo!(),
    //     Event::Login => todo!(),
    //     Event::Chat(_) => todo!(),
    //     Event::Tick => todo!(),
    //     Event::Packet(_) => todo!(),
    //     Event::UpdatePlayers(_) => todo!(),
    // }

    Ok(())
}
