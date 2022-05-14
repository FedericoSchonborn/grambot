use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::Serialize;

use crate::types::errors::ParseParseModeError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum ParseMode {
    #[serde(rename = "HTML")]
    Html,
    Markdown,
    MarkdownV2,
}

impl ParseMode {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Html => "HTML",
            Self::Markdown => "Markdown",
            Self::MarkdownV2 => "MarkdownV2",
        }
    }
}

impl Display for ParseMode {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl FromStr for ParseMode {
    type Err = ParseParseModeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "HTML" => Ok(Self::Html),
            "Markdown" => Ok(Self::Markdown),
            "MarkdownV2" => Ok(Self::MarkdownV2),
            _ => Err(ParseParseModeError),
        }
    }
}
