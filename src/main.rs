use anyhow::Result;
use mvg_cli_rs::api::{get_station,StationResponse, get_departures, get_notifications, get_routes};

#[tokio::main]
async fn main() -> Result<()>{

    // let s = get_station("uni").await?;
    // let r = &s.locations[0];
    //
    // let i = match r {
    //     StationResponse::Station(x) => x.id.clone(),
    //     _ => String::from("no"),
    // };
    // println!("{}", i);
    //
    // let d = get_departures(&i).await?;
    // println!("{:#?}", d);
    // let n = get_notifications().await?;
    // println!("{:#?}", n);

    let from = get_station("dachau").await?;
    let from_id = if let StationResponse::Station(x) = &from.locations[0] {
        x.id.clone()
    } else {
        "".to_string()
    };
    println!("{}", from_id);

    let to = get_station("hauptbahnhof").await?;
    let to_id = if let StationResponse::Station(x) = &to.locations[0] {
        x.id.clone()
    } else {
        "".to_string()
    };
    println!("{}", to_id);

    let routes = get_routes(&from_id, &to_id, None, None, None, None, None, None, None, None).await?;
    println!("{:#?}", routes);


    Ok(())
}
