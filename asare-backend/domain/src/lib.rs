use std::collections::HashMap;

pub mod price_provider;
pub mod rebalancer;
pub mod utils;

pub type Ticker = String;
pub type Price = f32;
pub type Portfolio = HashMap<Ticker, Price>;
