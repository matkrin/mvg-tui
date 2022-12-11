use std::io;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mvg_cli_rs::api::{
    get_departures, get_notifications, get_routes, get_station, StationResponse,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, ListItem, List},
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
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

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
    // Current value of input box
    input_start: String,
    input_destination: String,
    // current input mode
    input_mode: InputMode,
    // history of recoreded messages
    messages: Vec<String>,
    focus: Focus,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_start: String::new(),
            input_destination: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            focus: Focus::Start,
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    //quit app
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                    KeyCode::Char('l') => app.focus = Focus::Destination,
                    KeyCode::Char('h') => app.focus = Focus::Start,
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
                    KeyCode::Esc => { app.input_mode = InputMode::Normal; },
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
    let input_area_start = create_input_start(&app, "Start");
    let input_area_destination = create_input_destination(&app, "Destination");

    f.render_widget(input_area_start, input_areas[0]);
    f.render_widget(input_area_destination, input_areas[1]);

    match app.input_mode {
        InputMode::Normal => {},
        InputMode::Editing => match app.focus {
            Focus::Start => f.set_cursor(input_areas[0].x + app.input_start.width() as u16 + 1, input_areas[0].y + 1),
            Focus::Destination => f.set_cursor(input_areas[1].x + app.input_destination.width() as u16 + 1, input_areas[1].y + 1),
            _ => {}
        },
    }

    let messages: Vec<ListItem> = app
        .messages.
        iter().
        enumerate().
        map(|(i, m)| {
            let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
            ListItem::new(content)
        })
        .collect();

    let messages = List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));

    f.render_widget(messages, chunks[2]);
}

fn create_input_start<'a>(app: &'a App, title: &'a str) -> Paragraph<'a> {
    Paragraph::new(app.input_start.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => if let Focus::Start = app.focus {
                Style::default().fg(Color::Blue) } else { Style::default() }

            InputMode::Editing => if let Focus::Start = app.focus {
                Style::default().fg(Color::Yellow) } else { Style::default() }
        })
        .block(Block::default().borders(Borders::ALL).title(title))
}

fn create_input_destination<'a>(app: &'a App, title: &'a str) -> Paragraph<'a> {
    Paragraph::new(app.input_destination.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => if let Focus::Destination = app.focus {
                Style::default().fg(Color::Blue) } else { Style::default() }

            InputMode::Editing => if let Focus::Destination = app.focus {
                Style::default().fg(Color::Yellow) } else { Style::default() }
         })
        .block(Block::default().borders(Borders::ALL).title(title))
}
