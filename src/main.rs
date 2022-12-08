use anyhow::Result;
use mvg_cli_rs::api::{get_station,StationResponse, get_departures};

#[tokio::main]
async fn main() -> Result<()>{

    // println!("{:#?}", resp.locations[0]);
    let s = get_station("ostbahnhof").await?;
    let r = &s.locations[0];

    let i = match r {
        StationResponse::Station(x) => x.id.clone(),
        _ => String::from(""),
    };

    let d = get_departures(&i).await?;
    println!("{:#?}", d);

    Ok(())
}
