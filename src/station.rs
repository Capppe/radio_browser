use serde::{Deserialize, Serialize};

use crate::{ApiItem, ApiResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct Station {
    changeuuid: Option<String>,
    stationuuid: Option<String>,
    serveruuid: Option<String>,
    name: String,
    url: String,
    url_resolved: String,
    homepage: String,
    favicon: String,
    tags: String,
    country: String,
    countrycode: String,
    iso_3166_2: Option<String>,
    state: String,
    language: String,
    languagecodes: String,
    votes: u32,
    lastchangetime: String,
    lastchangetime_iso8601: String,
    codec: String,
    bitrate: u32,
    hls: u8,
    lastcheckok: u8,
    lastchecktime: String,
    lastchecktime_iso8601: String,
    lastcheckoktime: String,
    lastcheckoktime_iso8601: String,
    lastlocalchecktime: String,
    lastlocalchecktime_iso8601: String,
    clicktimestamp: String,
    clicktimestamp_iso8601: Option<String>,
    clickcount: u32,
    clicktrend: i32,
    ssl_error: u8,
    geo_lat: Option<f64>,
    geo_long: Option<f64>,
    has_extended_info: bool,
}

impl Station {}

#[derive(Serialize)]
struct Parameters {
    order: StationOrder,
    reverse: bool,
    offset: u32,
    limit: u32,
    hidebroken: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            order: StationOrder::default(),
            reverse: false,
            offset: 0,
            limit: 10000,
            hidebroken: false,
        }
    }
}

#[derive(Serialize)]
pub enum StationOrder {
    Name,
    Url,
}

impl Default for StationOrder {
    fn default() -> Self {
        Self::Name
    }
}

pub struct StationHandler {
    client: reqwest::Client,
    params: Parameters,
}

impl ApiItem for StationHandler {}

impl StationHandler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            params: Parameters::default(),
        }
    }

    async fn get_stations_by_field(&self, field: &str, value: &str) -> ApiResult
    where
        Self: ApiItem,
    {
        let request_url =
            format!("http://de1.api.radio-browser.info/json/stations/by{field}/{value}",);

        let response = self
            .client
            .post(&request_url)
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .form(&self.params)
            .send()
            .await?;

        let stations: Vec<Station> = response.json().await?;

        return Ok(stations);
    }

    pub async fn by_uuid(&self, uuid: &str) -> ApiResult {
        Ok(self.get_stations_by_field("uuid", uuid).await?)
    }

    pub async fn by_name(&self, name: &str) -> ApiResult {
        Ok(self.get_stations_by_field("name", name).await?)
    }

    pub async fn by_nameexact(&self, name: &str) -> ApiResult {
        Ok(self.get_stations_by_field("nameexact", name).await?)
    }

    pub async fn by_codec(&self, codec: &str) -> ApiResult {
        Ok(self.get_stations_by_field("codec", codec).await?)
    }

    pub async fn by_codecexact(&self, codec: &str) -> ApiResult {
        Ok(self.get_stations_by_field("codecexact", codec).await?)
    }

    pub async fn by_country(&self, country: &str) -> ApiResult {
        Ok(self.get_stations_by_field("country", country).await?)
    }

    pub async fn by_countryexact(&self, country: &str) -> ApiResult {
        Ok(self.get_stations_by_field("countryexact", country).await?)
    }

    pub async fn by_countrycodeexact(&self, country: &str) -> ApiResult {
        Ok(self
            .get_stations_by_field("countrycodeexact", country)
            .await?)
    }

    pub async fn by_state(&self, state: &str) -> ApiResult {
        Ok(self.get_stations_by_field("state", state).await?)
    }

    pub async fn by_stateexact(&self, state: &str) -> ApiResult {
        Ok(self.get_stations_by_field("stateexact", state).await?)
    }

    pub async fn by_language(&self, language: &str) -> ApiResult {
        Ok(self.get_stations_by_field("language", language).await?)
    }

    pub async fn by_languageexact(&self, language: &str) -> ApiResult {
        Ok(self
            .get_stations_by_field("languageexact", language)
            .await?)
    }

    pub async fn by_tag(&self, tag: &str) -> ApiResult {
        Ok(self.get_stations_by_field("tag", tag).await?)
    }

    pub async fn by_tagexact(&self, tag: &str) -> ApiResult {
        Ok(self.get_stations_by_field("tagexact", tag).await?)
    }

    // Parameters
    pub fn order(mut self, val: StationOrder) -> Self {
        self.params.order = val;
        self
    }

    pub fn reverse(mut self, val: bool) -> Self {
        self.params.reverse = val;
        self
    }

    pub fn offset(mut self, val: u32) -> Self {
        self.params.offset = val;
        self
    }

    pub fn limit(mut self, val: u32) -> Self {
        self.params.limit = val;
        self
    }

    pub fn hidebroken(mut self, val: bool) -> Self {
        self.params.hidebroken = val;
        self
    }
}
