use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone};
use crossterm::event::{self, Event, KeyCode};
use mvg_api::routes::Connection;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tui::{backend::Backend, widgets::TableState, Terminal};

use crate::networking::{start_tokio, IoEvent, RoutesParams};
use crate::ui::ui;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
    Table,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct App {
    pub input_mode: InputMode,
    pub focus: Focus,
    pub input_start: String,
    pub input_destination: String,
    pub start: String,
    pub destination: String,
    pub routes: Vec<Connection>,
    pub messages: Vec<String>,
    pub show_fetch_popup: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub frames: i64,
    pub datetime: DateTime<Local>,
    pub input_date: String,
    pub input_time: String,
    pub wrong_time: bool,
    pub wrong_date: bool,
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
            show_fetch_popup: false,
            io_tx: None,
            frames: 0,
            datetime: Local::now(),
            input_date: Local::now().format("%d.%m.%Y").to_string(),
            input_time: Local::now().format("%H:%M").to_string(),
            wrong_time: false,
            wrong_date: false,
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

    async fn handle_fetch(&mut self) {
        if self.wrong_time || self.wrong_date {
            return;
        }
        self.show_fetch_popup = true;
        if let Some(tx) = &self.io_tx {
            _ = tx
                .send(IoEvent::GetRoutes(RoutesParams {
                    from: self.start.to_string(),
                    to: self.destination.to_string(),
                    time: self.datetime,
                    arrival: self.is_arrival,
                    include_ubahn: self.use_ubahn,
                    include_bus: self.use_bus,
                    include_tram: self.use_tram,
                    include_sbahn: self.use_sbahn,
                }))
                .await;
        };
    }

    fn handle_i_key(&mut self) {
        match self.focus {
            Focus::Start => self.input_mode = InputMode::Editing,
            Focus::Destination => self.input_mode = InputMode::Editing,
            Focus::Routes => self.input_mode = InputMode::Table,
            Focus::Date => self.input_mode = InputMode::Editing,
            Focus::Time => self.input_mode = InputMode::Editing,
            Focus::Arrival => self.is_arrival = !self.is_arrival,
            Focus::Ubahn => self.use_ubahn = !self.use_ubahn,
            Focus::Sbahn => self.use_sbahn = !self.use_sbahn,
            Focus::Tram => self.use_tram = !self.use_tram,
            Focus::Bus => self.use_bus = !self.use_bus,
        }
    }

    fn handle_h_key(&mut self) {
        match self.focus {
            Focus::Start => {}
            Focus::Destination => self.focus = Focus::Start,
            Focus::Date => {}
            Focus::Time => self.focus = Focus::Date,
            Focus::Arrival => self.focus = Focus::Time,
            Focus::Ubahn => self.focus = Focus::Arrival,
            Focus::Sbahn => self.focus = Focus::Ubahn,
            Focus::Tram => self.focus = Focus::Sbahn,
            Focus::Bus => self.focus = Focus::Tram,
            Focus::Routes => {}
        }
    }
    fn handle_j_key(&mut self) {
        match self.focus {
            Focus::Start => self.focus = Focus::Date,
            Focus::Destination => self.focus = Focus::Date,
            Focus::Date => self.focus = Focus::Routes,
            Focus::Time => self.focus = Focus::Routes,
            Focus::Arrival => self.focus = Focus::Routes,
            Focus::Ubahn => self.focus = Focus::Routes,
            Focus::Sbahn => self.focus = Focus::Routes,
            Focus::Tram => self.focus = Focus::Routes,
            Focus::Bus => self.focus = Focus::Routes,
            Focus::Routes => {}
        }
    }

    fn handle_k_key(&mut self) {
        match self.focus {
            Focus::Start => {}
            Focus::Destination => {}
            Focus::Date => self.focus = Focus::Start,
            Focus::Time => self.focus = Focus::Start,
            Focus::Arrival => self.focus = Focus::Destination,
            Focus::Ubahn => self.focus = Focus::Destination,
            Focus::Sbahn => self.focus = Focus::Destination,
            Focus::Tram => self.focus = Focus::Destination,
            Focus::Bus => self.focus = Focus::Destination,
            Focus::Routes => self.focus = Focus::Date,
        }
    }

    fn handle_l_key(&mut self) {
        match self.focus {
            Focus::Start => self.focus = Focus::Destination,
            Focus::Destination => {}
            Focus::Date => self.focus = Focus::Time,
            Focus::Time => self.focus = Focus::Arrival,
            Focus::Arrival => self.focus = Focus::Ubahn,
            Focus::Ubahn => self.focus = Focus::Sbahn,
            Focus::Sbahn => self.focus = Focus::Tram,
            Focus::Tram => self.focus = Focus::Bus,
            Focus::Bus => {}
            Focus::Routes => {}
        }
    }

    fn handle_typing(&mut self, character: char) {
        match self.focus {
            Focus::Start => self.input_start.push(character),
            Focus::Destination => self.input_destination.push(character),
            Focus::Date => self.input_date.push(character),
            Focus::Time => self.input_time.push(character),
            _ => (),
        }
    }

    fn handle_backspace(&mut self) {
        match self.focus {
            Focus::Start => {
                self.input_start.pop();
            }
            Focus::Destination => {
                self.input_destination.pop();
            }
            Focus::Date => {
                self.input_date.pop();
            }
            Focus::Time => {
                self.input_time.pop();
            }
            _ => {}
        }
    }

    fn handle_esc(&mut self) {
        self.input_mode = InputMode::Normal;
        match self.focus {
            Focus::Start => self.start = self.input_start.clone(),
            Focus::Destination => self.destination = self.input_destination.clone(),
            Focus::Date => {
                let date = match NaiveDate::parse_from_str(&self.input_date, "%d.%m.%Y") {
                    Ok(date) => date,
                    Err(_) => {
                        self.wrong_date = true;
                        return;
                    }
                };
                let datetime = date.and_time(self.datetime.time());
                self.datetime = Local.from_local_datetime(&datetime).unwrap();
                self.wrong_date = false;
            }
            Focus::Time => {
                let time = match NaiveTime::parse_from_str(&self.input_time, "%H:%M") {
                    Ok(time) => time,
                    Err(_) => {
                        self.wrong_time = true;
                        return;
                    }
                };
                let datetime = self.datetime.date_naive().and_time(time);
                self.datetime = Local.from_local_datetime(&datetime).unwrap();
                self.wrong_time = false;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Default)]
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
        _ = start_tokio(&app, rx).await;
    });
    loop {
        let mut app = cloned_app.lock().await;
        terminal.draw(|f| ui(f, &mut app, &mut routes_table_state))?;

        if crossterm::event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('i') | KeyCode::Enter => app.handle_i_key(),
                        KeyCode::Char('h') | KeyCode::Left => app.handle_h_key(),
                        KeyCode::Char('l') | KeyCode::Right => app.handle_l_key(),
                        KeyCode::Char('j') | KeyCode::Down => app.handle_j_key(),
                        KeyCode::Char('k') | KeyCode::Up => app.handle_k_key(),
                        KeyCode::Char('f') | KeyCode::Char(' ') => app.handle_fetch().await,
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Char(c) => app.handle_typing(c),
                        KeyCode::Backspace => app.handle_backspace(),
                        KeyCode::Esc | KeyCode::Enter => app.handle_esc(),
                        _ => {}
                    },
                    InputMode::Table => match key.code {
                        KeyCode::Char('j') | KeyCode::Down => {
                            routes_table_state.next_table_entry(&app)
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            routes_table_state.previous_table_entry(&app)
                        }
                        KeyCode::Esc | KeyCode::Enter => app.input_mode = InputMode::Normal,
                        _ => {}
                    },
                }
            }
            app.frames += 1;
        }
    }
}
