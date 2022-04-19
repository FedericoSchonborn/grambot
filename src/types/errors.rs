use thiserror::Error;

#[derive(Debug, Error)]
#[error("expected one of `message`, `edited_message`, `channel_post`, `edited_channel_post`, `inline_query`, `chosen_inline_result`, `callback_query`, `shipping_query`, `pre_checkout_query`, `poll`, `poll_answer`, `my_chat_member`, `chat_member`, `can_join_request`")]
pub struct ParseAllowedUpdateError;

#[derive(Debug, Error)]
#[error("expected one of `typing`, `upload_photo`, `record_video`, `upload_video`, `record_voice`, `upload_voice`, `upload_document`, `choose_sticker`, `find_location`, `record_video_note`, `upload_video_note`")]
pub struct ParseChatActionError;

#[derive(Debug, Error)]
#[error("expected one of `private`, `group`, `supergroup`, `channel`")]
pub struct ParseChatKindError;

#[derive(Debug, Error)]
#[error("expected one of `🎲`, `🎯`, `🎳`, `🏀`, `⚽`, `🎰`")]
pub struct ParseDiceKindError;

#[derive(Debug, Error)]
#[error("expected one of `🎲`, `🎯`, `🎳`, `🏀`, `⚽`, `🎰`")]
pub struct TryFromDiceKindError;

#[derive(Debug, Error)]
#[error("expected one of `HTML`, `Markdown`, `MarkdownV2`")]
pub struct ParseParseModeError;
