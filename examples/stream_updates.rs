use std::env::var;

use anyhow::Result;
use grambot::Bot;
use tokio_stream::StreamExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);

    while let Some(update) = bot.stream_updates().try_next().await? {
        println!("{:#?}", update);
    }

    Ok(())
}
