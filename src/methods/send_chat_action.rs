use hyper::Method;
use serde::Serialize;

use crate::{
    types::{ChatAction, ChatId},
    Bot, Error,
};

#[derive(Debug, Clone, Serialize)]
pub struct SendChatAction<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
    chat_id: ChatId,
    action: ChatAction,
}

impl<'bot> SendChatAction<'bot> {
    pub fn new<C>(bot: &'bot Bot, chat_id: C, action: ChatAction) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            action,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<(), Error> {
        self.bot
            .request::<_, bool>(Method::POST, "sendChatAction", self)
            .await
            .map(|_| ())
    }
}
