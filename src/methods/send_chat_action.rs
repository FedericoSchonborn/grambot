use hyper::Method;
use serde::Serialize;

use crate::{
    traits::Request,
    types::{ChatAction, ChatId},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendChatAction {
    chat_id: ChatId,
    action: ChatAction,
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
    type Output = ();
}
