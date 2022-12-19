use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Sender, Receiver};
use std::thread;

use anyhow::Result;
use tui::Terminal;

use crossterm::event::{self, Event, KeyCode};
use tui::backend::Backend;
use tui::widgets::TableState;

use crate::api::routes::Connection;

use crate::networking::{start_tokio, IoEvent};
use crate::ui::ui;

pub enum InputMode {
    Normal,
    Editing,
    Table,
}

pub enum Focus {
    Start,
    Destination,
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
                        KeyCode::Char('h') => app.focus_start(),
                        KeyCode::Char('l') => app.focus_destination(),
                        KeyCode::Char('j') => app.focus_routes(),
                        KeyCode::Char('k') => app.focus_start(),
                        KeyCode::Char('f') => {
                            app.show_popup = true;
                            if let Some(tx) = &app.io_tx {
                                tx.send(IoEvent::GetRoutes(app.start.to_string(), app.destination.to_string())).await;
                            };
                        },
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

fn handle_i_key(app: &mut App) {
    match app.focus {
        Focus::Start => app.input_mode = InputMode::Editing,
        Focus::Destination => app.input_mode = InputMode::Editing,
        Focus::Routes => app.input_mode = InputMode::Table,
    }
}

fn handle_typing(app: &mut App, character: char) {
    match app.focus {
        Focus::Start => app.input_start.push(character),
        Focus::Destination => app.input_destination.push(character),
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
        _ => {}
    }
}

fn handle_esc(app: &mut App) {
    app.input_mode = InputMode::Normal;
    match app.focus {
        Focus::Start => app.start = app.input_start.clone(),
        Focus::Destination => app.destination = app.input_destination.clone(),
        _ => {}
    }
}

