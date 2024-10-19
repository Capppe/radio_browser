extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::station::StationHandler;

    #[tokio::test]
    async fn test_get_stations_by_country() {
        let station_handler = StationHandler::new();

        let stations = station_handler.by_country("finland").await;

        println!("Stations: {:?}", stations);
    }

    #[tokio::test]
    async fn test_get_stations_by_country_with_params() {
        let station_handler = StationHandler::new();

        let stations = station_handler.limit(1).by_country("finland").await;

        println!("Stations: {:?}", stations);
    }
}
