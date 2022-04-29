use std::env::var;

use anyhow::Result;
use grambot::{methods::GetMe, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // You can use your local Bot API server by using the Bot builder API.
    let bot = Bot::builder()
        .host(var("GRAMBOT_EXAMPLE_LOCAL_SERVER")?)
        .build(var("GRAMBOT_EXAMPLE_TOKEN")?);
    println!("{:#?}", bot.send(GetMe).await);

    Ok(())
}
