use std::env::var;

use anyhow::Result;
use grambot::{types::ChatAction, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    bot.new_chat_action(chat_id, ChatAction::Typing)
        .send()
        .await?;
    let message = bot
        .new_message(chat_id, "Hi!")
        .disable_notification(true)
        .send()
        .await?;
    println!("{message:#?}");

    Ok(())
}
