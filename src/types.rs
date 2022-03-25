//! Types returned from methods.

use serde::{Deserialize, Serialize};

mod response;
mod update;

pub use response::*;
pub use update::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: i64,
    pub retry_after: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    // TODO: sender_chat
    // TODO: date
    // TODO: chat
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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,
    pub offset: i32,
    pub length: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Spoiler,
    Code,
    Pre { language: Option<String> },
    TextLink { url: String },
    TextMention { user: User },
}
