use crate::{Price, Ticker};

pub type ApiProviderType = Box<dyn ApiProvider>;

pub trait ApiProvider {
    fn fetch_price(&self, ticker: &Ticker) -> Price;
}
