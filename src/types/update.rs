use serde::Deserialize;

use crate::types::{ChosenInlineResult, InlineQuery, Message};

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
#[serde(from = "raw::Update")]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

impl Update {
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        match &self.kind {
            UpdateKind::Message(message) => Some(message),
            _ => None,
        }
    }

    #[must_use]
    pub fn edited_message(&self) -> Option<&Message> {
        match &self.kind {
            UpdateKind::EditedMessage(message) => Some(message),
            _ => None,
        }
    }

    #[must_use]
    pub fn channel_post(&self) -> Option<&Message> {
        match &self.kind {
            UpdateKind::ChannelPost(message) => Some(message),
            _ => None,
        }
    }

    #[must_use]
    pub fn edited_channel_post(&self) -> Option<&Message> {
        match &self.kind {
            UpdateKind::EditedChannelPost(message) => Some(message),
            _ => None,
        }
    }

    #[must_use]
    pub fn inline_query(&self) -> Option<&InlineQuery> {
        match &self.kind {
            UpdateKind::InlineQuery(inline_query) => Some(inline_query),
            _ => None,
        }
    }

    #[must_use]
    pub fn chosen_inline_result(&self) -> Option<&ChosenInlineResult> {
        match &self.kind {
            UpdateKind::ChosenInlineResult(chosen_inline_result) => Some(chosen_inline_result),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    // TODO
}

mod raw {
    use super::*;

    #[derive(Deserialize)]
    pub struct Update {
        update_id: i32,
        message: Option<Message>,
        edited_message: Option<Message>,
        channel_post: Option<Message>,
        edited_channel_post: Option<Message>,
        inline_query: Option<InlineQuery>,
        chosen_inline_result: Option<ChosenInlineResult>,
        // TODO
    }

    impl From<Update> for super::Update {
        fn from(raw: Update) -> Self {
            Self {
                id: raw.update_id,
                kind: {
                    #[allow(clippy::enum_glob_use)]
                    use UpdateKind::*;

                    if let Some(message) = raw.message {
                        Message(message)
                    } else if let Some(edited_message) = raw.edited_message {
                        EditedMessage(edited_message)
                    } else if let Some(channel_post) = raw.channel_post {
                        ChannelPost(channel_post)
                    } else if let Some(edited_channel_post) = raw.edited_channel_post {
                        EditedChannelPost(edited_channel_post)
                    } else if let Some(inline_query) = raw.inline_query {
                        InlineQuery(inline_query)
                    } else if let Some(chosen_inline_result) = raw.chosen_inline_result {
                        ChosenInlineResult(chosen_inline_result)
                    } else {
                        todo!()
                    }
                },
            }
        }
    }
}
