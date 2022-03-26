use serde::Serialize;

use crate::methods::types::ChatId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct GetChat {
    pub chat_id: ChatId,
}

impl GetChat {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
        }
    }
}
