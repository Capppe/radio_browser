use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    name: String,
    stationcount: u32,
}
