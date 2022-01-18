use super::service::{RebalanceInput, RebalanceOutput, RebalancerSvcType};

pub struct RebalancerController {
    rebalancer_svc: RebalancerSvcType,
}

impl RebalancerController {
    pub fn new(rebalancer_svc: RebalancerSvcType) -> RebalancerController {
        RebalancerController { rebalancer_svc }
    }

    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        self.rebalancer_svc.rebalance_by_amount(input)
    }
}
