use std::sync::Arc;

use crate::portfolios::service::PortfoliosImpl;
use crate::rebalancer::routes::rebalance;
use crate::users::api_key_matcher::UserApiKeyMatcher;
use crate::users::controller::UsersController;
use crate::users::repository_builder::UserRepositoryBuilder;
use crate::users::service::UsersImpl;
use crate::{portfolios, users, Config};

use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Extension, Router};
use chrono::Duration;
use domain::price_provider::finance_api_builder::FinanceApiBuilder;
use domain::price_provider::price_provider_builder::PriceProviderBuilder;
use domain::price_provider::repository_builder::PricesRepoBuilder;
use domain::rebalancer::{controller::RebalancerController, service_builder::RebalancerSvcBuilder};
use domain::utils::ChainingExt;
use sqlx::postgres::PgPoolOptions;

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
    pub async fn new(config: Config) -> AsareApp {
        let db_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.db.user, config.db.password, config.db.host, config.db.port, config.db.name
        );

        let pool = PgPoolOptions::new()
            .max_connections(config.db.max_connections)
            .connect(&db_url)
            .await
            .unwrap();

        sqlx::migrate!().run(&pool).await.unwrap();

        let user_repo = UserRepositoryBuilder::postgres(pool.clone());
        let user_svc = UsersImpl::new(user_repo).await;

        let portfolios = PortfoliosImpl::new(pool.clone());
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

    pub async fn run(self) {
        // from `build.rs`
        let version = env!("ASARE_BACKEND_VERSION");
        log::info!("version {}", version);

        let rebalancer_ctl = Arc::new(self.rebalancer_ctl);
        let portfolio_interactor = Arc::new(self.portfolio_interactor);
        let user_ctl = Arc::new(self.user_ctl);

        let router_rebalance = Router::new()
            .route("/v4/rebel/rebalance", post(rebalance))
            .layer(Extension(rebalancer_ctl));

        let router_portfolios = Router::new()
            .route(
                "/v1/portfolios/",
                get(portfolios::routes::find).post(portfolios::routes::create),
            )
            .route("/v1/portfolios/:id", delete(portfolios::routes::delete))
            .layer(Extension(portfolio_interactor));

        let router_users = Router::new()
            .route("/v1/users/", post(users::routes::create_user))
            .route("/v1/users/refresh_api_key", post(users::routes::login_user))
            .layer(Extension(user_ctl));

        let router_version =
            Router::new().route("/version", get(move || async { version.into_response() }));

        let app = Router::new()
            .merge(router_rebalance)
            .merge(router_portfolios)
            .merge(router_users)
            .merge(router_version);

        let address = format!("{}:{}", self.config.http_host, self.config.http_port);

        axum::Server::bind(&address.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
}
