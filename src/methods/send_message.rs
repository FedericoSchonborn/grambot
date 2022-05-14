use hyper::Method;
use serde::Serialize;

use crate::{
    methods::builders::SendMessageBuilder,
    methods::Request,
    types::{Message, MessageEntity, ParseMode, ReplyMarkup, Target},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendMessage {
    pub chat_id: Target,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    pub fn new<T, S>(target: T, text: S) -> Self
    where
        T: Into<Target>,
        S: Into<String>,
    {
        Self {
            chat_id: target.into(),
            text: text.into(),
            parse_mode: None,
            entities: None,
            disable_web_page_preview: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn builder() -> SendMessageBuilder {
        SendMessageBuilder::new()
    }
}

impl Request for SendMessage {
    const NAME: &'static str = "sendMessage";
    const METHOD: Method = Method::POST;
    type Response = Message;
}
