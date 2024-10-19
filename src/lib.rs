pub mod codec;
pub mod country;
pub mod language;
pub mod state;
pub mod station;
pub mod tag;

use serde::{Deserialize, Serialize};

pub trait ApiItem {
    fn user_agent(&self) -> String {
        format!("RadioBrowserApi/1.0")
    }
}

pub type ApiResult<T> = Result<Vec<T>, reqwest::Error>;

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
pub struct Language {
    name: String,
    iso_639: Option<String>,
    stationcount: u32,
}
