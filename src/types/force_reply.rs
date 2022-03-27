use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
#[serde(from = "raw::ForceReply", into = "raw::ForceReply")]
pub struct ForceReply {
    pub input_field_placeholder: Option<String>,
    pub selective: Option<bool>,
}

impl ForceReply {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize, Serialize)]
    pub struct ForceReply {
        force_reply: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_field_placeholder: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        selective: Option<bool>,
    }

    impl From<ForceReply> for super::ForceReply {
        fn from(raw: ForceReply) -> Self {
            Self {
                input_field_placeholder: raw.input_field_placeholder,
                selective: raw.selective,
            }
        }
    }

    impl From<super::ForceReply> for ForceReply {
        fn from(it: super::ForceReply) -> Self {
            Self {
                force_reply: true,
                input_field_placeholder: it.input_field_placeholder,
                selective: it.selective,
            }
        }
    }
}
