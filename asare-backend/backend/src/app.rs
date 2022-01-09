use std::collections::HashMap;
use std::sync::Arc;

use crate::portfolio::service::PortfolioService;
use crate::rebalancer::routes::rebalance_request;
use crate::users::api_key_matcher::UserApiKeyMatcher;
use crate::users::routes::{create_user, login_user};
use crate::{portfolio, Config, UserService};
use actix_web::{middleware, web, App, HttpServer};
use async_trait::async_trait;

pub type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    config: Config,
    user_service: Arc<UserService>,
    portfolio_interactor: PortfolioInteractor,
}

pub struct PortfolioInteractor {
    pub service: PortfolioService,
    pub api_key_matcher: UserApiKeyMatcher,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        let user_service = Arc::new(UserService::new());
        let portfolio_service = PortfolioService::new();
        let api_key_matcher = UserApiKeyMatcher::new(Arc::clone(&user_service));

        let portfolio_interactor = PortfolioInteractor {
            service: portfolio_service,
            api_key_matcher,
        };

        AsareApp {
            config,
            user_service,
            portfolio_interactor,
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
        let user_app_data = web::Data::new(app.user_service);
        let portfolio_app_data = web::Data::new(app.portfolio_interactor);

        HttpServer::new(move || {
            App::new()
                .service(web::scope("/v4/rebel/").service(rebalance_request))
                .service(
                    web::scope("/v1/users/")
                        .service(create_user)
                        .service(login_user)
                        .app_data(user_app_data.clone()),
                )
                .service(
                    web::scope("/v1/portfolios/")
                        .service(rebalance_request)
                        .service(portfolio::routes::create)
                        .service(portfolio::routes::delete)
                        .service(portfolio::routes::find)
                        .app_data(portfolio_app_data.clone()),
                )
                .wrap(middleware::Logger::default())
        })
        .bind((app.config.http_host.to_owned(), app.config.http_port))?
        .workers(8)
        .run()
        .await
    }
}
