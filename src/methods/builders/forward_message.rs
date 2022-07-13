use crate::{
    methods::{ForwardMessage, Request},
    types::Target,
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct ForwardMessageBuilder<'bot> {
    bot: &'bot Bot,
    request: ForwardMessage,
}

impl<'bot> ForwardMessageBuilder<'bot> {
    pub fn new<T, F>(bot: &'bot Bot, target: T, from_target: F, message_id: i64) -> Self
    where
        T: Into<Target>,
        F: Into<Target>,
    {
        Self {
            bot,
            request: ForwardMessage::new(target, from_target, message_id),
        }
    }

    pub fn disable_notification(&mut self, value: bool) -> &mut Self {
        self.request.disable_notification = Some(value);
        self
    }

    pub fn protect_content(&mut self, value: bool) -> &mut Self {
        self.request.protect_content = Some(value);
        self
    }

    pub async fn send(&self) -> Result<<ForwardMessage as Request>::Output, Error> {
        self.bot.send(&self.request).await
    }
}
