use crate::states::{rebalance::RebalanceState, RebalanceDialogue};
use teloxide::prelude::*;

static HELP_MESSAGE: &str = concat!(
    "Bot for calculation asset allocation rebalance.\n",
    "Commands:\n\n",
    "/rebalance - calculate rebalance\n",
    "/example - example input message for /rebalance\n",
    "/help - help\n",
    "/about - for feature request and bug reports"
);

pub static EXAMPLE_INPUT: &str = concat!("A 75000 33\n", "B 100000 33\n", "C 125000 34\n");

static ABOUT_MESSAGE: &str = concat!(
    "This bot created with love and open source \n",
    "Code here - https://github.com/va-anufriev/asare \n",
    "Feel free create a issues with feature or bugfix requests!"
);

#[derive(Clone)]
pub struct MainLupaState;

#[teloxide(subtransition)]
async fn start(
    _state: MainLupaState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match input.as_str() {
        "/start" => {
            cx.answer(HELP_MESSAGE).await?;
            next(MainLupaState)
        }

        "/rebalance" => {
            cx.answer("Enter your portfolio and desired allocation")
                .await?;
            next(RebalanceState)
        }

        "/example" => {
            cx.answer(concat!(
                "Input format:",
                "<ticker> <current amount> <requered allocation>"
            ))
            .await?;

            cx.answer(EXAMPLE_INPUT).await?;

            next(MainLupaState)
        }

        "/help" => {
            cx.answer(HELP_MESSAGE).await?;
            next(MainLupaState)
        }

        "/about" => {
            cx.answer(ABOUT_MESSAGE).await?;
            next(MainLupaState)
        }

        _ => {
            cx.answer("Sorry, I don't understand this command.\nMaybe you need /help?")
                .await?;
            next(MainLupaState)
        }
    }
}
