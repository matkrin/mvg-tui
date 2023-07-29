use anyhow::Result;
use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_with::TimestampMilliSeconds;

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Departures {
//     pub serving_lines: Vec<ServingLineResp>,
//     pub departures: Vec<DepartureResp>,
// }
//
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct ServingLineResp {
//     pub destination: String,
//     pub sev: bool,
//     pub network: String,
//     pub product: String,
//     pub line_number: String,
//     pub diva_id: String,
// }
//
// #[serde_with::serde_as]
// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct DepartureResp {
//     #[serde_as(as = "TimestampMilliSeconds<i64>")]
//     pub departure_time: DateTime<Local>,
//     pub product: String,
//     pub label: String,
//     pub destination: String,
//     pub live: bool,
//     pub delay: Option<i32>,
//     pub cancelled: bool,
//     pub line_background_color: String,
//     pub departure_id: String,
//     pub sev: bool,
//     pub platform: String,
//     pub stop_position_number: i32,
//     pub info_messages: Vec<String>,
// }

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Departure {
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub planned_departure_time: DateTime<Local>,
    pub realtime: bool,
    pub delay_in_minutes: Option<isize>,
    #[serde_as(as = "TimestampMilliSeconds<i64>")]
    pub realtime_departure_time: DateTime<Local>,
    pub transport_type: String,
    pub label: String,
    pub network: String,
    pub train_type: String,
    pub destination: String,
    pub cancelled: bool,
    pub sev: bool,
    pub platform: Option<usize>,
    pub stop_position_number: Option<usize>,
    pub messages: Vec<String>,
    pub banner_hash: String,
    pub occupancy: String,
    pub stop_point_global_id: String,
}

pub async fn get_departures(station_id: &str) -> Result<Vec<Departure>> {
    // let url = format!("https://www.mvg.de/api/fahrinfo/departure/{}", station_id);

    let url = format!("https://www.mvg.de/api/fib/v2/departure?globalId={}&limit=10&offsetInMinutes=0&transportTypes=UBAHN,TRAM,BUS,SBAHN,SCHIFF", station_id);
    let resp = reqwest::get(url).await?.json::<Vec<Departure>>().await?;
    Ok(resp)
}
