use crate::api::station::StationResp;
use anyhow::Result;
use serde::Deserialize;
use chrono::Utc;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionList {
    pub connection_list: Vec<Connection>,
}

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
    pub departure: i64, //date,
    pub arrival: i64,   //date,
    pub connection_part_list: Vec<ConnectionPart>,
    pub efa_ticket_ids: Vec<String>,
    pub server_id: i64,
    pub ring_from: i32,
    pub ring_to: i32,
    pub sap_ticket_mapping_dtos: Option<Vec<SapTicketMappingDtos>>,
    pub old_tarif: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPart {
    pub stops: Option<Vec<Stop>>,
    pub from: StationResp,
    pub to: StationResp,
    pub path: Vec<LocationLongLat>,
    pub path_description: Vec<PathDescription>,
    pub interchange_path: Vec<InterchangePath>,
    pub departure: i64, // date
    pub arrival: i64,   // date
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub location: StationResp,
    pub time: i64, //date,
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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPartNotification {
    pub title: String,
    pub description: String,
    pub publication: i32, //date,
    pub valid_from: i32,  //date,
    pub valid_to: i32,    //date,
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
    pub available_ATM: bool,
    pub available_mobile_ATM: bool,
    pub available_APP: bool,
    pub ticket_aggregation_group: String,
    pub tarif_level: String,
    pub zones: String,
}

// export struct getroutesoptions {
//     epoch_time?: date,
//     arrival?: bool,
//     sap_ticket?: bool,
//     include_ubahn?: bool,
//     include_bus?: bool,
//     include_tram?: bool,
//     include_sbahn?: bool,
//     include_taxi?: bool,
// }
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
        // 1670671097508_i64,
        time.unwrap_or(Utc::now().timestamp_millis()),
        arrival.unwrap_or(false),
        sap_ticket.unwrap_or(false),
        include_ubahn.unwrap_or(true),
        include_bus.unwrap_or(true),
        include_tram.unwrap_or(true),
        include_sbahn.unwrap_or(true),
        include_taxi.unwrap_or(false),
    );
    println!("{}", url);
    let resp = reqwest::get(url).await?.json::<ConnectionList>().await?;
    Ok(resp)
}
