//! Types returned from methods.

use serde::Deserialize;
use thiserror::Error;

mod message_id;
mod response;
mod update;

pub use message_id::*;
pub use response::*;
pub use update::*;

use crate::shared::{MessageEntity, User};

/// Error type for errors thrown by the API.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("{description}")]
pub struct ResponseError {
    description: String,
    error_code: Option<i32>,
    parameters: Option<ResponseParameters>,
}

impl ResponseError {
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn error_code(&self) -> Option<i32> {
        self.error_code
    }

    #[must_use]
    pub fn parameters(&self) -> Option<ResponseParameters> {
        self.parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: i64,
    pub retry_after: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    // TODO
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub kind: ChatKind,
    // TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
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
}
