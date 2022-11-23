use std::io;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use minecraft_bots::{config, App, ui};
use tracing::Level;
use tui::{backend::CrosstermBackend, Terminal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::DEBUG)
    //     .init();

    // Init config and create app state
    let config = config::Config::parse();
    let app = App::new(config);

    // Init terminal ui
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    
    ui::run_app(&mut terminal, app)?;

    // Disable terminal ui
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(())
}
