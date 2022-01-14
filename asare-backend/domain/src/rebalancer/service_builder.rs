use super::{service::RebalancerSvc, service_impl::RebalancerImpl};

pub struct RebalancerSvcBuilder;

impl RebalancerSvcBuilder {
    pub fn default() -> RebalancerSvc {
        Box::new(RebalancerImpl {})
    }
}
