use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{Chat, Target},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct GetChat(#[serde(rename = "chat_id")] pub Target);

impl GetChat {
    pub fn new<T>(target: T) -> Self
    where
        T: Into<Target>,
    {
        Self(target.into())
    }
}

impl Request for GetChat {
    const ENDPOINT: &'static str = "getChat";
    const METHOD: Method = Method::GET;
    type Output = Chat;
}
