use std::env;

use anyhow::Result;
use grambot::{bot::Bot, methods::SendMessage};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let token = env::var("GRAMBOT_EXAMPLE_TOKEN")?;
    let chat_id = env::var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    let bot = Bot::new(token);

    // Create a SendMessage object.
    let mut params = SendMessage::new(chat_id, "Hello, world!");
    // We don't want to wake up anybody! ;)
    params.disable_notification = Some(true);
    // Send the message.
    bot.send_message(params).await?;

    // You can also use the builder API.
    bot.new_message(chat_id, "Hello again!")
        .disable_notification(true)
        .send()
        .await?;

    Ok(())
}