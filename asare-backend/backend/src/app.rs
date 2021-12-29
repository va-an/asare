use crate::users::routes::{create_user, login_user};
use crate::{conf::Config, rebalancer::routes::rebalance_request};
use actix_web::{middleware, web, App, HttpServer};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait AsareHttpServer {
    async fn run_http_server(config: Config) -> std::io::Result<()>;
}
pub struct ActixHttpServer;

#[async_trait(?Send)]
impl AsareHttpServer for ActixHttpServer {
    async fn run_http_server(config: Config) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(web::scope("/v4/rebel/").service(rebalance_request))
                .service(
                    web::scope("/v1/users/")
                        .service(create_user)
                        .service(login_user),
                )
                .wrap(middleware::Logger::default())
        })
        .bind((config.http_host, config.http_port))?
        .workers(8)
        .run()
        .await
    }
}
