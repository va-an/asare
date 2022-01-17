use crate::{Price, Ticker};

pub type PriceProviderType = Box<dyn PriceProvider>;

pub trait PriceProvider {
    fn fetch_price(&self, ticker: &Ticker) -> Price;
}
