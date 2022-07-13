use crate::{
    methods::{GetUpdates, Request},
    types::AllowedUpdate,
    Bot, Error,
};

#[derive(Debug, Clone)]
pub struct GetUpdatesBuilder<'bot> {
    bot: &'bot Bot,
    request: GetUpdates,
}

impl<'bot> GetUpdatesBuilder<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            request: GetUpdates::new(),
        }
    }

    pub fn offset(&mut self, value: i32) -> &mut Self {
        self.request.offset = Some(value);
        self
    }

    pub fn limit(&mut self, value: i8) -> &mut Self {
        self.request.limit = Some(value);
        self
    }

    pub fn timeout(&mut self, value: i32) -> &mut Self {
        self.request.timeout = Some(value);
        self
    }

    pub fn allowed_updates(&mut self, value: Vec<AllowedUpdate>) -> &mut Self {
        self.request.allowed_updates = Some(value);
        self
    }

    pub async fn send(&self) -> Result<<GetUpdates as Request>::Output, Error> {
        self.bot.send(&self.request).await
    }
}
