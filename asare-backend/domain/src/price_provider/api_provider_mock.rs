use crate::{Price, Ticker};
use rand::prelude::*;

use super::api_provider::ApiProvider;

pub struct ApiProviderMock;

impl ApiProvider for ApiProviderMock {
    fn fetch_price(&self, _ticker: &Ticker) -> Price {
        let mut rng = thread_rng();
        rng.gen_range(100.0..1000.0)
    }
}
