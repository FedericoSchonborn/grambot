use serde::Serialize;

use crate::types::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    Keyboard(ReplyKeyboardMarkup),
    KeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(inner: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboard(inner)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(inner: ReplyKeyboardMarkup) -> Self {
        Self::Keyboard(inner)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(inner: ReplyKeyboardRemove) -> Self {
        Self::KeyboardRemove(inner)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(inner: ForceReply) -> Self {
        Self::ForceReply(inner)
    }
}
