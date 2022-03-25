use std::env::var;

use anyhow::Result;
use grambot::{methods::types::AllowedUpdate, Bot};
use maplit::hashset;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    println!(
        "{:#?}",
        bot.new_get_updates()
            // Only get updates with the "message" type.
            .allowed_updates(hashset![AllowedUpdate::Message])
            .send()
            .await?
    );

    Ok(())
}
