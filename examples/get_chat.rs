use std::env::var;

use anyhow::Result;
use grambot::{methods::GetChat, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let target = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;
    println!("{:#?}", bot.send(GetChat::new(target)).await?);

    Ok(())
}
