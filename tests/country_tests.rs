extern crate radio_browser;

#[cfg(test)]
mod tests {
    use radio_browser::country::{Country, CountryHandler, CountryOrder};

    #[tokio::test]
    async fn test_countries1() {
        let country_handler = CountryHandler::new(None);

        let countries: Vec<Country> = country_handler.get().await.unwrap();

        println!("Countries: {:?}", countries);

        assert!(countries.len() > 0);
    }

    #[tokio::test]
    async fn test_countries2() {
        let codec_handler = CountryHandler::new(None);

        let countries: Vec<Country> = codec_handler
            .limit(10)
            .order(CountryOrder::StationCount)
            .reverse(true)
            .get()
            .await
            .unwrap();

        println!("Countries: {:?}", countries);

        assert!(countries.len() > 0);
    }

    #[tokio::test]
    async fn test_countries3() {
        let codec_handler = CountryHandler::new(None);

        let countries: Vec<Country> = codec_handler.filter("mp").limit(10).get().await.unwrap();

        println!("Countries: {:?}", countries);

        assert!(countries.len() > 0);
    }
}
