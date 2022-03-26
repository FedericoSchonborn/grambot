use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
    // TODO: request_poll
}

impl KeyboardButton {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            ..KeyboardButton::default()
        }
    }
}
