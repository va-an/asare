use crate::{
    resources::{ABOUT_MESSAGE, EXAMPLE_BY_AMOUNT_MESSAGE, EXAMPLE_BY_PRICE_MESSAGE, HELP_MESSAGE},
    states::{MainLupaState, RebalanceByAmountState, RebalanceByPriceState, RebalanceDialogue},
};
use teloxide::prelude::*;

#[teloxide(subtransition)]
async fn start(
    main_lupa_state: MainLupaState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match input.as_str() {
        "/start" => {
            cx.answer(HELP_MESSAGE).await?;
            next(main_lupa_state)
        }

        "/rebalance_by_amount" => {
            cx.answer("Enter your portfolio and desired allocation")
                .await?;

            next(RebalanceByAmountState {
                rebalancer_svc: main_lupa_state.rebalancer_svc,
            })
        }

        "/rebalance_by_price" => {
            cx.answer("Enter your portfolio and desired allocation")
                .await?;

            next(RebalanceByPriceState {
                rebalancer_svc: main_lupa_state.rebalancer_svc,
            })
        }

        "/example" => {
            cx.answer(EXAMPLE_BY_AMOUNT_MESSAGE).await?;
            cx.answer(EXAMPLE_BY_PRICE_MESSAGE).await?;

            next(main_lupa_state)
        }

        "/help" => {
            cx.answer(HELP_MESSAGE).await?;
            next(main_lupa_state)
        }

        "/about" => {
            cx.answer(ABOUT_MESSAGE).await?;
            next(main_lupa_state)
        }

        _ => {
            cx.answer("Sorry, I don't understand this command.\nMaybe you need /help?")
                .await?;
            next(main_lupa_state)
        }
    }
}
