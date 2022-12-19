use chrono::{DateTime, Local, NaiveDate, TimeZone, NaiveTime};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;

use anyhow::Result;
use tui::Terminal;

use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::widgets::TableState;

use crate::api::routes::Connection;

use crate::networking::{start_tokio, IoEvent, RoutesParams};
use crate::ui::ui;

pub enum InputMode {
    Normal,
    Editing,
    Table,
}

pub enum Focus {
    Start,
    Destination,
    Date,
    Time,
    Arrival,
    Ubahn,
    Sbahn,
    Tram,
    Bus,
    Routes,
}

pub struct App {
    pub input_mode: InputMode,
    pub focus: Focus,
    pub input_start: String,
    pub input_destination: String,
    pub start: String,
    pub destination: String,
    pub routes: Vec<Connection>,
    pub messages: Vec<String>,
    pub show_popup: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub frames: i64,
    pub datetime: DateTime<Local>,
    pub input_date: String,
    pub input_time: String,
    pub is_arrival: bool,
    pub use_ubahn: bool,
    pub use_sbahn: bool,
    pub use_tram: bool,
    pub use_bus: bool,
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
            show_popup: false,
            io_tx: None,
            frames: 0,
            datetime: Local::now(),
            input_date: Local::now().format("%d.%m.%Y").to_string(),
            input_time: Local::now().format("%H:%M").to_string(),
            is_arrival: false,
            use_ubahn: true,
            use_sbahn: true,
            use_tram: true,
            use_bus: true,
        }
    }
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> Self {
        App {
            io_tx: Some(io_tx),
            ..App::default()
        }
    }

    fn focus_start(&mut self) {
        self.focus = Focus::Start;
    }

    fn focus_destination(&mut self) {
        self.focus = Focus::Destination;
    }

    fn focus_routes(&mut self) {
        self.focus = Focus::Routes;
    }
}

pub struct RoutesTableState {
    pub table_state: TableState,
}

impl RoutesTableState {
    pub fn new() -> Self {
        RoutesTableState {
            table_state: TableState::default(),
        }
    }

    pub fn next_table_entry(&mut self, app: &App) {
        let i = match &app.input_mode {
            InputMode::Table => match self.table_state.selected() {
                Some(i) => {
                    if i >= app.routes.len() - 1 {
                        0
                    } else {
                        i + 1
                    }
                }
                None => 0,
            },
            _ => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn previous_table_entry(&mut self, app: &App) {
        let i = match &app.input_mode {
            InputMode::Table => match self.table_state.selected() {
                Some(i) => {
                    if i == 0 {
                        app.routes.len() - 1
                    } else {
                        i - 1
                    }
                }
                None => 0,
            },
            _ => 0,
        };
        self.table_state.select(Some(i));
    }
}

pub async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
    mut routes_table_state: RoutesTableState,
    rx: Receiver<IoEvent>,
) -> Result<()> {
    let cloned_app = Arc::clone(&app);
    tokio::spawn(async move {
        start_tokio(&app, rx).await;
    });
    loop {
        let mut app = cloned_app.lock().await;
        terminal.draw(|f| ui(f, &mut app, &mut routes_table_state))?;

        if crossterm::event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()), // quits app
                        KeyCode::Char('i') => handle_i_key(&mut app),
                        KeyCode::Char('h') => handle_h_key(&mut app),
                        KeyCode::Char('l') => handle_l_key(&mut app),
                        KeyCode::Char('j') => handle_j_key(&mut app),
                        KeyCode::Char('k') => handle_k_key(&mut app),
                        KeyCode::Char('f') => handle_fetch(&mut app).await,
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        // KeyCode::Enter => {
                        //     app.messages.push(app.input_start.drain(..).collect());
                        // }
                        KeyCode::Char(c) => handle_typing(&mut app, c),
                        KeyCode::Backspace => handle_backspace(&mut app),
                        KeyCode::Esc => handle_esc(&mut app),
                        _ => {}
                    },
                    InputMode::Table => match key.code {
                        KeyCode::Char('j') => routes_table_state.next_table_entry(&app),
                        KeyCode::Char('k') => routes_table_state.previous_table_entry(&app),
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        _ => {}
                    },
                }
            }
            app.frames += 1;
        }
    }
}

async fn handle_fetch(app: &mut App) {
    app.show_popup = true;
    if let Some(tx) = &app.io_tx {
        tx.send(IoEvent::GetRoutes(RoutesParams {
            from: app.start.to_string(),
            to: app.destination.to_string(),
            time: app.datetime.timestamp_millis(),
            arrival: app.is_arrival,
            include_ubahn: app.use_ubahn,
            include_bus: app.use_bus,
            include_tram: app.use_tram,
            include_sbahn: app.use_sbahn,
        }))
        .await;
    };
}

fn handle_i_key(app: &mut App) {
    match app.focus {
        Focus::Start => app.input_mode = InputMode::Editing,
        Focus::Destination => app.input_mode = InputMode::Editing,
        Focus::Routes => app.input_mode = InputMode::Table,
        Focus::Date => app.input_mode = InputMode::Editing,
        Focus::Time => app.input_mode = InputMode::Editing,
        Focus::Arrival => app.is_arrival = !app.is_arrival,
        Focus::Ubahn => app.use_ubahn = !app.use_ubahn,
        Focus::Sbahn => app.use_sbahn = !app.use_sbahn,
        Focus::Tram => app.use_tram = !app.use_tram,
        Focus::Bus => app.use_bus = !app.use_bus,
    }
}

fn handle_h_key(app: &mut App) {
    match app.focus {
        Focus::Start => {}
        Focus::Destination => app.focus = Focus::Start,
        Focus::Date => {}
        Focus::Time => app.focus = Focus::Date,
        Focus::Arrival => app.focus = Focus::Time,
        Focus::Ubahn => app.focus = Focus::Arrival,
        Focus::Sbahn => app.focus = Focus::Ubahn,
        Focus::Tram => app.focus = Focus::Sbahn,
        Focus::Bus => app.focus = Focus::Tram,
        Focus::Routes => {}
    }
}
fn handle_j_key(app: &mut App) {
    match app.focus {
        Focus::Start => app.focus = Focus::Date,
        Focus::Destination => app.focus = Focus::Date,
        Focus::Date => app.focus = Focus::Routes,
        Focus::Time => app.focus = Focus::Routes,
        Focus::Arrival => app.focus = Focus::Routes,
        Focus::Ubahn => app.focus = Focus::Routes,
        Focus::Sbahn => app.focus = Focus::Routes,
        Focus::Tram => app.focus = Focus::Routes,
        Focus::Bus => app.focus = Focus::Routes,
        Focus::Routes => {}
    }
}

fn handle_k_key(app: &mut App) {
    match app.focus {
        Focus::Start => {}
        Focus::Destination => {}
        Focus::Date => app.focus = Focus::Start,
        Focus::Time => app.focus = Focus::Start,
        Focus::Arrival => app.focus = Focus::Destination,
        Focus::Ubahn => app.focus = Focus::Destination,
        Focus::Sbahn => app.focus = Focus::Destination,
        Focus::Tram => app.focus = Focus::Destination,
        Focus::Bus => app.focus = Focus::Destination,
        Focus::Routes => app.focus = Focus::Date,
    }
}

fn handle_l_key(app: &mut App) {
    match app.focus {
        Focus::Start => app.focus = Focus::Destination,
        Focus::Destination => {}
        Focus::Date => app.focus = Focus::Time,
        Focus::Time => app.focus = Focus::Arrival,
        Focus::Arrival => app.focus = Focus::Ubahn,
        Focus::Ubahn => app.focus = Focus::Sbahn,
        Focus::Sbahn => app.focus = Focus::Tram,
        Focus::Tram => app.focus = Focus::Bus,
        Focus::Bus => {}
        Focus::Routes => {}
    }
}

fn handle_typing(app: &mut App, character: char) {
    match app.focus {
        Focus::Start => app.input_start.push(character),
        Focus::Destination => app.input_destination.push(character),
        Focus::Date => app.input_date.push(character),
        Focus::Time => app.input_time.push(character),
        _ => (),
    }
}

fn handle_backspace(app: &mut App) {
    match app.focus {
        Focus::Start => {
            app.input_start.pop();
        }
        Focus::Destination => {
            app.input_destination.pop();
        }
        Focus::Date => {
            app.input_date.pop();
        }
        Focus::Time => {
            app.input_time.pop();
        }
        _ => {}
    }
}

fn handle_esc(app: &mut App) {
    app.input_mode = InputMode::Normal;
    match app.focus {
        Focus::Start => app.start = app.input_start.clone(),
        Focus::Destination => app.destination = app.input_destination.clone(),
        Focus::Date => {
            let date = NaiveDate::parse_from_str(&app.input_date, "%d.%m.%Y").expect("to parse date");
            let datetime = date.and_time(app.datetime.time()); 
            app.datetime = Local.from_local_datetime(&datetime).unwrap();
        }
        Focus::Time => {
            let time = NaiveTime::parse_from_str(&app.input_time, "%H:%M").expect("to parse time");
            let datetime = app.datetime.date_naive().and_time(time);
            app.datetime = Local.from_local_datetime(&datetime).unwrap();
        }
        _ => {}
    }
}
