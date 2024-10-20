use serde::{Deserialize, Serialize};

use crate::{ApiHandler, ApiUrl};

#[derive(Serialize, Deserialize, Debug)]
pub struct Codec {
    pub name: String,
    pub stationcount: u32,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CodecOrder {
    Name,
    StationCount,
}

impl Default for CodecOrder {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Clone)]
pub struct CodecUrl;

impl ApiUrl for CodecUrl {
    const URL: &'static str = "http://de1.api.radio-browser.info/json/codecs";
}

pub type CodecHandler = ApiHandler<CodecOrder, CodecUrl>;
