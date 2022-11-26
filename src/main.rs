use std::{
    io,
    sync::{Arc, Mutex}, time::Duration,
};

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use minecraft_bots::{config, run_bots, ui, App};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> anyhow::Result<()> {
    let config = config::Config::parse();

    let app = App::new();
    let app = Arc::new(Mutex::new(app));

    // Run tokio runtimes
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime");

    {
        let config = config.clone();
        let app = app.clone();

        runtime.spawn(async move { run_bots(config, app).await });
    }

    // Init terminal ui
    enable_raw_mode()?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture,
    )?;

    let tick_rate = Duration::from_millis(250);
    ui::run_app(&mut terminal, app, tick_rate)?;

    // Disable terminal ui
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
