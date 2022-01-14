use crate::Portfolio;
use serde::{Deserialize, Serialize};

pub type RebalancerSvc = Box<dyn Rebalancer + Sync + Send>;

pub trait Rebalancer {
    fn calc_current_allocation(&self, portfolio: &Portfolio) -> Portfolio;
    fn calc_expected_portfolio(&self, input: &RebalanceInput) -> Portfolio;
    fn calc_purchase(&self, input: &RebalanceInput) -> Portfolio;
    fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput;
}

#[derive(Deserialize, Debug)]
pub struct RebalanceInput {
    pub current_portfolio: Portfolio,
    pub required_allocation: Portfolio,
}

#[derive(Serialize, Debug)]
pub struct RebalanceOutput {
    pub current_allocation: Portfolio,
    pub required_operations: Portfolio,
}
