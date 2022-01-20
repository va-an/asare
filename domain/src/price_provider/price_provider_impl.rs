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
    fn fetch_price(&self, ticker: Ticker) -> Price {
        let fetch_and_cache = |ticker: &Ticker| -> Price {
            self.finance_api
                .fetch_price(&ticker)
                .tap(|price| self.prices_repo.save(&ticker, &price))
        };

        self.prices_repo
            .find(&ticker)
            .map_or(fetch_and_cache(&ticker), |price_cache| {
                let cache_ttl_seconds = self.cache_ttl.num_seconds();
                let now = Utc::now().timestamp();

                if (now - price_cache.saved_timestamp) > cache_ttl_seconds {
                    fetch_and_cache(&ticker)
                } else {
                    price_cache.price
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::{self, time};

    use chrono::Duration;

    use crate::price_provider::{
        finance_api_builder::FinanceApiBuilder, price_provider::PriceProviderType,
        price_provider_builder::PriceProviderBuilder, repository_builder::PricesRepoBuilder,
    };

    fn get_price_provider() -> PriceProviderType {
        let finance_api = FinanceApiBuilder::random();
        let prices_repo = PricesRepoBuilder::in_memory();
        let price_provider =
            PriceProviderBuilder::default(finance_api, prices_repo, Duration::seconds(1));

        price_provider
    }

    #[test]
    fn fetch_price() {
        let price_provider = get_price_provider();
        let some_ticker = "A".to_string();

        let from_cache = price_provider.fetch_price(some_ticker.to_owned());
        let from_cache_again = price_provider.fetch_price(some_ticker.to_owned());
        assert_eq!(from_cache, from_cache_again);

        thread::sleep(time::Duration::from_secs(2));

        let after_cache_expired = price_provider.fetch_price(some_ticker);
        assert_ne!(from_cache, after_cache_expired);
    }
}
