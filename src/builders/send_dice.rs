use hyper::Method;

use crate::{
    methods::SendDice,
    types::{ChatId, DiceEmoji, Message, ReplyMarkup},
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct SendDiceBuilder<'bot> {
    bot: &'bot Bot,
    inner: SendDice,
}

impl<'bot> SendDiceBuilder<'bot> {
    pub fn new<C>(bot: &'bot Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            inner: SendDice::new(chat_id),
        }
    }

    #[must_use]
    pub fn emoji(mut self, value: DiceEmoji) -> Self {
        self.inner.emoji = Some(value);
        self
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

    #[must_use]
    pub fn reply_to_message_id(mut self, value: i64) -> Self {
        self.inner.reply_to_message_id = Some(value);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(value);
        self
    }

    #[must_use]
    pub fn reply_markup<M>(mut self, value: M) -> Self
    where
        M: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(value.into());
        self
    }

    pub async fn send(self) -> Result<Message, Error> {
        self.bot.request(Method::POST, "sendDice", self.inner).await
    }
}
