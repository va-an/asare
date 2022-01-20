use crate::{Price, Ticker};

pub type PriceRepoType = Box<dyn PriceProviderRepo + Send + Sync>;

pub struct PriceCache {
    pub price: Price,
    pub saved_timestamp: i64,
}

pub trait PriceProviderRepo {
    fn save(&self, ticker: &Ticker, price: &Price);
    fn find(&self, ticker: &Ticker) -> Option<PriceCache>;
}
