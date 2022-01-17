use super::service::{RebalanceInput, RebalanceOutput, RebalancerSvc};

pub struct RebalancerController {
    rebalancer_svc: RebalancerSvc,
}

impl RebalancerController {
    pub fn new(rebalancer_svc: RebalancerSvc) -> RebalancerController {
        RebalancerController { rebalancer_svc }
    }

    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        self.rebalancer_svc.rebalance_by_amount(input)
    }
}
