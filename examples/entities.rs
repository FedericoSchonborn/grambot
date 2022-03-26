use std::env::var;

use anyhow::Result;
use grambot::{methods::types::ParseMode, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let message = bot
        .new_message(
            var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?,
            "*Hello, world\\!*",
        )
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message.entities);

    Ok(())
}
