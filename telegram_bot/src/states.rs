use std::sync::Arc;

use chrono::Duration;
use derive_more::From;
use domain::{
    price_provider::{
        finance_api_builder::FinanceApiBuilder, price_provider_builder::PriceProviderBuilder,
        repository_builder::PricesRepoBuilder,
    },
    rebalancer::{service::RebalancerSvcType, service_builder::RebalancerSvcBuilder},
    utils::ChainingExt,
};
use teloxide::macros::Transition;

pub mod main_lupa;
pub mod rebalance;

#[derive(Clone)]
pub struct MainLupaState {
    pub rebalancer_svc: Arc<RebalancerSvcType>,
}

#[derive(Clone)]
pub struct RebalanceByAmountState {
    pub rebalancer_svc: Arc<RebalancerSvcType>,
}

#[derive(Clone)]
pub struct RebalanceByPriceState {
    pub rebalancer_svc: Arc<RebalancerSvcType>,
}

#[derive(Transition, Clone, From)]
pub enum RebalanceDialogue {
    MainLupa(MainLupaState),
    RebalanceByAmount(RebalanceByAmountState),
    RebalanceByPrice(RebalanceByPriceState),
}

impl Default for RebalanceDialogue {
    fn default() -> Self {
        let finance_api = FinanceApiBuilder::random();
        let prices_repo = PricesRepoBuilder::in_memory();
        let price_provider =
            PriceProviderBuilder::default(finance_api, prices_repo, Duration::days(1));
        let rebalancer_svc = RebalancerSvcBuilder::default(price_provider).pipe(Arc::new);

        Self::MainLupa(MainLupaState { rebalancer_svc })
    }
}
