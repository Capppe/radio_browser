pub mod codec;
pub mod country;
pub mod language;
pub mod state;
pub mod station;
pub mod station_check;
pub mod station_clicks;
pub mod tag;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData};

pub type ApiResult<T> = Result<Vec<T>, reqwest::Error>;

#[derive(Serialize)]
pub struct Parameters<Order> {
    order: Order,
    reverse: bool,
    offset: u32,
    limit: u32,
    hidebroken: bool,
}

impl<Order: Default> Default for Parameters<Order> {
    fn default() -> Self
where {
        Self {
            order: Order::default(),
            reverse: false,
            offset: 0,
            limit: 10000,
            hidebroken: false,
        }
    }
}

pub trait ApiUrl {
    const URL: &'static str;
}

pub struct ApiHandler<Order, U: ApiUrl> {
    client: reqwest::Client,
    params: Parameters<Order>,
    url: String,
    user_agent: String,
    phantom: PhantomData<U>,
}

impl<Order: serde::Serialize + Default, U: ApiUrl> ApiHandler<Order, U> {
    pub fn new(user_agent: Option<&str>) -> Self {
        Self {
            client: reqwest::Client::new(),
            params: Parameters::default(),
            url: U::URL.to_string(),
            user_agent: user_agent.unwrap_or("RadioBrowserApi/1.0").to_string(),
            phantom: PhantomData,
        }
    }

    pub fn order(mut self, val: Order) -> Self {
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

    pub fn filter(mut self, val: &str) -> Self {
        self.url = format!("{}/{}", self.url, val);
        self
    }

    pub fn params(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();

        let order: Value = serde_json::to_value(&self.params.order).unwrap_or("".into());

        if let Value::String(s) = order {
            map.insert("order", s);
        }

        map.insert("reverse", self.params.reverse.to_string());
        map.insert("offset", self.params.offset.to_string());
        map.insert("limit", self.params.limit.to_string());
        map.insert("hidebroken", self.params.hidebroken.to_string());

        map
    }

    pub async fn get<T>(&self) -> ApiResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let request_url = format!("{}", self.url);

        println!("Params: {:?}", self.params());

        let response = self
            .client
            .post(&request_url)
            .header(reqwest::header::USER_AGENT, self.user_agent.to_owned())
            .form(&self.params())
            .send()
            .await?;

        println!("Url: {}", request_url);
        println!("Resp: {:?}", response);

        let result: Vec<T> = response.json().await?;

        Ok(result)
    }
}

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
