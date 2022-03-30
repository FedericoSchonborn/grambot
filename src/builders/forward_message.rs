use hyper::Method;

use crate::{
    methods::ForwardMessage,
    types::{ChatId, Message},
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct ForwardMessageBuilder<'bot> {
    bot: &'bot Bot,
    inner: ForwardMessage,
}

impl<'bot> ForwardMessageBuilder<'bot> {
    pub fn new<C, F>(bot: &'bot Bot, chat_id: C, from_chat_id: F, message_id: i64) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        Self {
            bot,
            inner: ForwardMessage::new(chat_id, from_chat_id, message_id),
        }
    }

    #[must_use]
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.inner.disable_notification = Some(value);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, value: bool) -> Self {
        self.inner.protect_content = Some(value);
        self
    }

    pub async fn send(self) -> Result<Message, Error> {
        self.bot
            .request(Method::POST, "forwardMessage", self.inner)
            .await
    }
}
