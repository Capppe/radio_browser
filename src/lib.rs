mod codec;
mod country;
mod language;
mod state;
pub mod station;
mod tag;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use station::Station;

pub trait ApiItem {
    fn user_agent(&self) -> String {
        format!("RadioBrowserApi/1.0")
    }
    // fn parameters(&self) -> HashMap<String, String>;
}

// pub trait Parameters {
//     fn order(&self, order: StationOrder);
// }

pub struct ApiHandler<'a> {
    user_agent: &'a str,
    client: reqwest::Client,
    order: Option<&'a str>,
    reverse: Option<bool>,
    offset: Option<u32>,
    limit: Option<u32>,
    hidebroken: Option<bool>,
}

pub type ApiResult = Result<Vec<Station>, reqwest::Error>;

impl<'a> ApiHandler<'a> {
    pub fn new(user_agent: &'a str) -> Self {
        Self {
            user_agent,
            client: reqwest::Client::new(),
            order: None,
            reverse: None,
            offset: None,
            limit: None,
            hidebroken: None,
        }
    }

    fn create_parameter_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(order) = self.order {
            map.insert("order".to_string(), format!("{}", order));
        }

        if let Some(reverse) = self.reverse {
            map.insert("reverse".to_string(), format!("{}", reverse));
        }

        if let Some(offset) = self.offset {
            map.insert("offset".to_string(), format!("{}", offset));
        }

        if let Some(limit) = self.limit {
            map.insert("limit".to_string(), format!("{}", limit));
        }

        if let Some(hidebroken) = self.hidebroken {
            map.insert("hidebroken".to_string(), format!("{}", hidebroken));
        }

        return map;
    }

    async fn get_stations_by_field(&self, field: &str, value: &str) -> ApiResult {
        let request_url =
            format!("http://de1.api.radio-browser.info/json/stations/by{field}/{value}",);

        let params = self.create_parameter_map();

        let response = self
            .client
            .post(&request_url)
            .header(reqwest::header::USER_AGENT, self.user_agent)
            .form(&params)
            .send()
            .await?;

        let stations: Vec<Station> = response.json().await?;

        return Ok(stations);
    }

    pub async fn get_stations_by_uuid(&self, uuid: &str) -> ApiResult {
        Ok(self.get_stations_by_field("uuid", uuid).await?)
    }

    pub async fn get_stations_by_name(&self, name: &str) -> ApiResult {
        Ok(self.get_stations_by_field("name", name).await?)
    }

    pub async fn get_stations_by_nameexact(&self, name: &str) -> ApiResult {
        Ok(self.get_stations_by_field("nameexact", name).await?)
    }

    pub async fn get_stations_by_codec(&self, codec: &str) -> ApiResult {
        Ok(self.get_stations_by_field("codec", codec).await?)
    }

    pub async fn get_stations_by_codecexact(&self, codec: &str) -> ApiResult {
        Ok(self.get_stations_by_field("codecexact", codec).await?)
    }

    pub async fn get_stations_by_country(&self, country: &str) -> ApiResult {
        Ok(self.get_stations_by_field("country", country).await?)
    }

    pub async fn get_stations_by_countryexact(&self, country: &str) -> ApiResult {
        Ok(self.get_stations_by_field("countryexact", country).await?)
    }

    pub async fn get_stations_by_countrycodeexact(&self, country: &str) -> ApiResult {
        Ok(self
            .get_stations_by_field("countrycodeexact", country)
            .await?)
    }

    pub async fn get_stations_by_state(&self, state: &str) -> ApiResult {
        Ok(self.get_stations_by_field("state", state).await?)
    }

    pub async fn get_stations_by_stateexact(&self, state: &str) -> ApiResult {
        Ok(self.get_stations_by_field("stateexact", state).await?)
    }

    pub async fn get_stations_by_language(&self, language: &str) -> ApiResult {
        Ok(self.get_stations_by_field("language", language).await?)
    }

    pub async fn get_stations_by_languageexact(&self, language: &str) -> ApiResult {
        Ok(self
            .get_stations_by_field("languageexact", language)
            .await?)
    }

    pub async fn get_stations_by_tag(&self, tag: &str) -> ApiResult {
        Ok(self.get_stations_by_field("tag", tag).await?)
    }

    pub async fn get_stations_by_tagexact(&self, tag: &str) -> ApiResult {
        Ok(self.get_stations_by_field("tagexact", tag).await?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Country {
    name: String,
    iso_3166_1: String,
    stationcount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Codec {
    name: String,
    stationcount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    name: String,
    country: String,
    stationcount: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Language {
    name: String,
    iso_639: Option<String>,
    stationcount: u32,
}
