use chrono::Duration;

use crate::utils::ChainingExt;

use super::{
    finance_api::FinanceApiType, price_provider::PriceProviderType,
    price_provider_impl::PriceProviderImpl, repository::PriceRepoType,
};

pub struct PriceProviderBuilder;

impl PriceProviderBuilder {
    pub fn default(
        finance_api: FinanceApiType,
        prices_repo: PriceRepoType,
        cache_ttl: Duration,
    ) -> PriceProviderType {
        PriceProviderImpl::new(finance_api, prices_repo, cache_ttl).pipe(Box::new)
    }
}
