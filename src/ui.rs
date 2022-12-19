use std::collections::HashSet;

use chrono::Local;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use crate::{
    api::routes::{Connection, ConnectionPart},
    app::{App, Focus, InputMode, RoutesTableState},
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, routes_table_state: &mut RoutesTableState) {
    ///// Layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let input_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let options_areas = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(8),
            Constraint::Percentage(8),
        ])
        .split(chunks[1]);

    ///// Input ares
    let input_start = start_paragraph(app);
    let input_destination = desination_paragraph(app);

    f.render_widget(input_start, input_areas[0]);
    f.render_widget(input_destination, input_areas[1]);

    // Option areas
    let date_panel = date_paragraph(app);
    f.render_widget(date_panel, options_areas[0]);

    let time_panel = time_paragraph(app);
    f.render_widget(time_panel, options_areas[1]);

    let arrival_panel = arrival_paragraph(app);
    f.render_widget(arrival_panel, options_areas[2]);

    let ubahn_panel = ubahn_paragraph(app);
    f.render_widget(ubahn_panel, options_areas[3]);

    let sbahn_panel = sbahn_paragraph(app);
    f.render_widget(sbahn_panel, options_areas[4]);

    let tram_panel = tram_paragraph(app);
    f.render_widget(tram_panel, options_areas[5]);

    let bus_panel = bus_paragraph(app);
    f.render_widget(bus_panel, options_areas[6]);
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
        InputMode::Table => {}
    }

    ///// routes pane
    let routes = routes_table(app);
    f.render_stateful_widget(routes, chunks[2], &mut routes_table_state.table_state);

    ///// help message
    let help_message = help_message(app);
    // let help_message = Paragraph::new(Text::from(app.frames.to_string()));
    f.render_widget(help_message, chunks[3]);

    ///// Popup
    if app.show_popup {
        // let block = Block::default().borders(Borders::ALL);
        let popup_area = popup_rect(10, 5, f.size());
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Fetching")
            .title_alignment(Alignment::Center)
            .style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Blue)
                    .fg(Color::Black),
            );
        f.render_widget(Clear, popup_area);
        f.render_widget(block, popup_area);
    }
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
            InputMode::Table => {
                if let Focus::Start = app.focus {
                    Style::default().fg(Color::Blue)
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
            InputMode::Table => {
                if let Focus::Destination = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Destination"))
}

fn date_paragraph(app: &App) -> Paragraph {
    let date = app.datetime.format("%d.%m.%Y").to_string();
    Paragraph::new(date)
        .style(match app.input_mode {
            InputMode::Normal => {
                if let Focus::Date = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }

            InputMode::Editing => {
                if let Focus::Date = app.focus {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
            InputMode::Table => {
                if let Focus::Date = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Date"))
}

fn time_paragraph(app: &App) -> Paragraph {
    let time = app.datetime.format("%H:%M").to_string();
    Paragraph::new(time)
        .style(match app.input_mode {
            InputMode::Normal => {
                if let Focus::Time = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }

            InputMode::Editing => {
                if let Focus::Time = app.focus {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                }
            }
            InputMode::Table => {
                if let Focus::Time = app.focus {
                    Style::default().fg(Color::Blue)
                } else {
                    Style::default()
                }
            }
        })
        .block(Block::default().borders(Borders::ALL).title("Time"))
}

fn arrival_paragraph(app: &App) -> Paragraph {
    let text = match app.is_arrival {
        true => "Arrival",
        false => "Departure",
    };
    Paragraph::new(text)
        .style(match app.focus {
            Focus::Arrival => Style::default().fg(Color::Blue),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Dep <-> Arr"))
}

fn ubahn_paragraph(app: &App) -> Paragraph {
    let (text, fg) = match app.use_ubahn {
        true => ("True", Color::Green),
        false => ("False", Color::Red),
    };

    let style = match app.focus {
        Focus::Ubahn => Style::default().fg(Color::Blue),
        _ => Style::default().fg(fg),
    };

    Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("U-Bahn"))
}

fn sbahn_paragraph(app: &App) -> Paragraph {
    let (text, fg) = match app.use_sbahn {
        true => ("True", Color::Green),
        false => ("False", Color::Red),
    };

    let style = match app.focus {
        Focus::Sbahn => Style::default().fg(Color::Blue),
        _ => Style::default().fg(fg),
    };

    Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("S-Bahn"))
}

fn tram_paragraph(app: &App) -> Paragraph {
    let (text, fg) = match app.use_tram {
        true => ("True", Color::Green),
        false => ("False", Color::Red),
    };

    let style = match app.focus {
        Focus::Tram => Style::default().fg(Color::Blue),
        _ => Style::default().fg(fg),
    };

    Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("Tram"))
}

fn bus_paragraph(app: &App) -> Paragraph {
    let (text, fg) = match app.use_bus {
        true => ("True", Color::Green),
        false => ("False", Color::Red),
    };

    let style = match app.focus {
        Focus::Bus => Style::default().fg(Color::Blue),
        _ => Style::default().fg(fg),
    };

    Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title("Bus"))
}

fn popup_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
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

    let rows = items.iter().map(prepare_routes);

    Table::new(rows)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Routes")
                .border_style(match app.focus {
                    Focus::Routes => match app.input_mode {
                        InputMode::Table => Style::default().fg(Color::Yellow),
                        _ => Style::default().fg(Color::Blue),
                    },
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

fn prepare_routes(conn: &Connection) -> Row {
    let height = 1;
    let time = format!(
        "{} - {}",
        conn.departure.format("%H:%M"),
        conn.arrival.format("%H:%M")
    );
    let in_minutes = (conn.departure.time() - Local::now().time())
        .num_minutes()
        .to_string();
    let duration = (conn.arrival.time() - conn.departure.time())
        .num_minutes()
        .to_string();
    let lines = prepare_lines(&conn.connection_part_list);
    let delay = prepare_delay(&conn.connection_part_list);
    let info = prepare_info(&conn.connection_part_list);
    let cells = vec![time, in_minutes, duration, lines, delay, info];
    Row::new(cells)
        .height(height as u16)
        .bottom_margin(0)
        .style(Style::default())
}

fn prepare_lines(cp_list: &Vec<ConnectionPart>) -> String {
    let mut lines = HashSet::new();
    for cp in cp_list.iter() {
        if cp.connection_part_type == "FOOTWAY" {
            lines.insert("walk");
        } else {
            let label = if let Some(x) = &cp.label { x } else { "" };
            lines.insert(label);
        }
    }
    lines.into_iter().collect::<Vec<&str>>().join(", ")
}

fn prepare_delay(cp_list: &Vec<ConnectionPart>) -> String {
    let mut delay = if let Some(x) = cp_list[0].delay {
        x.to_string()
    } else {
        "-".to_string()
    };
    if delay == "0" {
        delay = "-".to_string();
    }
    delay
}

fn prepare_info(cp_list: &Vec<ConnectionPart>) -> String {
    let mut info = "".to_string();
    for cp in cp_list.iter() {
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
    info
}

fn help_message(app: &App) -> Paragraph {
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
        InputMode::Table => (
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
    Paragraph::new(text)
}
