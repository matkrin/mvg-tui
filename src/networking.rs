use anyhow::Result;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{api::{get_routes, get_station, StationResponse}, app::App};

pub enum IoEvent {
    GetRoutes(String, String),
}

pub async fn start_tokio(app: &Arc<Mutex<App>>, mut io_rx: tokio::sync::mpsc::Receiver<IoEvent>) -> Result<()> {
    while let Some(io_event) = io_rx.recv().await {
        match io_event {
            IoEvent::GetRoutes(from, to) => {
                let from = get_station(&from).await?;
                let from_id = if let StationResponse::Station(x) = &from.locations[0] {
                    x.id.clone()
                } else {
                    "".to_string()
                };
                let to = get_station(&to).await?;
                let to_id = if let StationResponse::Station(x) = &to.locations[0] {
                    x.id.clone()
                } else {
                    "".to_string()
                };
                let routes =
                    get_routes(&from_id, &to_id, None, None, None, None, None, None, None, None).await?;

                // Acquire a lock on the App Mutex and mutate the state
                let mut app = app.lock().await;
                app.routes = routes.connection_list;
                app.show_popup = false;
            }
        }
    }
    Ok(())
}
