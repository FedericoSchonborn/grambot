use std::env::var;

use anyhow::Result;
use grambot::{bot::Bot, methods::ParseMode};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let message = bot
        .new_message(
            var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?,
            "<b>Hello, world!</b>",
        )
        .parse_mode(ParseMode::Html)
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message.entities);

    Ok(())
}
