use std::io;

use anyhow::Result;
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use tui::widgets::TableState;

use crate::api::{get_routes, get_station, routes::Connection, StationResponse};

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
            x.id.clone()
        } else {
            "".to_string()
        };
        let to = get_station(&self.destination).await?;
        let to_id = if let StationResponse::Station(x) = &to.locations[0] {
            x.id.clone()
        } else {
            "".to_string()
        };
        let routes = get_routes(
            &from_id, &to_id, None, None, None, None, None, None, None, None,
        )
        .await?;
        self.routes = routes.connection_list;
        Ok(())
    }
}

pub struct RoutesTableState {
    pub table_state: TableState,
}

impl RoutesTableState {
    fn new() -> Self {
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
    mut app: App,
    mut routes_table_state: RoutesTableState,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app, &mut routes_table_state))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(()), // quits app
                    KeyCode::Char('i') => handle_i_key(&mut app),
                    KeyCode::Char('h') => app.focus_start(),
                    KeyCode::Char('l') => app.focus_destination(),
                    KeyCode::Char('j') => app.focus_routes(),
                    KeyCode::Char('k') => app.focus_start(),
                    KeyCode::Char('f') => app.fetch_routes().await?,
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

pub async fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let routes_table_state = RoutesTableState::new();
    let res = run_app(&mut terminal, app, routes_table_state).await?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // if let Err(err) = res {
    //     println!("{:?}", err);
    //     ()
    // }

    Ok(())
}
