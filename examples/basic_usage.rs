use radio_browser::station::{Station, StationHandler, StationOrder};

#[tokio::main]
async fn main() {
    let station_handler = StationHandler::new(None);

    // Get 10 stations ordered by name, reversed
    let stations1: Vec<Station> = station_handler
        .clone()
        .limit(10)
        .order(StationOrder::Name)
        .reverse(true)
        .get()
        .await
        .unwrap();

    println!("Stations: {:?}", stations1);

    // Get 10 stations, filtered(searched) by country
    let stations2: Vec<Station> = station_handler
        .limit(10)
        .by_country("finland")
        .get()
        .await
        .unwrap();

    println!("Stations in finland: {:?}", stations2);
}
