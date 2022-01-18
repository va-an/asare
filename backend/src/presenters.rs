use actix_web::{Error, HttpResponse};
use domain::rebalancer::service::RebalanceOutput;

pub struct HttpApiPresenter;

impl HttpApiPresenter {
    pub fn into(output: RebalanceOutput) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().json(output))
    }
}

// TODO:
// impl From<RebalanceOutput> for Result<HttpResponse, Error> {
//     fn from(output: RebalanceOutput) -> Self {
//         Ok(HttpResponse::Ok().json(output))
//     }
// }
