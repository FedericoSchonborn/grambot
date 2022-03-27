//! Available types.

use chrono::NaiveDateTime;
use serde::Deserialize;

mod allowed_update;
mod chat;
mod chat_id;
mod dice;
pub mod errors;
mod force_reply;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod keyboard_button;
mod message_entity;
mod message_id;
mod parse_mode;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
mod response;
mod update;
mod user;

pub use allowed_update::*;
pub use chat::*;
pub use chat_id::*;
pub use dice::*;
pub use force_reply::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_markup::*;
pub use keyboard_button::*;
pub use message_entity::*;
pub use message_id::*;
pub use parse_mode::*;
pub use reply_keyboard_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use response::*;
pub use update::*;
pub use user::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: i64,
    pub retry_after: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub date: NaiveDateTime,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub forward_date: NaiveDateTime,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub edit_date: NaiveDateTime,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    // TODO: Media messages.
    pub caption: Option<String>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub dice: Option<Dice>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    // TODO: Service messages.
    pub migrate_to_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    // TODO: Media messages.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
