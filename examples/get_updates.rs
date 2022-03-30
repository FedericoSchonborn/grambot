use std::env::var;

use anyhow::Result;
use grambot::{methods::GetUpdates, types::AllowedUpdate, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);

    let request = GetUpdates::builder()
        .allowed_updates(vec![AllowedUpdate::Message])
        .build();
    let updates = bot.send(request).await?;
    println!("{:#?}", updates);

    Ok(())
}
