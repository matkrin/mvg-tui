use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Departures {
    pub serving_lines: Vec<ServingLineResp>,
    pub departures: Vec<DepartureResp>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServingLineResp {
    pub destination: String,
    pub sev: bool,
    pub network: String,
    pub product: String,
    pub line_number: String,
    pub diva_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepartureResp {
    pub departure_time: i64,
    pub product: String,
    pub label: String,
    pub destination: String,
    pub live: bool,
    pub delay: Option<i32>,
    pub cancelled: bool,
    pub line_background_color: String,
    pub departure_id: String,
    pub sev: bool,
    pub platform: String,
    pub stop_position_number: i32,
    pub info_messages: Vec<String>,
}

pub async fn get_departures(station_id: &str) -> Result<Departures> {
    let url = format!("https://www.mvg.de/api/fahrinfo/departure/{}", station_id);
    let resp = reqwest::get(url).await?.json::<Departures>().await?;
    Ok(resp)
}
