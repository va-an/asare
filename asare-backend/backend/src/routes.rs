use actix_web::{middleware, post, web, App, Error, HttpResponse, HttpServer, Result};

use crate::conf::Config;
use crate::{Portfolio, RebalancerV1};
use crate::{RebalanceInput, Rebalancer};
use async_trait::async_trait;
use serde_derive::Serialize;

#[async_trait(?Send)]
pub trait AsareHttpServer {
    async fn run_http_server(config: Config) -> std::io::Result<()>;
}
pub struct ActixHttpServer {}

#[async_trait(?Send)]
impl AsareHttpServer for ActixHttpServer {
    async fn run_http_server(config: Config) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(web::scope("/v4/rebel/").service(rebalance_request))
                .wrap(middleware::Logger::default())
        })
        .bind((config.http_host, config.http_port))?
        .workers(8)
        .run()
        .await
    }
}

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
