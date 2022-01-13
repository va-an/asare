use crate::rebalancer::controller::RebalanceOutput;
use actix_web::{Error, HttpResponse};

impl From<RebalanceOutput> for Result<HttpResponse, Error> {
    fn from(output: RebalanceOutput) -> Self {
        Ok(HttpResponse::Ok().json(output))
    }
}
