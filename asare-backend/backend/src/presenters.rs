use actix_web::{Error, HttpResponse};

use crate::rebalancer::rebalancer_service::RebalanceOutput;

impl From<RebalanceOutput> for Result<HttpResponse, Error> {
    fn from(output: RebalanceOutput) -> Self {
        Ok(HttpResponse::Ok().json(output))
    }
}

impl From<RebalanceOutput> for String {
    fn from(output: RebalanceOutput) -> Self {
        format!("{:#?}", output)
    }
}
