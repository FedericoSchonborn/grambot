use serde::{Deserialize, Serialize};

use crate::types::InlineKeyboardButton;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup(
    #[serde(rename = "inline_keyboard")] pub Vec<Vec<InlineKeyboardButton>>,
);

impl InlineKeyboardMarkup {
    #[must_use]
    pub fn new(keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self(keyboard)
    }
}

impl FromIterator<InlineKeyboardButton> for InlineKeyboardMarkup {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self(vec![Vec::from_iter(iter)])
    }
}

impl FromIterator<Vec<InlineKeyboardButton>> for InlineKeyboardMarkup {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<InlineKeyboardButton>>,
    {
        Self(Vec::from_iter(iter))
    }
}
