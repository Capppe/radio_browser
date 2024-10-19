use serde::Serialize;

#[derive(Serialize)]
pub struct State {
    name: String,
    country: String,
    stationcount: u32,
}
