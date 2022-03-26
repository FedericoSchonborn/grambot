use serde::{Deserialize, Serialize};

use crate::shared::KeyboardButton;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    // TODO
}

impl ReplyKeyboardMarkup {
    #[must_use]
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self { keyboard }
    }
}
