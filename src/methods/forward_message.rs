use hyper::Method;
use serde::Serialize;

use crate::{
    methods::builders::ForwardMessageBuilder,
    methods::Request,
    types::{Message, Target},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct ForwardMessage {
    pub chat_id: Target,
    pub from_chat_id: Target,
    pub message_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl ForwardMessage {
    pub fn new<T, F>(target: T, from_target: F, message_id: i64) -> Self
    where
        T: Into<Target>,
        F: Into<Target>,
    {
        Self {
            chat_id: target.into(),
            from_chat_id: from_target.into(),
            message_id,
            disable_notification: None,
            protect_content: None,
        }
    }

    #[must_use]
    pub fn builder() -> ForwardMessageBuilder {
        ForwardMessageBuilder::new()
    }
}

impl Request for ForwardMessage {
    const NAME: &'static str = "forwardMessage";
    const METHOD: Method = Method::POST;
    type Response = Message;
}
