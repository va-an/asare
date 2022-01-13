use crate::rebalancer::rebalancer_service::{
    RebalanceInput, RebalanceOutput, Rebalancer, RebalancerImpl,
};

pub struct RebalancerController;

impl RebalancerController {
    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        RebalancerImpl::rebalance(input)
    }
}
