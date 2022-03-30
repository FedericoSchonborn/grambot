use crate::{methods::GetUpdates, types::AllowedUpdate};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetUpdatesBuilder {
    offset: Option<i32>,
    limit: Option<i8>,
    timeout: Option<i32>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl GetUpdatesBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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

    #[must_use]
    pub fn build(self) -> GetUpdates {
        GetUpdates {
            offset: self.offset,
            limit: self.limit,
            timeout: self.timeout,
            allowed_updates: self.allowed_updates,
        }
    }
}
