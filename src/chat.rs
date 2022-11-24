use azalea_chat::{style::TextColor, Component};
use tui::{
    style::{Color, Style, Modifier},
    text::{Span, Spans},
};

pub fn convert_component_to_span(component: &Component) -> Spans<'static> {
    let spans: Vec<Span> = component
        .clone()
        .into_iter()
        .map(|component| {
            let mut component_text = match &component {
                Component::Text(c) => c.text.to_string(),
                Component::Translatable(c) => c.to_string(),
            };

            let component_style = &component.get_base().style;

            let mut style = Style::default();

            // component_text.push_str(&format!("{:?}", component_style));

            if let Some(color) = &component_style.color {
                let value = color.value;

                // Исправить. За это надо руки отрывать
                if value > 100000 {
                    style = style.fg(Color::Rgb(
                        ((value >> 16) & 0xFF).try_into().unwrap(),
                        ((value >> 8) & 0xFF).try_into().unwrap(),
                        (value & 0xFF).try_into().unwrap(),
                    ));
                }
            }

            Span::styled(component_text, style)
        })
        .collect();

    Spans::from(spans)
}
