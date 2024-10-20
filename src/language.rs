use serde::{Deserialize, Serialize};

use crate::{ApiHandler, ApiUrl};

#[derive(Deserialize, Debug)]
pub struct Language {
    pub name: Option<String>,
    pub stationcount: Option<u32>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LanguageOrder {
    Name,
    StationCount,
}

impl Default for LanguageOrder {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Clone)]
pub struct LanguageUrl;

impl ApiUrl for LanguageUrl {
    const URL: &'static str = "http://de1.api.radio-browser.info/json/languages";
}

pub type LanguageHandler = ApiHandler<LanguageOrder, LanguageUrl>;
