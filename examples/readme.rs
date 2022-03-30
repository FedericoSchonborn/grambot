use anyhow::Result;
use grambot::{
    methods::{GetMe, GetUpdates},
    Bot,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new("<TOKEN>");
    println!("{:#?}", bot.send(GetMe).await);

    let updates = bot.send(GetUpdates::default()).await?;
    println!("{:#?}", updates);

    Ok(())
}
