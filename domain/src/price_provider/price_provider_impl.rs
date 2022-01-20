use chrono::{Duration, Utc};

use crate::{utils::ChainingExt, Price, Ticker};

use super::{
    finance_api::FinanceApiType, price_provider::PriceProvider, repository::PriceRepoType,
};

pub struct PriceProviderImpl {
    pub finance_api: FinanceApiType,
    pub prices_repo: PriceRepoType,
    cache_ttl: Duration,
}

impl PriceProviderImpl {
    pub fn new(
        finance_api: FinanceApiType,
        prices_repo: PriceRepoType,
        cache_ttl: Duration,
    ) -> PriceProviderImpl {
        PriceProviderImpl {
            finance_api,
            prices_repo,
            cache_ttl,
        }
    }
}

impl PriceProvider for PriceProviderImpl {
    fn fetch_price(&self, ticker: &Ticker) -> Price {
        let fetch_and_cache = |ticker: &Ticker| -> Price {
            self.finance_api
                .fetch_price(&ticker)
                .tap(|price| self.prices_repo.save(&ticker, &price))
        };

        self.prices_repo
            .find(&ticker)
            .map_or(fetch_and_cache(ticker), |price_cache| {
                let cache_ttl_seconds = self.cache_ttl.num_seconds();
                let now = Utc::now().timestamp();

                if (now - price_cache.saved_timestamp) > cache_ttl_seconds {
                    fetch_and_cache(ticker)
                } else {
                    price_cache.price
                }
            })
    }
}
