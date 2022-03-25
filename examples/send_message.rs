use std::env::var;

use anyhow::Result;
use grambot::{bot::Bot, methods::SendMessage};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    // Create a SendMessage object.
    let mut params = SendMessage::new(chat_id, "Hello, world!");
    // We don't want to wake up anybody! ;)
    params.disable_notification = Some(true);
    // Send the message.
    let message = bot.send_message(params).await?;
    println!("{message:#?}");

    // You can also use the builder API.
    let message = bot
        .new_message(chat_id, "Hello again!")
        .disable_notification(true)
        .send()
        .await?;
    println!("{message:#?}");

    Ok(())
}
