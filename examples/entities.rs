use std::env::var;

use anyhow::Result;
use grambot::{methods::SendMessage, types::ParseMode, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    let request = SendMessage::builder()
        .parse_mode(ParseMode::MarkdownV2)
        .disable_notification(true)
        .build(chat_id, "*Hello, world\\!*");
    let message = bot.send(request).await?;
    println!("{:#?}", message.entities());

    Ok(())
}
