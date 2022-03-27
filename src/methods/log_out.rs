use hyper::Method;
use serde::Serialize;

use crate::{Bot, Error};

#[derive(Debug, Clone, Serialize)]
pub struct LogOut<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
}

impl<'bot> LogOut<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<(), Error> {
        self.bot
            .request::<_, bool>(Method::POST, "logOut", self)
            .await
            .map(|_| ())
    }
}
