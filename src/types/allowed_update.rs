use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::Serialize;

use crate::types::errors::ParseAllowedUpdateError;

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

impl AllowedUpdate {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Message => "message",
            Self::EditedMessage => "edited_message",
            Self::ChannelPost => "channel_post",
            Self::EditedChannelPost => "edited_channel_post",
            Self::InlineQuery => "inline_query",
            Self::ChosenInlineResult => "chosen_inline_result",
            Self::CallbackQuery => "callback_query",
            Self::ShippingQuery => "shipping_query",
            Self::PreCheckoutQuery => "pre_checkout_query",
            Self::Poll => "poll",
            Self::PollAnswer => "poll_answer",
            Self::MyChatMember => "my_chat_member",
            Self::ChatMember => "chat_member",
            Self::CanJoinRequest => "can_join_request",
        }
    }
}

impl Display for AllowedUpdate {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for AllowedUpdate {
    type Err = ParseAllowedUpdateError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "message" => Ok(Self::Message),
            "edited_message" => Ok(Self::EditedMessage),
            "channel_post" => Ok(Self::ChannelPost),
            "edited_channel_post" => Ok(Self::EditedChannelPost),
            "inline_query" => Ok(Self::InlineQuery),
            "chosen_inline_result" => Ok(Self::ChosenInlineResult),
            "callback_query" => Ok(Self::CallbackQuery),
            "shipping_query" => Ok(Self::ShippingQuery),
            "pre_checkout_query" => Ok(Self::PreCheckoutQuery),
            "poll" => Ok(Self::Poll),
            "poll_answer" => Ok(Self::PollAnswer),
            "my_chat_member" => Ok(Self::MyChatMember),
            "chat_member" => Ok(Self::ChatMember),
            "can_join_request" => Ok(Self::CanJoinRequest),
            _ => Err(ParseAllowedUpdateError),
        }
    }
}
