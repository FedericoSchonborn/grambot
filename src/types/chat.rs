use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::Deserialize;

use crate::types::errors::ParseChatKindError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub kind: ChatKind,
    // TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatKind {
    Private,
    Group,
    Supergroup,
    Channel,
}

impl ChatKind {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Group => "group",
            Self::Supergroup => "supergroup",
            Self::Channel => "channel",
        }
    }
}

impl Display for ChatKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for ChatKind {
    type Err = ParseChatKindError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "private" => Ok(Self::Private),
            "group" => Ok(Self::Group),
            "supergroup" => Ok(Self::Supergroup),
            "channel" => Ok(Self::Channel),
            _ => Err(ParseChatKindError),
        }
    }
}
