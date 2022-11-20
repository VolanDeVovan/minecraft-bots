use azalea::{Account, plugins, pathfinder::State, Client, Event};
use futures::future::join_all;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let mut tasks = Vec::new();

    for i in 0..1000 {
        
        let account = Account::offline(&format!("bot_{i}"));

        let task = azalea::start(azalea::Options {
            account,
            address: "localhost",
            plugins: plugins![],
            state: State::default(),
            handle,
        });

        tasks.push(task);
    }
    
    join_all(tasks).await;


    Ok(())
}


async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {

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