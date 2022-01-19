use crate::{Price, Ticker};
use rand::prelude::*;

use super::finance_api::FinanceApi;

pub struct FinanceApiMock;

impl FinanceApi for FinanceApiMock {
    fn fetch_price(&self, _ticker: &Ticker) -> Price {
        let mut rng = thread_rng();
        rng.gen_range(100.0..1000.0)
    }
}
