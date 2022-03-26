use serde::Serialize;

use crate::methods::types::AllowedUpdate;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl GetUpdates {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
