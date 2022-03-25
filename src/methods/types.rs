//! Extra types used by method parameters.

use serde::Serialize;

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
