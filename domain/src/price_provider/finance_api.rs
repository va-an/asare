use crate::{Price, Ticker};

pub type FinanceApiType = Box<dyn FinanceApi + Send + Sync>;

pub trait FinanceApi {
    fn fetch_price(&self, ticker: &Ticker) -> Price;
}
