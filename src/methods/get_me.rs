use hyper::Method;
use serde::Serialize;

use crate::{types::BotUser, Bot, Error};

#[derive(Debug, Clone, Serialize)]
pub struct GetMe<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
}

impl<'bot> GetMe<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<BotUser, Error> {
        self.bot.request(Method::GET, "getMe", self).await
    }
}
