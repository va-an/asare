use actix_web::{post, web, Error, HttpResponse};
use serde::Serialize;

use crate::{
    app::Portfolio,
    rebalancer::rebalancer_service::{RebalanceInput, Rebalancer, RebalancerImpl},
};
pub struct RebalancerController;

impl RebalancerController {
    pub fn rebalance(&self, input: &RebalanceInput) -> RebalanceOutput {
        RebalancerImpl::rebalance(input)
    }
}

#[derive(Serialize, Debug)]
pub struct RebalanceOutput {
    pub current_allocation: Portfolio,
    pub required_operations: Portfolio,
}

#[post("/rebalance")]
pub async fn rebalance_request(
    req: web::Json<RebalanceInput>,
    ctl: web::Data<RebalancerController>,
) -> Result<HttpResponse, Error> {
    ctl.rebalance(&req).into()
}
