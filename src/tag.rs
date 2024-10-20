use crate::{ApiHandler, ApiUrl};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub name: Option<String>,
    pub stationcount: Option<u32>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TagOrder {
    Name,
    StationCount,
}

impl Default for TagOrder {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Clone)]
pub struct TagUrl;

impl ApiUrl for TagUrl {
    const URL: &'static str = "https://de1.api.radio-browser.info/json/tags";
}

pub type TagHandler = ApiHandler<TagOrder, TagUrl>;
