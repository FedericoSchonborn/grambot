use std::env::var;

use anyhow::Result;
use grambot::{methods::SendDice, types::DiceEmoji, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    let request = SendDice::builder()
        .emoji(DiceEmoji::Basketball)
        .disable_notification(true)
        .build(chat_id);
    let message = bot.send(request).await?;
    println!("{message:#?}");

    Ok(())
}
