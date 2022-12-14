use std::sync::Arc;

use axum::extract::{Json, State};
use axum::response::IntoResponse;
use domain::rebalancer::{controller::RebalancerController, service::RebalanceInput};

pub async fn rebalance(
    State(ctl): State<Arc<RebalancerController>>,
    Json(req): Json<RebalanceInput>,
) -> impl IntoResponse {
    Json(ctl.rebalance(&req))
}
