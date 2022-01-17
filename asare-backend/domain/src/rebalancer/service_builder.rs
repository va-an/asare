use crate::price_provider::price_provider::PriceProviderType;

use super::{service::RebalancerSvc, service_impl::RebalancerImpl};

pub struct RebalancerSvcBuilder;

impl RebalancerSvcBuilder {
    pub fn default(price_provider: PriceProviderType) -> RebalancerSvc {
        Box::new(RebalancerImpl { price_provider })
    }
}
