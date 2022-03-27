use hyper::Method;
use serde::Serialize;

use crate::{
    types::{Chat, ChatId},
    Bot, Error,
};

#[derive(Debug, Clone, Serialize)]
pub struct GetChat<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
    chat_id: ChatId,
}

impl<'bot> GetChat<'bot> {
    pub fn new<C>(bot: &'bot Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Chat, Error> {
        self.bot.request(Method::GET, "getChat", self).await
    }
}
