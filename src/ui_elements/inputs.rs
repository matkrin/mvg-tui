use tui::{
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Focus, InputMode};

pub fn start_paragraph(app: &App) -> Paragraph {
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

pub fn desination_paragraph(app: &App) -> Paragraph {
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

pub fn date_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input_date.as_ref())
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

pub fn time_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input_time.as_ref())
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

pub fn arrival_paragraph(app: &App) -> Paragraph {
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

pub fn ubahn_paragraph(app: &App) -> Paragraph {
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

pub fn sbahn_paragraph(app: &App) -> Paragraph {
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

pub fn tram_paragraph(app: &App) -> Paragraph {
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

pub fn bus_paragraph(app: &App) -> Paragraph {
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
