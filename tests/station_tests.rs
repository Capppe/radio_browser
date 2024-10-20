extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::station::{Station, StationHandler};

    #[tokio::test]
    async fn test_stations1() {
        let station_handler = StationHandler::new(None);

        let stations: Vec<Station> = station_handler.by_country("finland").get().await.unwrap();

        println!("Stations: {:?}", stations);

        assert!(stations.len() > 0);
    }

    #[tokio::test]
    async fn test_stations2() {
        let station_handler = StationHandler::new(None);

        let stations: Vec<Station> = station_handler
            .limit(1)
            .by_country("finland")
            .get()
            .await
            .unwrap();

        println!("Stations: {:?}", stations);

        assert!(!stations.len() > 1);
    }

    #[tokio::test]
    async fn test_stations3() {
        let station_handler = StationHandler::new(None);

        let stations: Vec<Station> = station_handler
            .limit(10)
            .by_name("vega")
            .get()
            .await
            .unwrap();

        println!("Stations: {:?}", stations);

        assert!(!stations.len() > 1);
    }
}
