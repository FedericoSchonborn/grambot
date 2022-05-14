use std::env::var;

use anyhow::Result;
use grambot::{
    methods::{SendChatAction, SendMessage},
    types::ChatAction,
    Bot,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    bot.send(SendChatAction::new(chat_id, ChatAction::Typing))
        .await?;

    let request = SendMessage::builder()
        .disable_notification(true)
        .build(chat_id, "Hi!");
    let message = bot.send(request).await?;
    println!("{message:#?}");

    Ok(())
}
