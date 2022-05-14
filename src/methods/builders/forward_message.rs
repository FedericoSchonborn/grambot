use crate::{methods::ForwardMessage, types::Target};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ForwardMessageBuilder {
    disable_notification: Option<bool>,
    protect_content: Option<bool>,
}

impl ForwardMessageBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    pub fn build<T, F>(self, target: T, from_target: F, message_id: i64) -> ForwardMessage
    where
        T: Into<Target>,
        F: Into<Target>,
    {
        ForwardMessage {
            chat_id: target.into(),
            from_chat_id: from_target.into(),
            message_id,
            disable_notification: self.disable_notification,
            protect_content: self.protect_content,
        }
    }
}
