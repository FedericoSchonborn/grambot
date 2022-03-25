use std::env::var;

use anyhow::Result;
use grambot::{bot::Bot, methods::AllowedUpdate};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    println!(
        "{:#?}",
        bot.new_get_updates()
            // Only get new messages.
            .allowed_updates(vec![AllowedUpdate::Message])
            .send()
            .await?
    );

    Ok(())
}
