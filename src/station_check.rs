use serde::{Deserialize, Serialize};

use crate::{ApiHandler, ApiUrl};

#[derive(Serialize, Deserialize, Debug)]
pub struct StationCheck {
    pub stationuuid: String,
    pub checkuuid: String,
    pub source: String,
    pub codec: String,
    pub bitrate: u32,
    pub hls: u8,
    pub ok: u8,
    pub timestamp: String,
    pub timestamp_iso8601: String,
    pub urlcache: String,
    pub metainfo_overrides_database: u8,
    pub public: Option<u32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tags: Option<String>,
    pub countrycode: Option<String>,
    pub countrysubdivisioncode: Option<String>,
    pub homepage: Option<String>,
    pub favicon: Option<String>,
    pub loadbalancer: Option<String>,
    pub server_software: Option<String>,
    pub sampling: Option<u32>,
    pub timing_ms: u32,
    pub languagecodes: Option<String>,
    pub ssl_error: u8,
    pub geo_lat: Option<f64>,
    pub geo_long: Option<f64>,
}

#[derive(Serialize, Clone)]
pub enum StationCheckOrder {
    Name,
}

impl Default for StationCheckOrder {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Clone)]
pub struct StationCheckUrl;

impl ApiUrl for StationCheckUrl {
    const URL: &'static str = "http://de1.api.radio-browser.info/json/checks";
}

pub type StationCheckHandler = ApiHandler<StationCheckOrder, StationCheckUrl>;
