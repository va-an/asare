use crate::{utils::ChainingExt, Price, Ticker};

use super::{
    finance_api::FinanceApiType, price_provider::PriceProvider, repository::PriceRepoType,
};

// TODO: cache ttl
pub struct PriceProviderImpl {
    pub finance_api: FinanceApiType,
    pub prices_repo: PriceRepoType,
}

impl PriceProvider for PriceProviderImpl {
    fn fetch_price(&self, ticker: &Ticker) -> Price {
        match self.prices_repo.find(&ticker) {
            Some(price) => price,
            None => self
                .finance_api
                .fetch_price(&ticker)
                .tap(|price| self.prices_repo.save(&ticker, &price)),
        }
    }
}
