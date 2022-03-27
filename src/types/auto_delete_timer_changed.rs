use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::AutoDeleteTimerChanged")]
pub struct AutoDeleteTimerChanged(pub i32);

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize)]
    pub struct AutoDeleteTimerChanged {
        #[serde(rename = "message_auto_delete_time")]
        auto_delete_time: i32,
    }

    impl From<AutoDeleteTimerChanged> for super::AutoDeleteTimerChanged {
        fn from(raw: AutoDeleteTimerChanged) -> Self {
            Self(raw.auto_delete_time)
        }
    }
}
