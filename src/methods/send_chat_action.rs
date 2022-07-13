use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{ChatAction, Target, True},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct SendChatAction {
    pub chat_id: Target,
    pub action: ChatAction,
}

impl SendChatAction {
    pub fn new<T>(target: T, action: ChatAction) -> Self
    where
        T: Into<Target>,
    {
        Self {
            chat_id: target.into(),
            action,
        }
    }
}

impl Request for SendChatAction {
    const ENDPOINT: &'static str = "sendChatAction";
    const METHOD: Method = Method::POST;
    type Output = True;
}
