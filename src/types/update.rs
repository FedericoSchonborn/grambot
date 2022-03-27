use serde::Deserialize;

use crate::types::Message;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::Update")]
pub struct Update {
    pub id: i32,
    pub kind: UpdateKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::module_name_repetitions)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    // TODO
}

mod raw {
    #[allow(clippy::wildcard_imports)]
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
                    if let Some(message) = raw.message {
                        UpdateKind::Message(message)
                    } else if let Some(edited_message) = raw.edited_message {
                        UpdateKind::EditedMessage(edited_message)
                    } else if let Some(channel_post) = raw.channel_post {
                        UpdateKind::ChannelPost(channel_post)
                    } else if let Some(edited_channel_post) = raw.edited_channel_post {
                        UpdateKind::EditedChannelPost(edited_channel_post)
                    } else {
                        todo!()
                    }
                },
            }
        }
    }
}
