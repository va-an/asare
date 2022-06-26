// use domain::utils::ChainingExt;
// use teloxide::prelude::*;

// use crate::{
//     conversions::BotController,
//     states::{RebalanceByAmountState, RebalanceDialogue},
// };

// use super::RebalanceByPriceState;

// #[teloxide(subtransition)]
// async fn rebalance_by_amount(
//     state: RebalanceByAmountState,
//     cx: TransitionIn<AutoSend<Bot>>,
//     input: String,
// ) -> TransitionOut<RebalanceDialogue> {
//     match BotController::from_input(&input) {
//         Ok(rebalance_input) => {
//             state
//                 .rebalancer_svc
//                 .rebalance_by_amount(&rebalance_input)
//                 .pipe(|output| BotController::from_output(&output))
//                 .pipe(|formatted_output| cx.answer(formatted_output))
//                 .await?;

//             exit()
//         }

//         Err(_) => {
//             cx.answer("Can't parse rebalance input").await?;
//             exit()
//         }
//     }
// }

// #[teloxide(subtransition)]
// async fn rebalance_by_price(
//     state: RebalanceByPriceState,
//     cx: TransitionIn<AutoSend<Bot>>,
//     input: String,
// ) -> TransitionOut<RebalanceDialogue> {
//     match BotController::from_input(&input) {
//         Ok(rebalance_input) => {
//             state
//                 .rebalancer_svc
//                 .rebalance_by_price(&rebalance_input)
//                 .pipe(|output| BotController::from_output(&output))
//                 .pipe(|formatted_output| cx.answer(formatted_output))
//                 .await?;

//             exit()
//         }

//         Err(_) => {
//             cx.answer("Can't parse rebalance input").await?;
//             exit()
//         }
//     }
// }

use teloxide::prelude::*;

use crate::{HandlerResult, MyDialogue, State};

use super::MainLupaState;

pub async fn rebalance_by_amount(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "rebalance_by_amount").await?;
    dialogue.update(State::MainLupa { state }).await?;

    Ok(())
}

pub async fn rebalance_by_price(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    bot.send_message(msg.chat.id, "rebalance_by_price").await?;
    dialogue.update(State::MainLupa { state }).await?;

    Ok(())
}
