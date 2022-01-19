use super::{
    finance_api::FinanceApiType, price_provider::PriceProviderType,
    price_provider_impl::PriceProviderImpl, repository::PriceRepoType,
};

pub struct PriceProviderBuilder;

impl PriceProviderBuilder {
    pub fn default(finance_api: FinanceApiType, prices_repo: PriceRepoType) -> PriceProviderType {
        Box::new(PriceProviderImpl {
            finance_api,
            prices_repo,
        })
    }
}
