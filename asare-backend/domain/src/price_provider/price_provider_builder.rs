use super::{
    api_provider::ApiProviderType, price_provider::PriceProviderType,
    price_provider_impl::PriceProviderImpl, repository::PriceRepoType,
};

pub struct PriceProviderBuilder;

impl PriceProviderBuilder {
    fn default(api_provider: ApiProviderType, repo: PriceRepoType) -> PriceProviderType {
        Box::new(PriceProviderImpl { api_provider, repo })
    }
}
