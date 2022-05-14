use std::env::var;

use anyhow::Result;
use grambot::{
    methods::SendMessage,
    types::{KeyboardButton, ReplyKeyboardMarkup},
    Bot,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let bot = Bot::new(var("GRAMBOT_EXAMPLE_TOKEN")?);
    let chat_id = var("GRAMBOT_EXAMPLE_TARGET_CHAT")?.parse::<i64>()?;

    let keyboard = vec![
        vec![KeyboardButton::new("Okay!"), KeyboardButton::new("Nope!")],
        vec![
            KeyboardButton::new("Maybe?"),
            KeyboardButton::new("Ask later."),
        ],
    ]
    .into_iter()
    .collect::<ReplyKeyboardMarkup>();

    let request = SendMessage::builder()
        .reply_markup(keyboard)
        .disable_notification(true)
        .build(chat_id, "Do you want to play?");
    let message = bot.send(request).await?;
    println!("{message:#?}");

    Ok(())
}
