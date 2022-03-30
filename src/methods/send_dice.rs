use hyper::Method;
use serde::Serialize;

use crate::{
    builders::SendDiceBuilder,
    traits::Request,
    types::{ChatId, DiceEmoji, Message, ReplyMarkup},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendDice {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<DiceEmoji>,
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

impl SendDice {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
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
    pub fn builder() -> SendDiceBuilder {
        SendDiceBuilder::new()
    }
}

impl Request for SendDice {
    const NAME: &'static str = "sendDice";
    const METHOD: Method = Method::POST;
    type Output = Message;
}
