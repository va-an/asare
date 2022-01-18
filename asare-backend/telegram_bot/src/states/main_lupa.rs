use crate::states::{
    MainLupaState, RebalanceByAmountState, RebalanceByPriceState, RebalanceDialogue,
};
use teloxide::prelude::*;

static HELP_MESSAGE: &str = concat!(
    "Bot for calculation asset allocation rebalance.\n",
    "Commands:\n\n",
    "/rebalance - calculate rebalance\n",
    "/example - example input message for /rebalance\n",
    "/help - help\n",
    "/about - for feature request and bug reports"
);

pub static EXAMPLE_AMOUNT_INPUT: &str = concat!("A 75000 33\n", "B 100000 33\n", "C 125000 34\n");
pub static EXAMPLE_PRICE_INPUT: &str = concat!("A 111.0 33\n", "B 56.0 33\n", "C 150.0 34\n");

static ABOUT_MESSAGE: &str = concat!(
    "This bot created with love and open source \n",
    "Code here - https://github.com/va-anufriev/asare \n",
    "Feel free create a issues with feature or bugfix requests!"
);

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

        // TODO: new example state with variant by_amount/by_price
        "/example" => {
            cx.answer(concat!(
                "Input format:",
                "<ticker> <current amount> <requered allocation>"
            ))
            .await?;

            cx.answer(EXAMPLE_AMOUNT_INPUT).await?;

            next(main_lupa_state)
        }

        // FIXME: fix info about rebalance commands
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
