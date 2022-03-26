//! Extra types used by method parameters.

use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

use crate::shared::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    CanJoinRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    Integer(i64),
    String(String),
}

impl From<i64> for ChatId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for ChatId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum ParseMode {
    #[serde(rename = "HTML")]
    Html,
    Markdown,
    MarkdownV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("expected one of `HTML`, `Markdown`, `MarkdownV2`")]
pub struct TryFromParseModeError;

impl FromStr for ParseMode {
    type Err = TryFromParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "HTML" => Ok(Self::Html),
            "Markdown" => Ok(Self::Markdown),
            "MarkdownV2" => Ok(Self::MarkdownV2),
            _ => Err(TryFromParseModeError),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
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
