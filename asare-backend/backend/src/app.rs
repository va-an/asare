use std::collections::HashMap;

use crate::rebalancer::routes::rebalance_request;
use crate::users::routes::{create_user, login_user};
use crate::{Config, UserService};
use actix_web::{middleware, web, App, HttpServer};
use async_trait::async_trait;

pub type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    config: Config,
    user_service: UserService,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        let user_service = UserService::new();

        AsareApp {
            config,
            user_service,
        }
    }

    pub async fn run(self) -> std::io::Result<()> {
        ActixHttpServer::run_http_server(self).await
    }
}

#[async_trait(?Send)]
pub trait AsareHttpServer {
    async fn run_http_server(app: AsareApp) -> std::io::Result<()>;
}
pub struct ActixHttpServer;

#[async_trait(?Send)]
impl AsareHttpServer for ActixHttpServer {
    async fn run_http_server(app: AsareApp) -> std::io::Result<()> {
        let app_data = web::Data::new(app.user_service);

        HttpServer::new(move || {
            App::new()
                .service(web::scope("/v4/rebel/").service(rebalance_request))
                .service(
                    web::scope("/v1/users/")
                        .app_data(app_data.clone())
                        .service(create_user)
                        .service(login_user),
                )
                .wrap(middleware::Logger::default())
        })
        .bind((app.config.http_host.clone(), app.config.http_port))?
        .workers(8)
        .run()
        .await
    }
}
