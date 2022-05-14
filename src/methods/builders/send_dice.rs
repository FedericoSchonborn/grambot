use crate::{
    methods::SendDice,
    types::{DiceEmoji, ReplyMarkup, Target},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendDiceBuilder {
    emoji: Option<DiceEmoji>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl SendDiceBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    pub fn reply_to_message_id(mut self, value: i64) -> Self {
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

    pub fn build<T>(self, target: T) -> SendDice
    where
        T: Into<Target>,
    {
        SendDice {
            chat_id: target.into(),
            emoji: self.emoji,
            disable_notification: self.disable_notification,
            protect_content: self.protect_content,
            reply_to_message_id: self.reply_to_message_id,
            allow_sending_without_reply: self.allow_sending_without_reply,
            reply_markup: self.reply_markup,
        }
    }
}
