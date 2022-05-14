use std::env::var;

use anyhow::Result;
use grambot::{types::DiceEmoji, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let target = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    let message = bot
        .dice(target)
        .emoji(DiceEmoji::Basketball)
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message.dice());

    Ok(())
}
