use std::env;

use states::rebalance::rebalance_by_price;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, types::BotCommand};

use crate::{
    resources::{
        ABOUT_COMMAND, EXAMPLE_COMMAND, HELP_COMMAND, PORTFOLIOS_COMMAND,
        REBALANCE_BY_AMOUNT_COMMAND, REBALANCE_BY_PRICE_COMMAND,
    },
    states::{main_lupa::start, rebalance::rebalance_by_amount, State},
};

mod api_client;
mod api_key_mapper;
mod conversions;
mod resources;
mod states;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const ASARE_BOT_TOKEN: &str = "ASARE_BOT_TOKEN";

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let token = env::var(ASARE_BOT_TOKEN)
        .expect(format!("{} env variable missing", ASARE_BOT_TOKEN).as_str());

    let bot = Bot::new(token).auto_send();

    let commands = vec![
        BotCommand::new(REBALANCE_BY_AMOUNT_COMMAND, "rebalance by amount"),
        BotCommand::new(REBALANCE_BY_PRICE_COMMAND, "rebalance by price"),
        BotCommand::new(PORTFOLIOS_COMMAND, "manage portfolios"),
        BotCommand::new(EXAMPLE_COMMAND, "show example"),
        BotCommand::new(ABOUT_COMMAND, "source code"),
        BotCommand::new(HELP_COMMAND, "show help page"),
    ];

    bot.set_my_commands(commands).send().await.unwrap();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::MainLupa { state }].endpoint(start))
            .branch(dptree::case![State::RebalanceByAmount { state }].endpoint(rebalance_by_amount))
            .branch(dptree::case![State::RebalanceByPrice { state }].endpoint(rebalance_by_price)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
}
