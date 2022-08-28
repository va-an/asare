use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use domain::rebalancer::{controller::RebalancerController, service::RebalanceInput};

pub async fn rebalance(
    Extension(ctl): Extension<Arc<RebalancerController>>,
    Json(req): Json<RebalanceInput>,
) -> impl IntoResponse {
    Json(ctl.rebalance(&req))
}
