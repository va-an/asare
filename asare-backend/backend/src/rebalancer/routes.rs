use actix_web::{post, web, Error, HttpResponse};

use crate::{
    app::Portfolio,
    rebalancer::service::{RebalanceInput, RebalancerV1},
    Rebalancer,
};
use serde_derive::Serialize;

#[derive(Serialize, Debug)]
struct RebalanceOutput {
    current_allocation: Portfolio,
    required_operations: Portfolio,
}

#[post("/rebalance")]
pub async fn rebalance_request(req: web::Json<RebalanceInput>) -> Result<HttpResponse, Error> {
    let current_allocation = RebalancerV1::calc_current_allocation(&req.current_portfolio);
    let required_operations = RebalancerV1::calc_purchase(&req);

    let output = RebalanceOutput {
        current_allocation,
        required_operations,
    };

    Ok(HttpResponse::Ok().json(output))
}
