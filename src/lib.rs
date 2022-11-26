use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use azalea::{plugins, Account, Client, Event};
use azalea_chat::text_component::TextComponent;
use azalea_protocol::ServerAddress;
use config::Config;
use tokio::sync::mpsc;
use tui::widgets::ListState;

pub mod config;
pub mod ui;

pub struct App {
    pub bots: Vec<Bot>,

    pub state: ListState,
}

impl App {
    pub fn new() -> Self {
        Self {
            bots: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.bots.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.bots.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

type Username = String;

#[derive(Debug)]
pub struct Bot {
    username: Username,

    state: BotState,

    chat: Vec<azalea_chat::Component>,
}

impl Bot {
    fn new(username: Username) -> Self {
        Self {
            username,
            state: BotState::Connecting,
            chat: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum BotState {
    Connecting,
    Joined,
    Leaved,
    Error(anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct State {
    tx: mpsc::Sender<Message>,
}

impl State {
    fn new(tx: mpsc::Sender<Message>) -> Self {
        Self { tx }
    }
}

#[derive(Debug)]
pub enum Message {
    Joined(Username),
    Leaved(Username),

    Message(Username, azalea_chat::Component),

    Error(Username, anyhow::Error),
}

pub async fn run_bots(config: Config, app: Arc<Mutex<App>>) -> anyhow::Result<()> {
    let config::Config {
        host,
        port,
        count,
        prefix, .. } = config;

    let (tx, mut rx) = mpsc::channel::<Message>(16);

    for i in 1..count + 1 {
        let host = host.clone();
        let prefix = prefix.clone();

        let tx = tx.clone();

        let username = format!("{}_{}", prefix, i);

        let bot = Bot::new(username.clone());
        app.lock().unwrap().bots.push(bot);

        tokio::spawn(async move {
            let account = Account::offline(&username);

            let state = State::new(tx);

            let tx = state.tx.clone();

            let bot = azalea::start(azalea::Options {
                account,
                address: ServerAddress {
                    host: host.clone(),
                    port,
                },
                plugins: plugins![],
                state,
                handle,
            })
            .await;

            match bot {
                Ok(()) => {
                    tx.send(Message::Leaved(username)).await.unwrap();
                }

                Err(err) => {
                    tx.send(Message::Error(username, err.into())).await.unwrap();
                }
            }
        });
    }

    while let Some(message) = rx.recv().await {
        let mut app = app.lock().unwrap();

        match message {
            Message::Joined(username) => {
                app.bots
                    .iter_mut()
                    .find(|bot| bot.username == username)
                    .unwrap()
                    .state = BotState::Joined;
            }
            Message::Leaved(username) => {
                app.bots
                    .iter_mut()
                    .find(|bot| bot.username == username)
                    .unwrap()
                    .state = BotState::Leaved;
            }

            Message::Message(username, msg) => {
                app.bots
                    .iter_mut()
                    .find(|bot| bot.username == username)
                    .unwrap()
                    .chat
                    .push(msg);
            }

            Message::Error(username, err) => {
                let text = err.to_string();

                app.bots
                    .iter_mut()
                    .find(|bot| bot.username == username)
                    .unwrap()
                    .state = BotState::Error(err);

                app.bots
                    .iter_mut()
                    .find(|bot| bot.username == username)
                    .unwrap()
                    .chat
                    .push(azalea_chat::Component::Text(TextComponent::new(format!(
                        "§c{}",
                        text
                    ))));
            }
        }
    }

    Ok(())
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    match event {
        Event::Login => state.tx.send(Message::Joined(bot.profile.name)).await?,

        Event::Chat(m) => {
            state
                .tx
                .send(Message::Message(bot.profile.name, m.message()))
                .await?
        }

        _ => {}
    }

    Ok(())
}
