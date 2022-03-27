use hyper::Method;
use serde::Serialize;

use crate::{
    types::{ChatId, Message},
    Bot, Error,
};

#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessage<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
    chat_id: ChatId,
    from_chat_id: ChatId,
    message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
}

impl<'bot> ForwardMessage<'bot> {
    pub fn new<C, F>(bot: &'bot Bot, chat_id: C, from_chat_id: F, message_id: i32) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
            protect_content: None,
        }
    }

    #[must_use]
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Message, Error> {
        self.bot.request(Method::POST, "forwardMessage", self).await
    }
}
