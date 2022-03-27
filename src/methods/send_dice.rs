use hyper::Method;
use serde::Serialize;

use crate::{
    types::{ChatId, DiceEmoji, Message, ReplyMarkup},
    Bot, Error,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendDice<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<DiceEmoji>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'bot> SendDice<'bot> {
    pub fn new<C>(bot: &'bot Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn emoji(mut self, value: DiceEmoji) -> Self {
        self.emoji = Some(value);
        self
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

    #[must_use]
    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
    }

    #[must_use]
    pub fn reply_markup<M>(mut self, value: M) -> Self
    where
        M: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Message, Error> {
        self.bot.request(Method::POST, "sendDice", self).await
    }
}
