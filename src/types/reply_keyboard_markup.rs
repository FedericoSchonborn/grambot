use serde::{Deserialize, Serialize};

use crate::types::KeyboardButton;

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

impl FromIterator<KeyboardButton> for ReplyKeyboardMarkup {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = KeyboardButton>,
    {
        Self {
            keyboard: vec![Vec::from_iter(iter)],
        }
    }
}

impl FromIterator<Vec<KeyboardButton>> for ReplyKeyboardMarkup {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Vec<KeyboardButton>>,
    {
        Self {
            keyboard: Vec::from_iter(iter),
        }
    }
}
