use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::MessageId")]
pub struct MessageId(pub i32);

mod raw {
    use super::*;

    #[derive(Deserialize)]
    pub struct MessageId {
        message_id: i32,
    }

    impl From<MessageId> for super::MessageId {
        fn from(raw: MessageId) -> Self {
            Self(raw.message_id)
        }
    }
}
