use states::RebalanceDialogue;
use teloxide::{prelude::*, types::BotCommand};

use crate::resources::{
    ABOUT_COMMAND, EXAMPLE_COMMAND, HELP_COMMAND, REBALANCE_BY_AMOUNT_COMMAND,
    REBALANCE_BY_PRICE_COMMAND,
};

mod api_client;
mod api_key_mapper;
mod conversions;
mod resources;
mod states;

async fn rebalance_dialogue_handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: RebalanceDialogue,
) -> TransitionOut<RebalanceDialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}

// TODO: get from env var ASARE_BOT_TOKEN instead of TELOXIDE_TOKEN
#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    let commands = vec![
        BotCommand::new(REBALANCE_BY_AMOUNT_COMMAND, "rebalance by amount"),
        BotCommand::new(REBALANCE_BY_PRICE_COMMAND, "rebalance by price"),
        BotCommand::new(EXAMPLE_COMMAND, "show example"),
        BotCommand::new(ABOUT_COMMAND, "source code"),
        BotCommand::new(HELP_COMMAND, "show help page"),
    ];

    bot.set_my_commands(commands).send().await.unwrap();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        rebalance_dialogue_handler(message, dialogue)
            .await
            .expect("Something wrong with the bot!")
    })
    .await;
}
