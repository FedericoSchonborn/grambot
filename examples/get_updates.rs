use std::env::var;

use anyhow::Result;
use grambot::{types::AllowedUpdate, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    println!(
        "{:#?}",
        bot.get_updates()
            // Only get updates with the "message" type.
            .allowed_updates(vec![AllowedUpdate::Message])
            .send()
            .await?
    );

    Ok(())
}
