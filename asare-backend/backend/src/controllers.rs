use crate::rebalancer::{
    rebalancer_service::{RebalanceInput, Rebalancer, RebalancerImpl},
    routes::RebalanceOutput,
};

pub type PresenterType = Box<dyn Presenter + Send + Sync>;

pub trait Presenter {}

pub struct HttpApiPresenter;

impl Presenter for HttpApiPresenter {}

pub struct RebalancerController {
    pub presenter: PresenterType,
}

impl RebalancerController {
    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        let current_allocation = RebalancerImpl::calc_current_allocation(&input.current_portfolio);
        let required_operations = RebalancerImpl::calc_purchase(&input);

        RebalanceOutput {
            current_allocation,
            required_operations,
        }
    }
}
