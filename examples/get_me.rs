use std::env;

use anyhow::Result;
use grambot::bot::Bot;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let token = env::var("GRAMBOT_EXAMPLE_TOKEN")?;
    let bot = Bot::new(token);
    println!("{:#?}", bot.get_me().await);

    Ok(())
}
