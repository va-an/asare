use actix_web::{post, web, Error, HttpResponse};

use crate::{
    app::Portfolio, controllers::RebalancerController,
    rebalancer::rebalancer_service::RebalanceInput, utils::ChainingExt,
};
use serde_derive::Serialize;

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
    // TODO: wrap to HttpResponse in presenter
    ctl.rebalance(&req)
        .pipe(|output| Ok(HttpResponse::Ok().json(output)))
}
