use std::fmt::{self, Display, Formatter};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum Target {
    /// Unique identifier of the target chat.
    Chat(i64),
    /// Username of the target channel (in the format `@channelname`).
    Channel(String),
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Chat(id) => id.fmt(f),
            Self::Channel(name) => name.fmt(f),
        }
    }
}

impl From<i64> for Target {
    fn from(id: i64) -> Self {
        Self::Chat(id)
    }
}

impl From<String> for Target {
    fn from(name: String) -> Self {
        Self::Channel(name)
    }
}
