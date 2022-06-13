use crate::{
    api_client,
    api_key_mapper::mapper::UserId,
    resources::{ABOUT_MESSAGE, EXAMPLE_BY_AMOUNT_MESSAGE, EXAMPLE_BY_PRICE_MESSAGE, HELP_MESSAGE},
    states::{MainLupaState, RebalanceByAmountState, RebalanceByPriceState, RebalanceDialogue},
};
use domain::users::User;
use reqwest::{Response, StatusCode};
use teloxide::prelude::*;

fn find_api_key(user_id: UserId, main_lupa_state: &MainLupaState) -> Option<String> {
    main_lupa_state
        .api_key_mapper
        .lock()
        .unwrap()
        .find_api_key(user_id)
}

async fn create_user_if_need(
    user_id: UserId,
    create_user_response: Response,
    main_lupa_state: &MainLupaState,
) -> String {
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

            format!("created user\n{:#?}", user)
        }
        _ => "unknown response code".to_string(),
    }
}

#[teloxide(subtransition)]
async fn start(
    main_lupa_state: MainLupaState,
    cx: TransitionIn<AutoSend<Bot>>,
    input: String,
) -> TransitionOut<RebalanceDialogue> {
    match input.as_str() {
        "/start" => {
            let user_id: UserId = cx.update.from().expect("User not found").id;
            let maybe_api_key = find_api_key(user_id, &main_lupa_state);

            match maybe_api_key {
                Some(api_key) => {
                    cx.answer(format!("for user '{}' api key is '{}'", user_id, api_key))
                        .await?;
                }
                None => {
                    // TODO: log instead answer
                    cx.answer(format!(
                        "not found api key for user '{}', creating user...",
                        user_id
                    ))
                    .await?;

                    let user_id_str = user_id.to_string();
                    let create_user_response = api_client::create_user(&user_id_str).await;

                    let response_status = create_user_response.status();
                    cx.answer(response_status.to_string()).await?;

                    let create_user_result =
                        create_user_if_need(user_id, create_user_response, &main_lupa_state).await;

                    cx.answer(create_user_result).await?;
                }
            }

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
