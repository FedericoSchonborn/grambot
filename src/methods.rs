//! Method parameters.

use serde::Serialize;

use crate::{
    methods::types::{AllowedUpdate, ChatId, ParseMode},
    types::MessageEntity,
};

pub mod builders;
pub mod types;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Default)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl GetUpdates {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendMessage {
    pub chat_id: ChatId,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    // TODO: reply_markup
}

impl SendMessage {
    pub fn new<C, T>(chat_id: C, text: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
        }
    }
}
