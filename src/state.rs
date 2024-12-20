use crate::{ApiHandler, ApiUrl};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct State {
    pub name: String,
    pub country: String,
    pub stationcount: u32,
}

#[derive(Serialize, Clone)]
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

#[derive(Clone)]
pub struct StateUrl;

impl ApiUrl for StateUrl {
    const URL: &'static str = "https://de1.api.radio-browser.info/json/states";
}

pub type StateHandler = ApiHandler<StateOrder, StateUrl>;
