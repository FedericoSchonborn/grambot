use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct InlineKeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    // TODO
}

impl InlineKeyboardButton {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            url: None,
        }
    }
}
