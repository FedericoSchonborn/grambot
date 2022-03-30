use hyper::Method;
use serde::Serialize;

use crate::{
    builders::ForwardMessageBuilder,
    traits::Request,
    types::{ChatId, Message},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ForwardMessage {
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl ForwardMessage {
    pub fn new<C, F>(chat_id: C, from_chat_id: F, message_id: i64) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
            protect_content: None,
        }
    }

    #[must_use]
    pub fn builder() -> ForwardMessageBuilder {
        ForwardMessageBuilder::new()
    }
}

impl Request for ForwardMessage {
    const NAME: &'static str = "forwardMessage";
    const METHOD: Method = Method::POST;
    type Output = Message;
}
