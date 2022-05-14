use std::env::var;

use anyhow::Result;
use grambot::{
    types::{KeyboardButton, ReplyKeyboardMarkup},
    Bot,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let target = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    let keyboard = vec![
        vec![KeyboardButton::new("Okay!"), KeyboardButton::new("Nope!")],
        vec![
            KeyboardButton::new("Maybe?"),
            KeyboardButton::new("Ask later."),
        ],
    ]
    .into_iter()
    .collect::<ReplyKeyboardMarkup>();

    let message = bot
        .message(target, "Do you want to play?")
        .reply_markup(keyboard)
        .disable_notification(true)
        .send()
        .await?;
    println!("{:#?}", message);

    Ok(())
}
