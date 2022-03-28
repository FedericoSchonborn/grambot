use hyper::Method;

use crate::{
    methods::GetUpdates,
    types::{AllowedUpdate, Update},
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct GetUpdatesBuilder<'bot> {
    bot: &'bot Bot,
    inner: GetUpdates,
}

impl<'bot> GetUpdatesBuilder<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            inner: GetUpdates::new(),
        }
    }

    #[must_use]
    pub fn offset(mut self, value: i32) -> Self {
        self.inner.offset = Some(value);
        self
    }

    #[must_use]
    pub fn limit(mut self, value: i8) -> Self {
        self.inner.limit = Some(value);
        self
    }

    #[must_use]
    pub fn timeout(mut self, value: i32) -> Self {
        self.inner.timeout = Some(value);
        self
    }

    #[must_use]
    pub fn allowed_updates(mut self, value: Vec<AllowedUpdate>) -> Self {
        self.inner.allowed_updates = Some(value);
        self
    }

    pub async fn send(self) -> Result<Vec<Update>, Error> {
        self.bot
            .request(Method::GET, "getUpdates", self.inner)
            .await
    }
}
