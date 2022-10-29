use domain::utils::ChainingExt;
use teloxide::prelude::*;

use crate::{conversions::BotController, HandlerResult, MyDialogue, State};

use super::MainLupaState;

pub async fn rebalance_by_amount(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    match msg.text() {
        Some(input) => match BotController::from_input(input) {
            Ok(rebalance_input) => {
                state
                    .rebalancer_svc
                    .rebalance_by_amount(&rebalance_input)
                    .pipe(|output| BotController::from_output(&output))
                    .pipe(|formatted_output| bot.send_message(msg.chat.id, formatted_output))
                    .await?;

                dialogue.update(State::MainLupa { state }).await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Can't parse rebalance input")
                    .await?;
                dialogue.update(State::MainLupa { state }).await?;
            }
        },
        None => {
            dialogue.update(State::MainLupa { state }).await?;
        }
    }

    Ok(())
}

pub async fn rebalance_by_price(
    bot: Bot,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    match msg.text() {
        Some(input) => match BotController::from_input(input) {
            Ok(rebalance_input) => {
                state
                    .rebalancer_svc
                    .rebalance_by_price(&rebalance_input)
                    .pipe(|output| BotController::from_output(&output))
                    .pipe(|formatted_output| bot.send_message(msg.chat.id, formatted_output))
                    .await?;

                dialogue.update(State::MainLupa { state }).await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Can't parse rebalance input")
                    .await?;
                dialogue.update(State::MainLupa { state }).await?;
            }
        },
        None => {
            dialogue.update(State::MainLupa { state }).await?;
        }
    }

    Ok(())
}
