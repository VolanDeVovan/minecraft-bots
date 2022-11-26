use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use ansi_to_tui::IntoText;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::App;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    tick_rate: Duration,
) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app.lock().unwrap()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let app = &mut app.lock().unwrap();
                match key.code {
                    KeyCode::Char('q') => return Ok(()),

                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Left => app.unselect(),

                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .bots
        .iter()
        .map(|bot| {
            let span = Span::styled(
                bot.username.clone(),
                match bot.state {
                    crate::BotState::Connecting => Style::default().fg(Color::White),
                    crate::BotState::Joined => Style::default().fg(Color::Green),
                    crate::BotState::Leaved => Style::default().fg(Color::Gray),
                    crate::BotState::Error(_) => Style::default().fg(Color::Red),
                },
            );

            let spans = Spans::from(span);

            ListItem::new(spans)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Bots"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_stateful_widget(items, chunks[0], &mut app.state);

    if let Some(i) = app.state.selected() {
        let mut text: Text = Text::default();

        app.bots.get(i).unwrap().chat.iter().for_each(|msg| {
            let msg = msg.to_ansi(None);
            match msg.into_text() {
                Ok(msg) => text.extend(msg),
                Err(_) => text.extend(Text::raw(msg)),
            };
        });

        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Chat").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }
}
