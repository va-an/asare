use crate::{Price, Ticker};

pub type PriceRepoType = Box<dyn PriceProviderRepo>;

pub trait PriceProviderRepo {
    fn save(&self, ticker: &Ticker, price: &Price);
    fn find(&self, ticker: &Ticker) -> Option<Price>;
}
