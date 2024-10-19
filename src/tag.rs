use serde::{Deserialize, Serialize};

use crate::{ApiItem, ApiResult};

type TagResult = ApiResult<Tag>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    name: Option<String>,
    stationcount: Option<u32>,
}

#[derive(Serialize, Debug)]
struct Parameters {
    order: TagOrder,
    reverse: bool,
    offset: u32,
    limit: u32,
    hidebroken: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            order: TagOrder::default(),
            reverse: false,
            offset: 0,
            limit: 10000,
            hidebroken: false,
        }
    }
}

#[derive(Serialize, Debug)]
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

pub struct TagHandler {
    client: reqwest::Client,
    params: Parameters,
}

impl ApiItem for TagHandler {}

impl TagHandler {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            params: Parameters::default(),
        }
    }

    pub async fn tags(&self, filter: Option<&str>) -> TagResult {
        let request_url = format!(
            "https://de1.api.radio-browser.info/json/tags/{}",
            filter.unwrap_or("")
        );

        let response = self
            .client
            .post(&request_url)
            .header(reqwest::header::USER_AGENT, self.user_agent())
            .form(&self.params)
            .send()
            .await?;

        println!("Params: {:?}", self.params);
        println!("Url: {}", request_url);
        println!("Resp: {:?}", response);
        let tags: Vec<Tag> = response.json().await?;

        Ok(tags)
    }

    // Parameters
    pub fn order(mut self, val: TagOrder) -> Self {
        self.params.order = val;
        self
    }

    pub fn reverse(mut self, val: bool) -> Self {
        self.params.reverse = val;
        self
    }

    pub fn offset(mut self, val: u32) -> Self {
        self.params.offset = val;
        self
    }

    pub fn limit(mut self, val: u32) -> Self {
        self.params.limit = val;
        self
    }

    pub fn hidebroken(mut self, val: bool) -> Self {
        self.params.hidebroken = val;
        self
    }
}
