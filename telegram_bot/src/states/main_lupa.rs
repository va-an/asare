// use crate::{
//     api_client,
//     api_key_mapper::mapper::UserId,
//     resources::{ABOUT_MESSAGE, EXAMPLE_BY_AMOUNT_MESSAGE, EXAMPLE_BY_PRICE_MESSAGE, HELP_MESSAGE},
//     states::{MainLupaState, RebalanceByAmountState, RebalanceByPriceState, RebalanceDialogue},
// };
// use domain::{users::User, utils::ChainingExt};
// use reqwest::StatusCode;
// use teloxide::prelude::*;

// fn find_api_key(user_id: UserId, main_lupa_state: &MainLupaState) -> Option<String> {
//     main_lupa_state
//         .api_key_mapper
//         .lock()
//         .unwrap()
//         .find_api_key(user_id)
// }

// async fn create_user_if_need(user_id: UserId, main_lupa_state: &MainLupaState) -> String {
//     let user_id_str = user_id.to_string();
//     let create_user_response = api_client::create_user(&user_id_str).await;

//     match create_user_response.status() {
//         StatusCode::CONFLICT => {
//             format!("user already exists")
//         }
//         StatusCode::OK => {
//             let user = create_user_response
//                 .json::<User>()
//                 .await
//                 .expect("error parsing user");

//             main_lupa_state
//                 .api_key_mapper
//                 .lock()
//                 .unwrap()
//                 .create(user_id, &user.api_key);

//             format!("user created\n\n{:#?}", user)
//         }
//         _ => "unknown response code".to_string(),
//     }
// }

// #[teloxide(subtransition)]
// async fn start(
//     main_lupa_state: MainLupaState,
//     cx: TransitionIn<AutoSend<Bot>>,
//     input: String,
// ) -> TransitionOut<RebalanceDialogue> {
//     match input.as_str() {
//         "/start" => {
//             let user_id: UserId = cx.update.from().expect("User not found").id;
//             let maybe_api_key = find_api_key(user_id, &main_lupa_state);

//             match maybe_api_key {
//                 Some(api_key) => {
//                     log::debug!("for user '{}' api key is '{}'", user_id, api_key);
//                 }
//                 None => {
//                     log::info!("not found api key for user '{}', creating user...", user_id);

//                     create_user_if_need(user_id, &main_lupa_state).await.tap(
//                         |create_user_result| {
//                             log::debug!("create user result:\n{}", create_user_result)
//                         },
//                     );
//                 }
//             }

//             cx.answer(HELP_MESSAGE).await?;
//             next(main_lupa_state)
//         }

//         "/rebalance_by_amount" => {
//             cx.answer("Enter your portfolio and desired allocation")
//                 .await?;

//             next(RebalanceByAmountState {
//                 rebalancer_svc: main_lupa_state.rebalancer_svc,
//             })
//         }

//         "/rebalance_by_price" => {
//             cx.answer("Enter your portfolio and desired allocation")
//                 .await?;

//             next(RebalanceByPriceState {
//                 rebalancer_svc: main_lupa_state.rebalancer_svc,
//             })
//         }
//     }
// }

use teloxide::prelude::*;

use crate::{
    resources::{ABOUT_MESSAGE, EXAMPLE_BY_AMOUNT_MESSAGE, EXAMPLE_BY_PRICE_MESSAGE, HELP_MESSAGE},
    HandlerResult, MyDialogue, State,
};

use super::MainLupaState;

pub async fn start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    match msg.text() {
        Some(input) => match input {
            "/start" => {
                bot.send_message(msg.chat.id, HELP_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            "/rebalance_by_amount" => {
                bot.send_message(msg.chat.id, "Enter your portfolio and desired allocation")
                    .await?;

                dialogue.update(State::RebalanceByAmount { state }).await?;
            }

            "/rebalance_by_price" => {
                bot.send_message(msg.chat.id, "not implemented").await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            "/portfolios" => {
                bot.send_message(msg.chat.id, "in progress").await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            "/example" => {
                bot.send_message(msg.chat.id, EXAMPLE_BY_AMOUNT_MESSAGE)
                    .await?;

                bot.send_message(msg.chat.id, EXAMPLE_BY_PRICE_MESSAGE)
                    .await?;

                dialogue.update(State::MainLupa { state }).await?;
            }

            "/help" => {
                bot.send_message(msg.chat.id, HELP_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            "/about" => {
                bot.send_message(msg.chat.id, ABOUT_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            _ => {
                bot.send_message(
                    msg.chat.id,
                    "Sorry, I don't understand this command.\nMaybe you need /help?",
                )
                .await?;
            }
        },
        None => {
            bot.send_message(msg.chat.id, "Hmmm").await?;
        }
    };

    Ok(())
}

async fn receive_full_name(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "How old are you?").await?;
            dialogue
                .update(State::ReceiveAge {
                    full_name: text.into(),
                })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn receive_age(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    full_name: String, // Available from `State::ReceiveAge`.
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(age)) => {
            bot.send_message(msg.chat.id, "What's your location?")
                .await?;

            dialogue
                .update(State::ReceiveLocation { full_name, age })
                .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}

async fn receive_location(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
) -> HandlerResult {
    match msg.text() {
        Some(location) => {
            let message = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
            bot.send_message(msg.chat.id, message).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
