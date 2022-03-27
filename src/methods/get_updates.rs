use hyper::Method;
use serde::Serialize;

use crate::{methods::types::AllowedUpdate, types::Update, Bot, Error};

#[derive(Debug, Clone, Serialize)]
pub struct GetUpdates<'bot> {
    #[serde(skip)]
    bot: &'bot Bot,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl<'bot> GetUpdates<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }

    #[must_use]
    pub fn offset(mut self, value: i32) -> Self {
        self.offset = Some(value);
        self
    }

    #[must_use]
    pub fn limit(mut self, value: i8) -> Self {
        self.limit = Some(value);
        self
    }

    #[must_use]
    pub fn timeout(mut self, value: i32) -> Self {
        self.timeout = Some(value);
        self
    }

    #[must_use]
    pub fn allowed_updates(mut self, value: Vec<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(value);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Vec<Update>, Error> {
        self.bot.request(Method::GET, "getUpdates", self).await
    }
}
