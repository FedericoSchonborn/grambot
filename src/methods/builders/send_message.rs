use crate::{
    methods::{Request, SendMessage},
    types::{MessageEntity, ParseMode, ReplyMarkup, Target},
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct SendMessageBuilder<'bot> {
    bot: &'bot Bot,
    request: SendMessage,
}

impl<'bot> SendMessageBuilder<'bot> {
    pub fn new<T, S>(bot: &'bot Bot, target: T, text: S) -> Self
    where
        T: Into<Target>,
        S: Into<String>,
    {
        Self {
            bot,
            request: SendMessage::new(target, text),
        }
    }

    pub fn parse_mode(&mut self, value: ParseMode) -> &mut Self {
        self.request.parse_mode = Some(value);
        self
    }

    pub fn entities(&mut self, value: Vec<MessageEntity>) -> &mut Self {
        self.request.entities = Some(value);
        self
    }

    pub fn disable_web_page_preview(&mut self, value: bool) -> &mut Self {
        self.request.disable_web_page_preview = Some(value);
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

    pub fn reply_markup<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<ReplyMarkup>,
    {
        self.request.reply_markup = Some(value.into());
        self
    }

    pub async fn send(&self) -> Result<<SendMessage as Request>::Response, Error> {
        self.bot.send(&self.request).await
    }
}
