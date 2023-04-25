use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::Paragraph,
};

use crate::app::{App, InputMode};

pub fn help_message(app: &App) -> Paragraph {
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled("q: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Exit, "),
                Span::styled("i / Enter: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Insert mode/toggle, "),
                Span::styled("f / Space: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Fetch data, "),
                Span::styled(
                    "hjkl / ← ↑ ↓ →: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("Navigation "),
            ],
            Style::default().fg(Color::Cyan),
        ),
        InputMode::Editing => (
            vec![
                Span::styled(
                    "Esc / Enter: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("Normal Mode "),
            ],
            Style::default().fg(Color::Cyan),
        ),
        InputMode::Table => (
            vec![
                Span::styled(
                    "Esc / Enter: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw("Stop table navigation, "),
                Span::styled("jk / ↑ ↓: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("Navigation "),
            ],
            Style::default().fg(Color::Cyan),
        ),
    };
    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    Paragraph::new(text)
}
