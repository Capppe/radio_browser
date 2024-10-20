use crate::{ApiHandler, ApiUrl};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StationClick {
    pub ok: bool,
    pub message: String,
    pub stationuuid: String,
    pub name: String,
    pub url: String,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StationClickOrder {
    Name,
    Url,
}

impl Default for StationClickOrder {
    fn default() -> Self {
        Self::Name
    }
}

pub struct StationClickUrl;

impl ApiUrl for StationClickUrl {
    const URL: &'static str = "https://de1.api.radio-browser.info/json/url";
}

pub type StationClickHandler = ApiHandler<StationClickOrder, StationClickUrl>;

impl StationClickHandler {
    pub fn uuid(mut self, uuid: &str) -> Self {
        self.url = format!("{}/{}", self.url, uuid);
        self
    }
}
