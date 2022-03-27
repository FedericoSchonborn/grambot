use std::env::var;

use anyhow::Result;
use grambot::Bot;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    let message = bot
        .new_dice(chat_id)
        .disable_notification(true)
        .send()
        .await?;
    println!("{message:#?}");

    Ok(())
}
