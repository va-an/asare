use std::sync::{Arc, Mutex};

use api_key_mapper::{mapper::ApiKeyMapperType, mapper_builder::ApiKeyMapperBuilder};
use chrono::Duration;
use domain::{
    price_provider::{
        finance_api_builder::FinanceApiBuilder, price_provider_builder::PriceProviderBuilder,
        repository_builder::PricesRepoBuilder,
    },
    rebalancer::{service::RebalancerSvcType, service_builder::RebalancerSvcBuilder},
    utils::ChainingExt,
};
use states::MainLupaState;
// use states::RebalanceDialogue;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::BotCommand};

use crate::{
    resources::{
        ABOUT_COMMAND, EXAMPLE_COMMAND, HELP_COMMAND, PORTFOLIOS_COMMAND,
        REBALANCE_BY_AMOUNT_COMMAND, REBALANCE_BY_PRICE_COMMAND,
    },
    states::{main_lupa::start, rebalance::rebalance_by_amount},
};

mod api_client;
mod api_key_mapper;
mod conversions;
mod resources;
mod states;

// async fn rebalance_dialogue_handler(
//     cx: UpdateWithCx<AutoSend<Bot>, Message>,
//     dialogue: RebalanceDialogue,
// ) -> TransitionOut<RebalanceDialogue> {
//     match cx.update.text().map(ToOwned::to_owned) {
//         None => {
//             cx.answer("Send me a text message.").await?;
//             next(dialogue)
//         }
//         Some(ans) => dialogue.react(cx, ans).await,
//     }
// }

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub enum State {
    MainLupa { state: MainLupaState },
    RebalanceByAmount { state: MainLupaState },
    ReceiveFullName,
    ReceiveAge { full_name: String },
    ReceiveLocation { full_name: String, age: u8 },
}

impl Default for State {
    fn default() -> Self {
        let finance_api = FinanceApiBuilder::random();
        let prices_repo = PricesRepoBuilder::in_memory();

        let price_provider =
            PriceProviderBuilder::default(finance_api, prices_repo, Duration::days(1));

        let rebalancer_svc = RebalancerSvcBuilder::default(price_provider).pipe(Arc::new);

        let api_key_mapper = ApiKeyMapperBuilder::pickle()
            .pipe(Mutex::new)
            .pipe(Arc::new);

        State::MainLupa {
            state: MainLupaState {
                rebalancer_svc,
                api_key_mapper,
            },
        }
    }
}

// TODO: get from env var ASARE_BOT_TOKEN instead of TELOXIDE_TOKEN
#[tokio::main]
async fn main() {
    // teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let commands = vec![
        BotCommand::new(REBALANCE_BY_AMOUNT_COMMAND, "rebalance by amount"),
        BotCommand::new(REBALANCE_BY_PRICE_COMMAND, "rebalance by price"),
        BotCommand::new(PORTFOLIOS_COMMAND, "manage portfolios"),
        BotCommand::new(EXAMPLE_COMMAND, "show example"),
        BotCommand::new(ABOUT_COMMAND, "source code"),
        BotCommand::new(HELP_COMMAND, "show help page"),
    ];

    bot.set_my_commands(commands).send().await.unwrap();

    // teloxide::dialogues_repl(bot, |message, dialogue| async move {
    //     rebalance_dialogue_handler(message, dialogue)
    //         .await
    //         .expect("Something wrong with the bot!")
    // })
    // .await;

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::MainLupa { state }].endpoint(start))
            .branch(dptree::case![State::RebalanceByAmount { state }].endpoint(rebalance_by_amount))
            .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(dptree::case![State::ReceiveAge { full_name }].endpoint(receive_age))
            .branch(
                dptree::case![State::ReceiveLocation { full_name, age }].endpoint(receive_location),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
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
