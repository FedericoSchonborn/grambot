use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{ChatAction, ChatId, True},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendChatAction {
    pub chat_id: ChatId,
    pub action: ChatAction,
}

impl SendChatAction {
    pub fn new<C>(chat_id: C, action: ChatAction) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            action,
        }
    }
}

impl Request for SendChatAction {
    const NAME: &'static str = "sendChatAction";
    const METHOD: Method = Method::POST;
    type Response = True;
}
