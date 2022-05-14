use std::env::var;

use anyhow::Result;
use grambot::{types::AllowedUpdate, Bot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);

    let updates = bot
        .get_updates()
        .allowed_updates(vec![AllowedUpdate::Message])
        .send()
        .await?;
    println!("{:#?}", updates);

    Ok(())
}
