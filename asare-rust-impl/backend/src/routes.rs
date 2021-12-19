use actix_web::{post, web, Error, HttpResponse, Result};

use crate::RebalancerV1;
use crate::{RebalanceInput, Rebalancer};

#[post("/rebalance")]
pub async fn rebalance_request(req: web::Json<RebalanceInput>) -> Result<HttpResponse, Error> {
    let response = RebalancerV1::calc_purchase(&req.0);

    Ok(HttpResponse::Ok().json(response))
}
