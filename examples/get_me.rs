use std::env::var;

use anyhow::Result;
use grambot::Bot;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    println!("{:#?}", bot.get_me().await);

    Ok(())
}
