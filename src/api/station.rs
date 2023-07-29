use anyhow::Result;
use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// #[serde(tag = "locations", rename_all = "camelCase")]
// pub struct Locations {
//     pub locations: Vec<StationResponse>,
// }

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "UPPERCASE")]
pub enum Location {
    Station(StationResp),
    Address(AddressResp),
    Poi(PoiResponse),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StationResp {
    // #[serde(rename = "type")]
    // pub type_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place: String,
    pub name: String,
    pub global_id: String,
    pub diva_id: i32,
    // pub has_live_data: bool,
    pub has_zoom_data: bool,
    pub transport_types: Vec<String>,
    pub surrounding_plan_link: String,
    pub aliases: String,
    pub tariff_zones: String,
    // pub tariff_zones: Option<String>,
    // pub products: Vec<String>,
    // pub efa_lon: Option<f64>,
    // pub efa_lat: Option<f64>,
    // pub link: Option<String>,
    // pub occupancy: String,
}



// #[derive(Deserialize, Debug)]
// pub struct Lines {
//     pub tram: Vec<String>,
//     pub nachttram: Vec<String>,
//     pub sbahn: Vec<String>,
//     pub ubahn: Vec<String>,
//     pub bus: Vec<String>,
//     pub nachtbus: Vec<String>,
//     pub otherlines: Vec<String>,
// }

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressResp {
    // #[serde(rename = "type")]
    // pub type_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place: String,
    pub name: String,
    pub post_code: String,
    pub street: String,
    pub house_number: String,
    // pub poi: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PoiResponse {
    // #[serde(rename = "type")]
    // pub type_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub place: String,
    pub name: String,
}

pub async fn get_station(station_search: &str) -> Result<Vec<Location>> {
    let url = format!(

        "https://www.mvg.de/api/fib/v2/location?query={}",
        // "https://www.mvg.de/api/fahrinfo/location/query?q={}",
        station_search
    );
    let resp = reqwest::get(url).await?.json::<Vec<Location>>().await?;
    Ok(resp)
}
