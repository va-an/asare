use domain::{
    price_provider::{
        api_provider_builder::ApiProviderBuilder, price_provider_builder::PriceProviderBuilder,
        repository_builder::PricesRepoBuilder,
    },
    rebalancer::service_builder::RebalancerSvcBuilder,
    utils::ChainingExt,
};
use teloxide::prelude::*;

use crate::{
    conversions::BotController,
    states::{RebalanceByAmountState, RebalanceDialogue},
};

use super::RebalanceByPriceState;

#[teloxide(subtransition)]
async fn rebalance_by_amount(
    _state: RebalanceByAmountState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match BotController::from_input(&input) {
        Ok(rebalance_input) => {
            // FIXME: pass services instead of create on place
            let api_provider = ApiProviderBuilder::mock();
            let prices_repo = PricesRepoBuilder::in_memory();
            let price_provider = PriceProviderBuilder::default(api_provider, prices_repo);

            // FIXME: we don't need price_provider here
            RebalancerSvcBuilder::default(price_provider)
                .rebalance_by_amount(&rebalance_input)
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

#[teloxide(subtransition)]
async fn rebalance_by_price(
    _state: RebalanceByPriceState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match BotController::from_input(&input) {
        Ok(rebalance_input) => {
            let api_provider = ApiProviderBuilder::mock();
            let prices_repo = PricesRepoBuilder::in_memory();
            let price_provider = PriceProviderBuilder::default(api_provider, prices_repo);

            RebalancerSvcBuilder::default(price_provider)
                .rebalance_by_price(&rebalance_input)
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
