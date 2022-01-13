use std::collections::HashMap;
use std::sync::Arc;

use crate::portfolios::portfolios_service::PortfoliosImpl;
use crate::rebalancer::controller::{rebalance_request, RebalancerController};
use crate::users::api_key_matcher::UserApiKeyMatcher;
use crate::users::controller::UsersController;
use crate::users::repository_builder::UserRepositoryBuilder;
use crate::users::users_service::UsersImpl;
use crate::utils::ChainingExt;
use crate::{portfolios, users, Config};
use actix_web::{middleware, web, App, HttpServer};
use async_trait::async_trait;

pub type Portfolio = HashMap<String, f32>;

pub struct AsareApp {
    config: Config,
    portfolio_interactor: PortfolioInteractor,
    rebalancer_ctl: RebalancerController,
    users_ctl: UsersController,
}

pub struct PortfolioInteractor {
    pub portfolios: PortfoliosImpl,
    pub api_key_matcher: UserApiKeyMatcher,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        let users_repo = UserRepositoryBuilder::in_memory();
        let users_svc = UsersImpl::new(users_repo);

        let portfolios = PortfoliosImpl::new();
        let api_key_matcher = Arc::clone(&users_svc).pipe(UserApiKeyMatcher::new);

        let portfolio_interactor = PortfolioInteractor {
            portfolios,
            api_key_matcher,
        };

        let rebalancer_ctl = RebalancerController {};
        let users_ctl = UsersController::new(users_svc);

        AsareApp {
            config,
            portfolio_interactor,
            rebalancer_ctl,
            users_ctl,
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
        let rebalancer_app_data = app.rebalancer_ctl.pipe(web::Data::new);
        let users_app_data = web::Data::new(app.users_ctl);
        let portfolio_app_data = web::Data::new(app.portfolio_interactor);

        HttpServer::new(move || {
            App::new()
                .service(
                    web::scope("/v4/rebel/")
                        .service(rebalance_request)
                        .app_data(rebalancer_app_data.clone()),
                )
                .service(
                    web::scope("/v1/users/")
                        .service(users::routes::create_user)
                        .service(users::routes::login_user)
                        .app_data(users_app_data.clone()),
                )
                .service(
                    web::scope("/v1/portfolios/")
                        .service(portfolios::routes::create)
                        .service(portfolios::routes::delete)
                        .service(portfolios::routes::find)
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
