use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Language {
    name: Option<String>,
    stationcount: Option<u32>,
}

#[derive(Serialize, Debug)]
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
