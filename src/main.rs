use std::env;

use azalea::{plugins, Account, Client, Event};
use azalea_protocol::ServerAddress;

use tokio::sync::mpsc;

#[derive(Debug)]
enum Message {
    Connected(String),
}

#[derive(Clone, Debug)]
struct State {
    tx: mpsc::Sender<Message>,
}

impl State {
    fn new(tx: mpsc::Sender<Message>) -> State {
        State { tx }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port: u16 = env::var("PORT").unwrap_or("25565".to_string()).parse()?;

    let count: usize = env::var("COUNT").unwrap_or("10".to_string()).parse()?;
    let prefix = env::var("PREFIX").unwrap_or("bot_".to_string());

    let (tx, mut rx) = mpsc::channel::<Message>(1);

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
            state: State::new(tx.clone()),
            handle,
        })
        .await
        .unwrap_or_else(|err| {
            println!("{}: {}", username, err);
        });
    });

    while let Some(msg) = rx.recv().await {
        println!("got = {:?}", msg);
    }

    task.await?;

    Ok(())
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => {
            println!("{} logged in", bot.profile.name);

            state.tx.send(Message::Connected(bot.profile.name.to_string())).await?;

            bot.send_chat_packet("hello world").await?;
        }

        Event::Chat(m) => {
            println!("{}: {}", bot.profile.name, m.message().to_ansi(None));
        }

        _ => {}
    }

    Ok(())
}
