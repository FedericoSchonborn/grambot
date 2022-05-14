use std::env::var;

use anyhow::Result;
use grambot::{methods::SendChatAction, types::ChatAction, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let target = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    bot.send(SendChatAction::new(target, ChatAction::Typing))
        .await?;

    let message = bot
        .message(target, "Wow, that took me a long time to type for sure!")
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message);

    Ok(())
}
