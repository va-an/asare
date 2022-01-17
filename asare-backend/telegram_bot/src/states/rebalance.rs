use domain::{
    price_provider::{
        api_provider_builder::ApiProviderBuilder, price_provider_builder::PriceProviderBuilder,
        repository_builder::PricesRepoBuilder,
    },
    rebalancer::service_builder::RebalancerSvcBuilder,
    utils::ChainingExt,
};
use teloxide::prelude::*;

use crate::{conversions::BotController, states::RebalanceDialogue};

#[derive(Clone)]
pub struct RebalanceState;

#[teloxide(subtransition)]
async fn receive_input(
    _state: RebalanceState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match BotController::from_input(&input) {
        Ok(rebalance_input) => {
            let api_provider = ApiProviderBuilder::mock();
            let prices_repo = PricesRepoBuilder::in_memory();
            let price_provider = PriceProviderBuilder::default(api_provider, prices_repo);

            RebalancerSvcBuilder::default(price_provider)
                .rebalance(&rebalance_input)
                .pipe(|output| BotController::from_output(&output))
                .pipe(|formatted_output| cx.answer(formatted_output))
                .await?;

            exit()
        }

        Err(_) => {
            cx.answer("Can't parse rebalance input").await?;
            exit()
        }
    }
}
