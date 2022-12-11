use std::io;

use anyhow::Result;
use chrono::{Utc, DateTime};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mvg_cli_rs::api::{
    get_departures, get_notifications, get_routes, get_station, StationResponse, routes::{ConnectionList, Connection},
};
use reqwest::get;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, ListItem, List, Table, Cell, Row},
    Frame, Terminal, style::{Modifier, Style, Color}, text::{Span, Spans, Text},
};

use unicode_width::UnicodeWidthStr;

#[tokio::main]
async fn main() -> Result<()> {
    // let s = get_station("uni").await?;
    // let r = &s.locations[0];
    //
    // let i = match r {
    //     StationResponse::Station(x) => x.id.clone(),
    //     _ => String::from("no"),
    // };
    // println!("{}", i);
    //
    // let d = get_departures(&i).await?;
    // println!("Depatures: {:#?}", d.departures[0].departure_time.time());
    // let n = get_notifications().await?;
    // println!("{:#?}", n[0].active_duration);
    //
    // let from = get_station("ostbahnhof").await?;
    // let from_id = if let StationResponse::Station(x) = &from.locations[0] {
    //     x.id.clone()
    // } else {
    //     "".to_string()
    // };
    // println!("{}", from_id);
    //
    // let to = get_station("hauptbahnhof").await?;
    // let to_id = if let StationResponse::Station(x) = &to.locations[0] {
    //     x.id.clone()
    // } else {
    //     "".to_string()
    // };
    // println!("{}", to_id);
    //
    // let routes = get_routes(
    //     &from_id, &to_id, None, None, None, None, None, None, None, None,
    // )
    // .await?;
    //
    // println!("{:#?}", routes.connection_list[0].departure.time());
    // println!("{:#?}", routes.connection_list[0].arrival.time());
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title("Block")
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;

    // thread::sleep(Duration::from_millis(5000));

    let app = App::default();
    let res = run_app(&mut terminal, app).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // if let Err(err) = res {
    //     println!("{:?}", err)
    // }

    Ok(())
}

enum InputMode {
    Normal,
    Editing,
}

enum Focus {
    Start,
    Destination,
    Routes
}

struct App {
    input_mode: InputMode,
    focus: Focus,
    input_start: String,
    input_destination: String,
    start: String,
    destination: String,
    routes: Vec<Connection>,
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_mode: InputMode::Normal,
            focus: Focus::Start,
            input_start: String::new(),
            input_destination: String::new(),
            start: String::new(),
            destination: String::new(),
            routes: Vec::new(),
            messages: Vec::new(),
        }
    }
}

impl App {
    fn focus_start(&mut self) {
        self.focus = Focus::Start;
    }

    fn focus_destination(&mut self) {
        self.focus = Focus::Destination;
    }

    fn focus_routes(&mut self) {
        self.focus = Focus::Routes;
    }

    async fn fetch_routes(&mut self) -> Result<()> {
        let from = get_station(&self.start).await?;
        let from_id = if let StationResponse::Station(x) = &from.locations[0] {
            x.id.clone() } else { "".to_string() };
        let to = get_station(&self.destination).await?;
        let to_id = if let StationResponse::Station(x) = &to.locations[0] {
            x.id.clone() } else { "". to_string() };
        let routes = get_routes(&from_id, &to_id, None, None, None, None, None, None, None, None).await?;
        self.routes = routes.connection_list;
        Ok(())
    }
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()),  // quits app
                    KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('h') => app.focus_start(),
                    KeyCode::Char('l') => app.focus_destination(),
                    KeyCode::Char('j') => app.focus_routes(),
                    KeyCode::Char('k') => app.focus_start(),
                    KeyCode::Char('f') => app.fetch_routes().await?,
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => { app.messages.push(app.input_start.drain(..).collect()); },
                    KeyCode::Char(c) =>  match app.focus {
                        Focus::Start => app.input_start.push(c),
                        Focus::Destination => app.input_destination.push(c), 
                        _ => {}
                    },
                    KeyCode::Backspace => match app.focus {
                        Focus::Start => { app.input_start.pop(); },
                        Focus::Destination => { app.input_destination.pop(); },
                        _ => {}

                    },
                    KeyCode::Esc => { 
                        app.input_mode = InputMode::Normal;
                        match app.focus {
                            Focus::Start => app.start = app.input_start.clone(),
                            Focus::Destination => app.destination = app.input_destination.clone(),
                            _ => {},
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
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
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref()
        )
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
        InputMode::Normal => {},
        InputMode::Editing => match app.focus {
            Focus::Start => f.set_cursor(input_areas[0].x + app.input_start.width() as u16 + 1, input_areas[0].y + 1),
            Focus::Destination => f.set_cursor(input_areas[1].x + app.input_destination.width() as u16 + 1, input_areas[1].y + 1),
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
            InputMode::Normal => if let Focus::Start = app.focus {
                Style::default().fg(Color::Blue) } else { Style::default() }

            InputMode::Editing => if let Focus::Start = app.focus {
                Style::default().fg(Color::Yellow) } else { Style::default() }
        })
        .block(Block::default().borders(Borders::ALL).title("Start"))
}

fn desination_paragraph(app: &App) -> Paragraph {
    Paragraph::new(app.input_destination.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => if let Focus::Destination = app.focus {
                Style::default().fg(Color::Blue) } else { Style::default() }

            InputMode::Editing => if let Focus::Destination = app.focus {
                Style::default().fg(Color::Yellow) } else { Style::default() }
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

    // let items = vec![
    //     vec!["one", "two", "three", "four", "five", "six"],
    //     vec!["one", "two", "three", "four", "five", "six"],
    //     vec!["one", "two", "three", "four", "five", "six"],
    //     vec!["one", "two", "three", "four", "five", "six"],
    //     vec!["one", "two", "three", "four", "five", "six"],
    // ];
    let items = &app.routes;

    let rows = items
        .iter()
        .map(|item| {
            let height = 1;
            // let cells = item.iter().map(|c| Cell::from(*c));
            let time = item.departure.time().to_string();
            let in_minutes = "1".to_string();
            let duration = "2".to_string();
            let lines = "3".to_string();
            let delay = item.connection_part_list[0].delay.unwrap().to_string();
            let info = "info".to_string();
            let cells = vec![time, in_minutes, duration, lines, delay, info];
            Row::new(cells).height(height as u16).bottom_margin(0).style(Style::default())
        });


    Table::new(rows)
        .header(header)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Routes")
            .border_style(match app.focus {
                Focus::Routes => Style::default().fg(Color::Blue),
                _ => Style::default(),

            })
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
