use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize,
)]
#[serde(from = "raw::ReplyKeyboardRemove", into = "raw::ReplyKeyboardRemove")]
pub struct ReplyKeyboardRemove {
    pub selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct ReplyKeyboardRemove {
        remove_keyboard: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    }

    impl From<ReplyKeyboardRemove> for super::ReplyKeyboardRemove {
        fn from(raw: ReplyKeyboardRemove) -> Self {
            Self {
                selective: raw.selective,
            }
        }
    }

    impl From<super::ReplyKeyboardRemove> for ReplyKeyboardRemove {
        fn from(it: super::ReplyKeyboardRemove) -> Self {
            Self {
                remove_keyboard: true,
                selective: it.selective,
            }
        }
    }
}
