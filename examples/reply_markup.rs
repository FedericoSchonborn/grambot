use std::env::var;

use anyhow::Result;
use grambot::{
    types::{KeyboardButton, ReplyKeyboardMarkup},
    Bot,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_CHATID")?.parse::<i64>()?;

    let keyboard = ReplyKeyboardMarkup {
        keyboard: vec![
            vec![KeyboardButton::new("Okay!"), KeyboardButton::new("Nope!")],
            vec![
                KeyboardButton::new("Maybe?"),
                KeyboardButton::new("Ask later."),
            ],
        ],
    };

    let message = bot
        .new_message(chat_id, "Do you want to play?")
        .reply_markup(keyboard)
        .disable_notification(true)
        .send()
        .await?;
    println!("{message:#?}");

    Ok(())
}
