use std::env::var;

use anyhow::Result;
use grambot::{methods::SendMessage, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let target = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    let mut request = SendMessage::new(target, "Hello, world!");
    request.disable_notification = Some(true);
    let message = bot.send(request).await?;

    println!("{:#?}", message.text());

    // Or:

    let message = bot
        .message(target, "Hello again, world!")
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message.text());

    Ok(())
}
