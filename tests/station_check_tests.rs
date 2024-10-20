extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::station_check::{StationCheck, StationCheckHandler};

    #[tokio::test]
    async fn test_station_checks1() {
        let station_check_handler = StationCheckHandler::new(None);

        let checks: Vec<StationCheck> = station_check_handler.get().await.unwrap();

        println!("Checks: {:?}", checks);

        assert!(checks.len() > 0);
    }

    #[tokio::test]
    async fn test_station_checks2() {
        let station_check_handler = StationCheckHandler::new(None);

        let checks: Vec<StationCheck> = station_check_handler.limit(10).get().await.unwrap();

        println!("Checks: {:?}", checks);

        assert!(checks.len() > 0);
    }

    #[tokio::test]
    async fn test_station_checks3() {
        let station_check_handler = StationCheckHandler::new(None);

        let checks: Vec<StationCheck> = station_check_handler
            .filter("77113a1f-0c93-46ea-9005-2546d33963fe")
            .get()
            .await
            .unwrap();

        println!("Checks: {:?}", checks);

        assert!(checks.len() > 0);
    }
}
