use crate::{ApiHandler, ApiUrl};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    pub changeuuid: Option<String>,
    pub stationuuid: Option<String>,
    pub serveruuid: Option<String>,
    pub name: String,
    pub url: String,
    pub url_resolved: String,
    pub homepage: String,
    pub favicon: String,
    pub tags: String,
    pub country: String,
    pub countrycode: String,
    pub iso_3166_2: Option<String>,
    pub state: String,
    pub language: String,
    pub languagecodes: String,
    pub votes: u32,
    pub lastchangetime: String,
    pub lastchangetime_iso8601: String,
    pub codec: String,
    pub bitrate: u32,
    pub hls: u8,
    pub lastcheckok: u8,
    pub lastchecktime: String,
    pub lastchecktime_iso8601: String,
    pub lastcheckoktime: String,
    pub lastcheckoktime_iso8601: String,
    pub lastlocalchecktime: String,
    pub lastlocalchecktime_iso8601: String,
    pub clicktimestamp: String,
    pub clicktimestamp_iso8601: Option<String>,
    pub clickcount: u32,
    pub clicktrend: i32,
    pub ssl_error: u8,
    pub geo_lat: Option<f64>,
    pub geo_long: Option<f64>,
    pub has_extended_info: bool,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum StationOrder {
    Name,
    Url,
}

impl Default for StationOrder {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Clone)]
pub struct StationUrl;

impl ApiUrl for StationUrl {
    const URL: &'static str = "https://de1.api.radio-browser.info/json/stations";
}

pub type StationHandler = ApiHandler<StationOrder, StationUrl>;

impl<'a> StationHandler {
    pub fn by_uuid(mut self, uuid: &str) -> Self {
        self.url = format!("{}/byuuid/{}", self.url, uuid);
        self
    }

    pub fn by_name(mut self, name: &str) -> Self {
        self.url = format!("{}/byname/{}", self.url, name);
        self
    }

    pub fn by_nameexact(mut self, name: &str) -> Self {
        self.url = format!("{}/bynameexact/{}", self.url, name);
        self
    }

    pub fn by_codec(mut self, codec: &str) -> Self {
        self.url = format!("{}/bycodec/{}", self.url, codec);
        self
    }

    pub fn by_codecexact(mut self, codec: &str) -> Self {
        self.url = format!("{}/bycodecexact/{}", self.url, codec);
        self
    }

    pub fn by_country(mut self, country: &str) -> Self {
        let mut country = country.to_string();
        let c = format!("{}{country}", country.remove(0).to_uppercase());

        self.url = format!("{}/bycountry/{}", self.url, c);
        self
    }

    pub fn by_countryexact(mut self, country: &str) -> Self {
        self.url = format!("{}/bycountryexact/{}", self.url, country);
        self
    }

    pub fn by_countrycodeexact(mut self, country: &str) -> Self {
        self.url = format!("{}/bycountrycodeexact/{}", self.url, country);
        self
    }

    pub fn by_state(mut self, state: &str) -> Self {
        self.url = format!("{}/bystate/{}", self.url, state);
        self
    }

    pub fn by_stateexact(mut self, state: &str) -> Self {
        self.url = format!("{}/bystateexact/{}", self.url, state);
        self
    }

    pub fn by_language(mut self, language: &str) -> Self {
        self.url = format!("{}/bylanguage/{}", self.url, language);
        self
    }

    pub fn by_languageexact(mut self, language: &str) -> Self {
        self.url = format!("{}/bylanguageexact/{}", self.url, language);
        self
    }

    pub fn by_tag(mut self, tag: &str) -> Self {
        self.url = format!("{}/bytag/{}", self.url, tag);
        self
    }

    pub fn by_tagexact(mut self, tag: &str) -> Self {
        self.url = format!("{}/bytagexact/{}", self.url, tag);
        self
    }
}
