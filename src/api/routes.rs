use crate::api::station::StationResp;
use anyhow::Result;
use chrono::{Utc, DateTime, Local};
use serde::Deserialize;
use serde_with::TimestampMilliSeconds;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionList {
    pub connection_list: Vec<Connection>,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub zoom_notice_from: bool,
    pub zoom_notice_to: bool,
    pub zoom_notice_from_escalator: bool,
    pub zoom_notice_to_escalator: bool,
    pub zoom_notice_from_elevator: bool,
    pub zoom_notice_to_elevator: bool,
    pub from: StationResp,
    pub to: StationResp,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub departure: DateTime<Local>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub arrival: DateTime<Local>,
    pub connection_part_list: Vec<ConnectionPart>,
    pub efa_ticket_ids: Vec<String>,
    pub server_id: i64,
    pub ring_from: i32,
    pub ring_to: i32,
    pub sap_ticket_mapping_dtos: Option<Vec<SapTicketMappingDtos>>,
    pub old_tarif: bool,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPart {
    pub stops: Option<Vec<Stop>>,
    pub from: StationResp,
    pub to: StationResp,
    pub path: Vec<LocationLongLat>,
    pub path_description: Vec<PathDescription>,
    pub interchange_path: Vec<InterchangePath>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub departure: DateTime<Local>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub arrival: DateTime<Local>,
    pub delay: Option<i32>,
    pub arr_delay: Option<i32>,
    pub cancelled: bool,
    pub product: Option<String>,
    pub label: Option<String>,
    pub network: Option<String>,
    pub connection_part_type: String,
    pub server_id: Option<String>,
    pub destination: Option<String>,
    pub line_direction: Option<String>,
    pub sev: bool,
    pub zoom_notice_departure: bool,
    pub zoom_notice_arrival: bool,
    pub zoom_notice_departure_escalator: bool,
    pub zoom_notice_arrival_escalator: bool,
    pub zoom_notice_departure_elevator: bool,
    pub zoom_notice_arrival_elevator: bool,
    pub departure_platform: Option<String>,
    pub departure_stop_position_number: i32,
    pub arrival_platform: Option<String>,
    pub arrival_stop_position_number: i32,
    pub no_changing_required: bool,
    pub from_id: Option<String>,
    pub departure_id: Option<String>,
    pub info_messages: Option<Vec<String>>,
    pub notifications: Option<Vec<ConnectionPartNotification>>,
    pub occupancy: Option<String>,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub location: StationResp,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub time: DateTime<Local>,
    pub delay: i32,
    pub arr_delay: i32,
    pub cancelled: bool,
}

#[derive(Deserialize, Debug)]
pub struct LocationLongLat {
    // always "location"
    #[serde(rename = "type")]
    pub type_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPartNotification {
    pub title: String,
    pub description: String,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub publication: DateTime<Local>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub valid_from: DateTime<Local>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub valid_to: DateTime<Local>,
    pub id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    // lines: array<any>,
    // event_types: array<any>,
}

#[derive(Deserialize, Debug)]
pub struct PathDescription {
    pub from: i32,
    pub to: i32,
    pub level: i32,
}

#[derive(Deserialize, Debug)]
pub struct InterchangePath {
    // always "location"
    #[serde(rename = "type")]
    pub type_name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SapTicketMappingDtos {
    pub sap_id: String,
    pub sap_name: String,
    pub sap_price: String,
    pub display_title_de: String,
    pub display_title_en: String,
    pub display_subtitle_de: String,
    pub display_subtitle_en: String,
    pub efa_id: String,
    #[serde(rename = "type")]
    pub type_name: String,
    #[serde(rename = "availableATM")]
    pub available_atm: bool,
    #[serde(rename = "availableMobileATM")]
    pub available_mobile_atm: bool,
    #[serde(rename = "availableAPP")]
    pub available_app: bool,
    pub ticket_aggregation_group: String,
    pub tarif_level: String,
    pub zones: String,
}

pub async fn get_routes(
    from_station_id: &str,
    to_station_id: &str,
    time: Option<i64>,
    arrival: Option<bool>,
    sap_ticket: Option<bool>,
    include_ubahn: Option<bool>,
    include_bus: Option<bool>,
    include_tram: Option<bool>,
    include_sbahn: Option<bool>,
    include_taxi: Option<bool>,
) -> Result<ConnectionList> {
    let url = format!(
        "https://www.mvg.de/api/fahrinfo/routing/?fromStation={}
&toStation={}
&time={}
&arrival={}
&sapTickets={}
&transportTypeUnderground={}
&transportTypeBues={}
&transportTram={}
&transportSBahn={}
&transportCallTaxi={}
",
        from_station_id,
        to_station_id,
        time.unwrap_or(Utc::now().timestamp_millis()),
        arrival.unwrap_or(false),
        sap_ticket.unwrap_or(false),
        include_ubahn.unwrap_or(true),
        include_bus.unwrap_or(true),
        include_tram.unwrap_or(true),
        include_sbahn.unwrap_or(true),
        include_taxi.unwrap_or(false),
    );
    let resp = reqwest::get(url).await?.json::<ConnectionList>().await?;
    Ok(resp)
}
