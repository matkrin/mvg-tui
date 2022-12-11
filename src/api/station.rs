use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "locations", rename_all = "camelCase")]
pub struct Locations {
    pub locations: Vec<StationResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum StationResponse {
    Station(StationResp),
    Address(AddressResp),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StationResp {
    // #[serde(rename = "type")]
    // type_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub id: String,
    pub diva_id: i32,
    pub place: String,
    pub name: String,
    pub has_live_data: bool,
    pub has_zoom_data: bool,
    pub products: Vec<String>,
    pub efa_lon: Option<f64>,
    pub efa_lat: Option<f64>,
    pub link: Option<String>,
    pub tariff_zones: Option<String>,
    pub occupancy: String,
    pub lines: Lines,
}

#[derive(Deserialize, Debug)]
pub struct Lines {
    pub tram: Vec<String>,
    pub nachttram: Vec<String>,
    pub sbahn: Vec<String>,
    pub ubahn: Vec<String>,
    pub bus: Vec<String>,
    pub nachtbus: Vec<String>,
    pub otherlines: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct AddressResp {
    // #[serde(rename = "type")]
    // type_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place: String,
    pub street: String,
    pub poi: bool,
}

pub async fn get_station(station_search: &str) -> Result<Locations> {
    let url = format!(
        "https://www.mvg.de/api/fahrinfo/location/query?q={}",
        station_search
    );
    let resp = reqwest::get(url).await?.json::<Locations>().await?;
    Ok(resp)
}
