use anyhow::Result;
use chrono::{DateTime, Local};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::app::App;
use mvg_api::{get_routes, get_station, Location};

pub struct RoutesParams {
    pub from: String,
    pub to: String,
    pub time: DateTime<Local>,
    pub arrival: bool,
    pub include_ubahn: bool,
    pub include_bus: bool,
    pub include_tram: bool,
    pub include_sbahn: bool,
}

pub enum IoEvent {
    GetRoutes(RoutesParams),
}

pub async fn start_tokio(
    app: &Arc<Mutex<App>>,
    mut io_rx: tokio::sync::mpsc::Receiver<IoEvent>,
) -> Result<()> {
    while let Some(io_event) = io_rx.recv().await {
        match io_event {
            IoEvent::GetRoutes(rp) => {
                let from = get_station(&rp.from).await?;
                let from_id = if let Location::Station(x) = &from[0] {
                    x.global_id.clone()
                } else {
                    "".to_string()
                };
                let to = get_station(&rp.to).await?;
                let to_id = if let Location::Station(x) = &to[0] {
                    x.global_id.clone()
                } else {
                    "".to_string()
                };
                let routes = get_routes(
                    &from_id,
                    &to_id,
                    Some(rp.time),
                    Some(rp.arrival),
                    Some(rp.include_ubahn),
                    Some(rp.include_bus),
                    Some(rp.include_tram),
                    Some(rp.include_sbahn),
                    None,
                )
                .await?;

                // Acquire a lock on the App Mutex and mutate the state
                let mut app = app.lock().await;
                app.routes = routes;
                app.show_fetch_popup = false;
            }
        }
    }
    Ok(())
}
