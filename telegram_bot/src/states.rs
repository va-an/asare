use std::sync::{Arc, Mutex};

use chrono::Duration;
use domain::{
    price_provider::{
        finance_api_builder::FinanceApiBuilder, price_provider_builder::PriceProviderBuilder,
        repository_builder::PricesRepoBuilder,
    },
    rebalancer::{service::RebalancerSvcType, service_builder::RebalancerSvcBuilder},
    utils::ChainingExt,
};

use crate::api_key_mapper::{mapper::ApiKeyMapperType, mapper_builder::ApiKeyMapperBuilder};

pub mod main_lupa;
pub mod rebalance;

#[derive(Clone)]
pub struct MainLupaState {
    pub rebalancer_svc: Arc<RebalancerSvcType>,
    pub api_key_mapper: Arc<Mutex<ApiKeyMapperType>>,
}

#[derive(Clone)]
pub enum State {
    MainLupa { state: MainLupaState },
    RebalanceByAmount { state: MainLupaState },
    RebalanceByPrice { state: MainLupaState },
}

impl Default for State {
    fn default() -> Self {
        let finance_api = FinanceApiBuilder::random();
        let prices_repo = PricesRepoBuilder::in_memory();

        let price_provider =
            PriceProviderBuilder::default(finance_api, prices_repo, Duration::days(1));

        let rebalancer_svc = RebalancerSvcBuilder::default(price_provider).pipe(Arc::new);

        let api_key_mapper = ApiKeyMapperBuilder::pickle()
            .pipe(Mutex::new)
            .pipe(Arc::new);

        State::MainLupa {
            state: MainLupaState {
                rebalancer_svc,
                api_key_mapper,
            },
        }
    }
}
