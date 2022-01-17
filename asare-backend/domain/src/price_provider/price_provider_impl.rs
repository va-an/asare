use crate::{utils::ChainingExt, Price, Ticker};

use super::{
    api_provider::ApiProvider, price_provider::PriceProvider, repository::PriceProviderRepo,
};

// TODO: cache ttl
pub struct PriceProviderImpl {
    pub api_provider: Box<dyn ApiProvider>,
    pub repo: Box<dyn PriceProviderRepo>,
}

impl PriceProvider for PriceProviderImpl {
    fn fetch_price(&self, ticker: &Ticker) -> Price {
        match self.repo.find(&ticker) {
            Some(price) => price,
            None => self
                .api_provider
                .fetch_price(&ticker)
                .tap(|price| self.repo.save(&ticker, &price)),
        }
    }
}
