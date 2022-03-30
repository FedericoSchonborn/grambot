use std::env::var;

use anyhow::Result;
use grambot::{methods::SendMessage, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    let mut request = SendMessage::new(chat_id, "Hello, world!");
    request.disable_notification = Some(true);
    let message = bot.send(request).await?;
    println!("{message:#?}");

    let request = SendMessage::builder()
        .disable_notification(true)
        .build(chat_id, "Hello again, world!");
    let message = bot.send(request).await?;
    println!("{message:#?}");

    Ok(())
}
