use actix_web::{post, web, Error, HttpResponse};

use crate::rebalancer::{controller::RebalancerController, rebalancer_service::RebalanceInput};

#[post("/rebalance")]
pub async fn rebalance(
    req: web::Json<RebalanceInput>,
    ctl: web::Data<RebalancerController>,
) -> Result<HttpResponse, Error> {
    ctl.rebalance(&req).into()
}
