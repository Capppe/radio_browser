use serde::Serialize;

use crate::{ApiHandler, ApiUrl};

pub struct Country {
    pub name: String,
    pub iso_3166_1: Option<String>,
    pub stationcount: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CountryOrder {
    Name,
    StationCount,
}

impl Default for CountryOrder {
    fn default() -> Self {
        Self::Name
    }
}

pub struct CountryUrl;

impl ApiUrl for CountryUrl {
    const URL: &'static str = "http://de1.api.radio-browser.info/json/countries";
}

pub type CountryHandler = ApiHandler<CountryOrder, CountryUrl>;
