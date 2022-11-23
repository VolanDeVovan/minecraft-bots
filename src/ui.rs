use crossterm::event::{self, read, Event, KeyCode};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Span, Spans, Text},
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::App;

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),

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

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // Create two chunks with equal horizontal screen space
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());



    // let msg = vec![
    //     Span::raw("Press "),
    //     Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
    //     Span::raw(" to exit, "),
    //     Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
    //     Span::raw(" to start editing."),
    // ];

    // let style = Style::default().add_modifier(Modifier::RAPID_BLINK);

    // let mut text = Text::from(Spans::from(msg));
    // text.patch_style(style);

    // let help_message = Paragraph::new(text.clone());

    // f.render_widget(help_message, chunks[0]);


}
