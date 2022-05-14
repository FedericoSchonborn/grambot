use crate::{
    methods::{Request, SendDice},
    types::{DiceEmoji, ReplyMarkup, Target},
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct SendDiceBuilder<'bot> {
    bot: &'bot Bot,
    request: SendDice,
}

impl<'bot> SendDiceBuilder<'bot> {
    pub fn new<T>(bot: &'bot Bot, target: T) -> Self
    where
        T: Into<Target>,
    {
        Self {
            bot,
            request: SendDice::new(target),
        }
    }

    pub fn emoji(&mut self, value: DiceEmoji) -> &mut Self {
        self.request.emoji = Some(value);
        self
    }

    pub fn disable_notification(&mut self, value: bool) -> &mut Self {
        self.request.disable_notification = Some(value);
        self
    }

    pub fn protect_content(&mut self, value: bool) -> &mut Self {
        self.request.protect_content = Some(value);
        self
    }

    pub fn reply_to_message_id(&mut self, value: i64) -> &mut Self {
        self.request.reply_to_message_id = Some(value);
        self
    }

    pub fn allow_sending_without_reply(&mut self, value: bool) -> &mut Self {
        self.request.allow_sending_without_reply = Some(value);
        self
    }

    pub fn reply_markup<M>(&mut self, value: M) -> &mut Self
    where
        M: Into<ReplyMarkup>,
    {
        self.request.reply_markup = Some(value.into());
        self
    }

    pub async fn send(&self) -> Result<<SendDice as Request>::Response, Error> {
        self.bot.send(&self.request).await
    }
}
