use super::{
    api_provider::ApiProviderType, price_provider::PriceProviderType,
    price_provider_impl::PriceProviderImpl, repository::PriceRepoType,
};

pub struct PriceProviderBuilder;

impl PriceProviderBuilder {
    pub fn default(api_provider: ApiProviderType, prices_repo: PriceRepoType) -> PriceProviderType {
        Box::new(PriceProviderImpl {
            api_provider,
            prices_repo,
        })
    }
}
