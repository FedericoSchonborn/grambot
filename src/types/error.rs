use thiserror::Error;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
#[error("expected one of `message`, `edited_message`, `channel_post`, `edited_channel_post`, `inline_query`, `chosen_inline_result`, `callback_query`, `shipping_query`, `pre_checkout_query`, `poll`, `poll_answer`, `my_chat_member`, `chat_member`, `can_join_request`")]
pub struct TryFromAllowedUpdateError;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
#[error("expected one of `🎲`, `🎯`, `🎳`, `🏀`, `⚽`, `🎰`")]
pub struct TryFromDiceError;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Error)]
#[error("expected one of `HTML`, `Markdown`, `MarkdownV2`")]
pub struct TryFromParseModeError;
