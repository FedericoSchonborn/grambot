use std::env::var;

use anyhow::Result;
use grambot::{methods::GetMe, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // You can use your local Bot API server by using the Bot builder API.
    let bot = Bot::builder(var("GRAMBOT_EXAMPLE_TOKEN")?)
        .host(var("GRAMBOT_EXAMPLE_HOST")?)
        .build();
    println!("{:#?}", bot.send(GetMe).await);

    Ok(())
}
