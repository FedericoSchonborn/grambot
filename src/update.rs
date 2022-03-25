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
        struct RawUpdate {
            update_id: i32,
            message: Option<Message>,
            edited_message: Option<Message>,
            channel_post: Option<Message>,
            edited_channel_post: Option<Message>,
            // TODO
        }

        let update @ RawUpdate { update_id: id, .. } = RawUpdate::deserialize(deserializer)?;
        if let Some(message) = update.message {
            return Ok(Update {
                id,
                kind: UpdateKind::Message(message),
            });
        }
        if let Some(edited_message) = update.edited_message {
            return Ok(Update {
                id,
                kind: UpdateKind::EditedMessage(edited_message),
            });
        }
        if let Some(channel_post) = update.channel_post {
            return Ok(Update {
                id,
                kind: UpdateKind::ChannelPost(channel_post),
            });
        }
        if let Some(edited_channel_post) = update.edited_channel_post {
            return Ok(Update {
                id,
                kind: UpdateKind::EditedChannelPost(edited_channel_post),
            });
        }

        todo!()
    }
}
