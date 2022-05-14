use std::fmt::{self, Display, Formatter};

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(untagged)]
pub enum ChatId {
    /// Unique identifier of the target chat.
    Integer(i64),
    /// Username of the target channel (in the format `@channelname`).
    String(String),
}

impl Display for ChatId {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Integer(value) => value.fmt(f),
            Self::String(value) => value.fmt(f),
        }
    }
}

impl From<i64> for ChatId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for ChatId {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
