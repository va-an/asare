use std::sync::Arc;

use crate::portfolios::portfolios_service::PortfoliosImpl;
use crate::rebalancer;
use crate::users::api_key_matcher::UserApiKeyMatcher;
use crate::users::controller::UsersController;
use crate::users::repository_builder::UserRepositoryBuilder;
use crate::users::user_service::UsersImpl;
use crate::{portfolios, users, Config};
use actix_web::{middleware, web, App, HttpServer};
use async_trait::async_trait;
use chrono::Duration;
use domain::price_provider::finance_api_builder::FinanceApiBuilder;
use domain::price_provider::price_provider_builder::PriceProviderBuilder;
use domain::price_provider::repository_builder::PricesRepoBuilder;
use domain::rebalancer::{controller::RebalancerController, service_builder::RebalancerSvcBuilder};
use domain::utils::ChainingExt;

pub struct AsareApp {
    config: Config,
    portfolio_interactor: PortfolioInteractor,
    rebalancer_ctl: RebalancerController,
    user_ctl: UsersController,
}

pub struct PortfolioInteractor {
    pub portfolios: PortfoliosImpl,
    pub api_key_matcher: UserApiKeyMatcher,
}

impl AsareApp {
    pub fn new(config: Config) -> AsareApp {
        let user_repo = UserRepositoryBuilder::pickle();
        let user_svc = UsersImpl::new(user_repo);

        let portfolios = PortfoliosImpl::new();
        let api_key_matcher = Arc::clone(&user_svc).pipe(UserApiKeyMatcher::new);

        let portfolio_interactor = PortfolioInteractor {
            portfolios,
            api_key_matcher,
        };

        let user_ctl = UsersController::new(user_svc);

        let finance_api = FinanceApiBuilder::random();
        let prices_repo = PricesRepoBuilder::in_memory();
        let price_provider =
            PriceProviderBuilder::default(finance_api, prices_repo, Duration::days(1));
        let rebalancer_svc = RebalancerSvcBuilder::default(price_provider);

        let rebalancer_ctl = RebalancerController::new(rebalancer_svc);

        AsareApp {
            config,
            portfolio_interactor,
            rebalancer_ctl,
            user_ctl,
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
        let user_app_data = web::Data::new(app.user_ctl);
        let portfolio_app_data = web::Data::new(app.portfolio_interactor);

        HttpServer::new(move || {
            App::new()
                .service(
                    web::scope("/v4/rebel/")
                        .service(rebalancer::routes::rebalance)
                        .app_data(rebalancer_app_data.clone()),
                )
                .service(
                    web::scope("/v1/users/")
                        .service(users::routes::create_user)
                        .service(users::routes::login_user)
                        .app_data(user_app_data.clone()),
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
