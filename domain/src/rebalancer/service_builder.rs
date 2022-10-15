use crate::price_provider::price_provider_trait::PriceProviderType;

use super::{service::RebalancerSvcType, service_impl::RebalancerImpl};

pub struct RebalancerSvcBuilder;

impl RebalancerSvcBuilder {
    pub fn default(price_provider: PriceProviderType) -> RebalancerSvcType {
        Box::new(RebalancerImpl { price_provider })
    }
}
