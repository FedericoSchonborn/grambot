//! Available types.

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
    // TODO: sender_chat
    // TODO: date
    pub chat: Chat,
    pub forward_from: Option<User>,
    // TODO: forward_from_chat
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    // TODO: forward_date
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    // TODO: edit_date
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    // TODO
    pub dice: Option<Dice>,
    // TODO
}
