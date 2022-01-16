use domain::{rebalancer::service_builder::RebalancerSvcBuilder, utils::ChainingExt};
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
            let output = RebalancerSvcBuilder::default()
                .rebalance(&rebalance_input)
                .pipe(|output| BotController::from_output(&output));

            cx.answer(output).await?;

            exit()
        }

        Err(_) => {
            cx.answer("Can't parse rebalance input").await?;
            exit()
        }
    }
}
