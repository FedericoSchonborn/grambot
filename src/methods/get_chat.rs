use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{Chat, ChatId},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct GetChat(#[serde(rename = "chat_id")] pub ChatId);

impl GetChat {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self(chat_id.into())
    }
}

impl Request for GetChat {
    const NAME: &'static str = "getChat";
    const METHOD: Method = Method::GET;
    type Response = Chat;
}
