use actix_web::{post, web, Error, HttpResponse};
use domain::{
    rebalancer::{controller::RebalancerController, service::RebalanceInput},
    utils::ChainingExt,
};

use crate::presenters::HttpApiPresenter;

#[post("/rebalance")]
pub async fn rebalance(
    req: web::Json<RebalanceInput>,
    ctl: web::Data<RebalancerController>,
) -> Result<HttpResponse, Error> {
    ctl.rebalance(&req).pipe(HttpApiPresenter::into)
}
