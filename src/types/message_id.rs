use serde::{Deserialize, Deserializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(pub i32);

impl<'de> Deserialize<'de> for MessageId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner {
            message_id: i32,
        }

        let inner = Inner::deserialize(deserializer)?;
        Ok(Self(inner.message_id))
    }
}
