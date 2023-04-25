use std::{io, sync::Arc};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use mvg_tui::app::{run_app, App, RoutesTableState};
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

async fn run_tui() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = tokio::sync::mpsc::channel(100);
    let app = Arc::new(Mutex::new(App::new(tx)));
    let routes_table_state = RoutesTableState::new();
    let res = run_app(&mut terminal, app, routes_table_state, rx).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
        ()
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run_tui().await?;
    Ok(())
}
