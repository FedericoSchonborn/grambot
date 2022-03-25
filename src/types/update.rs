use serde::{Deserialize, Deserializer};

use crate::types::Message;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            update_id: i32,
            message: Option<Message>,
            edited_message: Option<Message>,
            channel_post: Option<Message>,
            edited_channel_post: Option<Message>,
            // TODO
        }

        let inner = Inner::deserialize(deserializer)?;
        Ok(Update {
            id: inner.update_id,
            kind: {
                if let Some(message) = inner.message {
                    UpdateKind::Message(message)
                } else if let Some(edited_message) = inner.edited_message {
                    UpdateKind::EditedMessage(edited_message)
                } else if let Some(channel_post) = inner.channel_post {
                    UpdateKind::ChannelPost(channel_post)
                } else if let Some(edited_channel_post) = inner.edited_channel_post {
                    UpdateKind::EditedChannelPost(edited_channel_post)
                } else {
                    todo!()
                }
            },
        })
    }
}
