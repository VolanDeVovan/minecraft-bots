use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use ansi_to_tui::IntoText;
use crossterm::event::{self, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::{config::Config, App, BotState};

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    config: &Config,
) -> anyhow::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app.lock().unwrap(), config))?;

        let timeout = config
            .rate
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

        if last_tick.elapsed() >= config.rate {
            last_tick = Instant::now();
        }
    }
}

fn draw_header<B: Backend>(f: &mut Frame<B>, _app: &mut App, config: &Config, area: Rect) {
    let text = Spans::from(vec![
        Span::raw("Minecraft bots"),
        Span::raw(" | "),
        Span::raw(format!("{}:{}", config.host, config.port)),
        Span::raw(" | "),
        Span::raw(format!("{} threads", config.threads)),
    ]);

    let header = Paragraph::new(text).block(
        Block::default().style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    );

    f.render_widget(header, area);
}

fn draw_bottom<B: Backend>(f: &mut Frame<B>, _app: &mut App, _config: &Config, area: Rect) {
    let key_style = Style::default().bg(Color::Black);
    let description_style = Style::default().bg(Color::Cyan);

    let text = Spans::from(vec![
        Span::styled("Q", key_style),
        Span::styled("Exit", description_style),
    ]);

    let bottom = Paragraph::new(text).block(
        Block::default().style(
            Style::default()
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    );
    f.render_widget(bottom, area);
}

fn draw_bot_block<B: Backend>(f: &mut Frame<B>, app: &mut App, _config: &Config, area: Rect) {
    let items: Vec<ListItem> = app
        .bots
        .iter()
        .map(|bot| {
            let span = Span::styled(
                bot.username.clone(),
                match bot.state {
                    crate::BotState::Connecting => Style::default().fg(Color::Yellow),
                    crate::BotState::Joined => Style::default().fg(Color::Green),
                    crate::BotState::Leaved => Style::default().fg(Color::DarkGray),
                    crate::BotState::Error(_) => Style::default().fg(Color::Red),
                },
            );

            let spans = Spans::from(span);

            ListItem::new(spans)
        })
        .collect();

    let joined = app
        .bots
        .iter()
        .filter(|bot| matches!(bot.state, BotState::Joined))
        .collect::<Vec<_>>()
        .len();

    let error = app
        .bots
        .iter()
        .filter(|bot| matches!(bot.state, BotState::Error(_)))
        .collect::<Vec<_>>()
        .len();

    let leaved = app
        .bots
        .iter()
        .filter(|bot| matches!(bot.state, BotState::Leaved))
        .collect::<Vec<_>>()
        .len();

    let total = app.bots.len();
    let percent = joined as f32 / total as f32 * 100.0;

    let items = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Spans::from(vec![
                    Span::raw("Bots "),
                    Span::styled(format!("{} ", leaved), Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{} ", error), Style::default().fg(Color::Red)),
                    Span::styled(
                        format!("({}%)", percent as usize),
                        Style::default().fg(Color::Magenta),
                    ),
                ])),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let bot_block = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(1), Constraint::Min(0)].as_ref())
        .split(area);

    let gauge = Gauge::default()
        .percent(percent as u16)
        .gauge_style(Style::default().fg(Color::Green))
        .label(format!("{}/{}", joined, total));

    f.render_widget(gauge, bot_block[0]);
    f.render_stateful_widget(items, bot_block[1], &mut app.state);
}

fn draw_chat<B: Backend>(f: &mut Frame<B>, app: &mut App, _config: &Config, area: Rect) {
    let mut text: Text = Text::default();

    if let Some(i) = app.state.selected() {
        app.bots.get(i).unwrap().chat.iter().for_each(|msg| {
            let msg = msg.to_ansi(None);
            match msg.into_text() {
                Ok(msg) => text.extend(msg),
                Err(_) => text.extend(Text::raw(msg)),
            };
        });
    }

    let paragraph = Paragraph::new(text)
        .block(Block::default().title("Chat").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, config: &Config) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Min(0),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    draw_header(f, app, config, layout[0]);
    draw_bottom(f, app, config, layout[2]);

    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(layout[1]);

    draw_bot_block(f, app, config, chunks[0]);
    draw_chat(f, app, config, chunks[1])
}
