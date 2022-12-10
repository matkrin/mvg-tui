use anyhow::Result;
use mvg_cli_rs::api::{get_station,StationResponse, get_departures, get_notifications};

#[tokio::main]
async fn main() -> Result<()>{

    // let s = get_station("ostbahnhof").await?;
    // let r = &s.locations[0];
    //
    // let i = match r {
    //     StationResponse::Station(x) => x.id.clone(),
    //     _ => String::from(""),
    // };
    //
    // let d = get_departures(&i).await?;
    // println!("{:#?}", d);
    let n = get_notifications().await?;
    println!("{:#?}", n);

    Ok(())
}
