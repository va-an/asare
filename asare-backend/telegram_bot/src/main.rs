use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Bot for calculation asset allocation rebalance.\nThese commands are supported:\n"
)]
enum Command {
    #[command(description = "calculate rebalance")]
    Rebalance,

    #[command(description = "show input example")]
    Example,

    #[command(description = "show all commands")]
    Help,

    #[command(description = "about this bot")]
    About,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Rebalance => todo!(),

        Command::Example => {
            cx.answer(concat!(
                "Input format:",
                "<ticker> <current amount> <requered allocation>"
            ))
            .await?;

            cx.answer(concat!("A 75000 33\n", "B 100000 33\n", "C 125000 34\n"))
                .await?
        }

        Command::About => {
            cx.answer(concat!(
                "This bot created with love and open source \n",
                "Code here - https://github.com/va-anufriev/asare \n",
                "Feel free create a issues with feature or bugfix requests!"
            ))
            .await?
        }

        Command::Help => cx.answer(Command::descriptions()).await?,
    };

    Ok(())
}

// TODO: get from env var ASARE_BOT_TOKEN instead of TELOXIDE_TOKEN
#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "asare bot".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
