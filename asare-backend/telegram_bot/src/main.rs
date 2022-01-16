use states::RebalanceDialogue;
use teloxide::prelude::*;

mod conversions;
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
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        rebalance_dialogue_handler(message, dialogue)
            .await
            .expect("Something wrong with the bot!")
    })
    .await;
}
