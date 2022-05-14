use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{DiceEmoji, Message, ReplyMarkup, Target},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendDice {
    pub chat_id: Target,
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
    pub fn new<T>(target: T) -> Self
    where
        T: Into<Target>,
    {
        Self {
            chat_id: target.into(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }
}

impl Request for SendDice {
    const NAME: &'static str = "sendDice";
    const METHOD: Method = Method::POST;
    type Response = Message;
}
