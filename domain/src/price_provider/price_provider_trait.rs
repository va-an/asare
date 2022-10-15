use crate::{Price, Ticker};

pub type PriceProviderType = Box<dyn PriceProvider + Send + Sync>;

pub trait PriceProvider {
    fn fetch_price(&self, ticker: Ticker) -> Price;
}
