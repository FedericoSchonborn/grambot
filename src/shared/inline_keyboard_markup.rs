use serde::{Deserialize, Serialize};

use crate::shared::InlineKeyboardButton;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(from = "raw::InlineKeyboardMarkup", into = "raw::InlineKeyboardMarkup")]
pub struct InlineKeyboardMarkup(pub Vec<Vec<InlineKeyboardButton>>);

impl InlineKeyboardMarkup {
    #[must_use]
    pub fn new(keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self(keyboard)
    }
}

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct InlineKeyboardMarkup {
        inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
    }

    impl From<InlineKeyboardMarkup> for super::InlineKeyboardMarkup {
        fn from(raw: InlineKeyboardMarkup) -> Self {
            Self(raw.inline_keyboard)
        }
    }

    impl From<super::InlineKeyboardMarkup> for InlineKeyboardMarkup {
        fn from(it: super::InlineKeyboardMarkup) -> Self {
            Self {
                inline_keyboard: it.0,
            }
        }
    }
}
