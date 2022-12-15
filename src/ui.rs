use std::collections::HashSet;

use chrono::Local;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::app::{App, Focus, InputMode};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    ///// Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let input_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    ///// help message
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("i", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start edition."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to recored message"),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);

    let help_message = Paragraph::new(text);
    f.render_widget(help_message, chunks[0]);

    ///// Input ares
    let input_start = start_paragraph(&app);
    let input_destination = desination_paragraph(&app);

    f.render_widget(input_start, input_areas[0]);
    f.render_widget(input_destination, input_areas[1]);

    ///// cursor position
    match app.input_mode {
        InputMode::Normal => {}
        InputMode::Editing => match app.focus {
            Focus::Start => f.set_cursor(
                input_areas[0].x + app.input_start.width() as u16 + 1,
                input_areas[0].y + 1,
            ),
            Focus::Destination => f.set_cursor(
                input_areas[1].x + app.input_destination.width() as u16 + 1,
                input_areas[1].y + 1,
            ),
            _ => {}
        },
    }

    ///// routes pane
    let routes = routes_table(&app);

    f.render_widget(routes, chunks[2]);
}

fn start_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input_start.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => {
                if let Focus::Start = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }

            InputMode::Editing => {
                if let Focus::Start = app.focus {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Start"))
}

fn desination_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input_destination.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => {
                if let Focus::Destination = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }

            InputMode::Editing => {
                if let Focus::Destination = app.focus {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Destination"))
}

fn routes_table(app: &App) -> Table {
    let header_cells = ["TIME", "IN", "DURATION", "LINES", "DELAY", "INFO"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(Style::default())
        .height(1)
        .bottom_margin(1);

    let items = &app.routes;

    let rows = items.iter().map(|item| {
        let height = 1;
        let time = format!("{} - {}", item.departure.time(), item.arrival.time());
        let in_minutes = (item.departure.time() - Local::now().time())
            .num_minutes()
            .to_string();
        let duration = (item.arrival.time() - item.departure.time())
            .num_minutes()
            .to_string();

        let mut lines = HashSet::new();
        for cp in item.connection_part_list.iter() {
            if cp.connection_part_type == "FOOTWAY" {
                lines.insert("walk");
            } else {
                let label = if let Some(x) = &cp.label { x } else { "" };
                lines.insert(label);
            }
        }
        let lines = lines.into_iter().collect::<Vec<&str>>().join(", ");

        let mut delay = if let Some(x) = item.connection_part_list[0].delay {
            x.to_string()
        } else {
            "-".to_string()
        };
        if delay == "0" {
            delay = "-".to_string();
        }

        let mut info = "info".to_string();
        for cp in item.connection_part_list.iter() {
            let label = if let Some(x) = &cp.label { x } else { "" };
            let nots = if let Some(x) = &cp.notifications {
                x.iter().map(|n| n.title.clone()).collect()
            } else {
                "".to_string()
            };
            if nots == "" {
                info = if let Some(x) = &cp.info_messages {
                    x.join(" ")
                } else {
                    "".to_string()
                };
            } else {
                info = format!("{}: {}", label, nots);
            }
        }

        let cells = vec![time, in_minutes, duration, lines, delay, info];
        Row::new(cells)
            .height(height as u16)
            .bottom_margin(0)
            .style(Style::default())
    });

    Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Routes")
                .border_style(match app.focus {
                    Focus::Routes => Style::default().fg(Color::Blue),
                    _ => Style::default(),
                }),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol("> ")
        .widths(&[
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
        ])
}
