//! Extra types used by method parameters.

use serde::Serialize;

mod chat_id;
mod parse_mode;
mod reply_markup;

pub use chat_id::*;
pub use parse_mode::*;
pub use reply_markup::*;

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
