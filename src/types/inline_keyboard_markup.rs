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
