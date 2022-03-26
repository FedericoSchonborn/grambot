use serde::Serialize;

use crate::methods::types::ChatId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(into = "raw::GetChat")]
pub struct GetChat(pub ChatId);

impl GetChat {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self(chat_id.into())
    }
}

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Serialize)]
    pub struct GetChat {
        chat_id: ChatId,
    }

    impl From<super::GetChat> for GetChat {
        fn from(it: super::GetChat) -> Self {
            Self { chat_id: it.0 }
        }
    }
}
