use serde::{Deserialize, Serialize};

use crate::{ApiHandler, ApiUrl};

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
    pub name: String,
    pub iso_3166_1: Option<String>,
    pub stationcount: u32,
}

#[derive(Serialize, Clone)]
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

#[derive(Clone)]
pub struct CountryUrl;

impl ApiUrl for CountryUrl {
    const URL: &'static str = "http://de1.api.radio-browser.info/json/countries";
}

pub type CountryHandler = ApiHandler<CountryOrder, CountryUrl>;
