use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod price_provider;
pub mod rebalancer;
pub mod utils;

pub type Ticker = String;
pub type Price = f32;

pub type Portfolio = HashMap<Ticker, f32>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login: String,
    pub password: String, // FIXME: store hash instead raw password
    pub api_key: String,
}
