use std::env::var;

use anyhow::Result;
use grambot::{methods::GetChat, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?.into();
    println!("{:#?}", bot.send(GetChat(chat_id)).await?);

    Ok(())
}
