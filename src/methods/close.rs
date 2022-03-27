use hyper::Method;
use serde::Serialize;

use crate::{Bot, Error};

#[derive(Debug, Clone, Serialize)]
pub struct Close<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
}

impl<'bot> Close<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self { bot }
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<(), Error> {
        self.bot
            .request::<_, bool>(Method::POST, "close", self)
            .await
            .map(|_| ())
    }
}
