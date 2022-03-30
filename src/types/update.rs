use serde::Deserialize;

use crate::types::Message;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::Update")]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

impl Update {
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        self.kind.message()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    // TODO
}

impl UpdateKind {
    #[must_use]
    pub fn message(&self) -> Option<&Message> {
        match self {
            Self::Message(message)
            | Self::EditedMessage(message)
            | Self::ChannelPost(message)
            | Self::EditedChannelPost(message) => Some(message),
            _ => None,
        }
    }
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
                    } else {
                        todo!()
                    }
                },
            }
        }
    }
}
