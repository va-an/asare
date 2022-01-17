use crate::{Price, Ticker};

pub type ApiProviderType = Box<dyn ApiProvider + Send + Sync>;

pub trait ApiProvider {
    fn fetch_price(&self, ticker: &Ticker) -> Price;
}
