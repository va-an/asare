use domain::{users::User, utils::ChainingExt};
use reqwest::StatusCode;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::{
    api_client,
    resources::{ABOUT_MESSAGE, EXAMPLE_BY_AMOUNT_MESSAGE, EXAMPLE_BY_PRICE_MESSAGE, HELP_MESSAGE},
    Command, HandlerResult, MyDialogue, State,
};

use super::MainLupaState;

fn find_api_key(user_id: u64, main_lupa_state: &MainLupaState) -> Option<String> {
    main_lupa_state
        .api_key_mapper
        .lock()
        .unwrap()
        .find_api_key(user_id)
}

async fn create_user_if_need(user_id: u64, main_lupa_state: &MainLupaState) -> String {
    let user_id_str = user_id.to_string();
    let create_user_response = api_client::create_user(&user_id_str).await;

    match create_user_response.status() {
        StatusCode::CONFLICT => {
            format!("user already exists")
        }
        StatusCode::OK => {
            let user = create_user_response
                .json::<User>()
                .await
                .expect("error parsing user");

            main_lupa_state
                .api_key_mapper
                .lock()
                .unwrap()
                .create(user_id, &user.api_key);

            format!("user created\n\n{:#?}", user)
        }
        _ => "unknown response code".to_string(),
    }
}

pub async fn start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    state: MainLupaState,
) -> HandlerResult {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, "asare bot") {
            Ok(Command::Start) => {
                let user_id: u64 = msg.from().expect("User not found").id.0;
                let maybe_api_key = find_api_key(user_id, &state);

                match maybe_api_key {
                    Some(api_key) => {
                        log::debug!("for user '{}' api key is '{}'", user_id, api_key);
                    }
                    None => {
                        log::info!("not found api key for user '{}', creating user...", user_id);

                        create_user_if_need(user_id, &state)
                            .await
                            .tap(|create_user_result| {
                                log::debug!("create user result:\n{}", create_user_result)
                            });
                    }
                }

                bot.send_message(msg.chat.id, HELP_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            Ok(Command::RebalanceByAmount) => {
                bot.send_message(msg.chat.id, "Enter your portfolio and desired allocation")
                    .await?;

                dialogue.update(State::RebalanceByAmount { state }).await?;
            }

            Ok(Command::RebalanceByPrice) => {
                bot.send_message(msg.chat.id, "Enter your portfolio and desired allocation")
                    .await?;

                dialogue.update(State::RebalanceByPrice { state }).await?;
            }

            Ok(Command::Portfolios) => {
                bot.send_message(msg.chat.id, "in progress").await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            Ok(Command::Example) => {
                bot.send_message(msg.chat.id, EXAMPLE_BY_AMOUNT_MESSAGE)
                    .await?;

                bot.send_message(msg.chat.id, EXAMPLE_BY_PRICE_MESSAGE)
                    .await?;

                dialogue.update(State::MainLupa { state }).await?;
            }

            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, HELP_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            Ok(Command::About) => {
                bot.send_message(msg.chat.id, ABOUT_MESSAGE).await?;
                dialogue.update(State::MainLupa { state }).await?;
            }

            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    "Sorry, I don't understand this command.\nMaybe you need /help?",
                )
                .await?;
            }
        }
    }

    Ok(())
}
