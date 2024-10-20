use crate::{ApiHandler, ApiUrl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct State {
    name: String,
    country: String,
    stationcount: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StateOrder {
    Name,
    StationCount,
}

impl Default for StateOrder {
    fn default() -> Self {
        Self::Name
    }
}

pub struct StateUrl;

impl ApiUrl for StateUrl {
    const URL: &'static str = "https://de1.api.radio-browser.info/json/states";
}

pub type StateHandler = ApiHandler<StateOrder, StateUrl>;
