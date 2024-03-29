use std::env;

use states::rebalance::rebalance_by_price;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*, utils::command::BotCommands};

use crate::states::{main_lupa::start, rebalance::rebalance_by_amount, State};

mod api_client;
mod api_key_mapper;
mod conversions;
mod resources;
mod states;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

const ASARE_BOT_TOKEN: &str = "ASARE_BOT_TOKEN";

#[derive(BotCommands)]
#[command(
    rename_rule = "snake_case",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "start bot")]
    Start,

    #[command(description = "rebalance by amount")]
    RebalanceByAmount,

    #[command(description = "rebalance by price")]
    RebalanceByPrice,

    #[command(description = "manage portfolios")]
    Portfolios,

    #[command(description = "show example")]
    Example,

    #[command(description = "source code")]
    About,

    #[command(description = "show help page")]
    Help,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let token = env::var(ASARE_BOT_TOKEN)
        .unwrap_or_else(|_| panic!("{} env variable missing", ASARE_BOT_TOKEN));

    let bot = Bot::new(token);

    bot.set_my_commands(Command::bot_commands())
        .send()
        .await
        .unwrap();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::MainLupa { state }].endpoint(start))
            .branch(dptree::case![State::RebalanceByAmount { state }].endpoint(rebalance_by_amount))
            .branch(dptree::case![State::RebalanceByPrice { state }].endpoint(rebalance_by_price)),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;
}
