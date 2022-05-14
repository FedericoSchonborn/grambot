use crate::{
    methods::SendMessage,
    types::{MessageEntity, ParseMode, ReplyMarkup, Target},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendMessageBuilder {
    parse_mode: Option<ParseMode>,
    entities: Option<Vec<MessageEntity>>,
    disable_web_page_preview: Option<bool>,
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
    reply_to_message_id: Option<i64>,
    allow_sending_without_reply: Option<bool>,
    reply_markup: Option<ReplyMarkup>,
}

impl SendMessageBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }

    #[must_use]
    pub fn entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.entities = Some(value);
        self
    }

    #[must_use]
    pub fn disable_web_page_preview(mut self, value: bool) -> Self {
        self.disable_web_page_preview = Some(value);
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
    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    pub fn build<T, S>(self, target: T, text: S) -> SendMessage
    where
        T: Into<Target>,
        S: Into<String>,
    {
        SendMessage {
            chat_id: target.into(),
            text: text.into(),
            parse_mode: self.parse_mode,
            entities: self.entities,
            disable_web_page_preview: self.disable_web_page_preview,
            disable_notification: self.disable_notification,
            protect_content: self.protect_content,
            reply_to_message_id: self.reply_to_message_id,
            allow_sending_without_reply: self.allow_sending_without_reply,
            reply_markup: self.reply_markup,
        }
    }
}
