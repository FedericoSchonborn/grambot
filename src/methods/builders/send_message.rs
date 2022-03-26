use crate::{
    methods::{
        types::{ChatId, ParseMode, ReplyMarkup},
        SendMessage,
    },
    shared::MessageEntity,
    types::Message,
    Bot, Error,
};

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct SendMessageBuilder<'bot> {
    bot: &'bot Bot,
    inner: SendMessage,
}

impl<'bot> SendMessageBuilder<'bot> {
    pub(crate) fn new<C, T>(bot: &'bot Bot, chat_id: C, text: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            bot,
            inner: SendMessage::new(chat_id, text),
        }
    }

    #[must_use]
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.parse_mode = Some(value);
        self
    }

    #[must_use]
    pub fn entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.inner.entities = Some(value);
        self
    }

    #[must_use]
    pub fn disable_web_page_preview(mut self, value: bool) -> Self {
        self.inner.disable_web_page_preview = Some(value);
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
    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.inner.reply_to_message_id = Some(value);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(value);
        self
    }

    #[must_use]
    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.inner.reply_markup = Some(value.into());
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Message, Error> {
        self.bot.send_message(self.inner).await
    }
}
