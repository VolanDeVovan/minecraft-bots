use std::sync::{Arc, Mutex};

use crossterm::event::{self, read, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Wrap},
    Frame, Terminal,
};

use crate::{chat, App};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: Arc<Mutex<App>>) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app.lock().unwrap()))?;

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
        // match event::read()? {
        //     Event::FocusGained => todo!(),
        //     Event::FocusLost => todo!(),
        //     Event::Key(_) => todo!(),
        //     Event::Mouse(_) => todo!(),
        //     Event::Paste(_) => todo!(),
        //     Event::Resize(_, _) => todo!(),
        // };
    }

    Ok(())
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
        let text: Vec<Spans> = app
            .bots
            .get(i)
            .unwrap()
            .chat
            .iter()
            .map(|msg| {
                chat::convert_component_to_span(msg)
                // let span = Span::from(msg.to_ansi(None));

                // Spans::from(span)
            })
            .collect();

        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Chat").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[1]);
    }
}

#[cfg(test)]
mod tests {
    use crossterm::style::Colored;

    #[test]
    fn parse_ansi() {}
}
