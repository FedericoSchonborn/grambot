use thiserror::Error;

use crate::output::ResponseParameters;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("{description}")]
pub struct Error {
    pub(crate) description: String,
    pub(crate) error_code: Option<i32>,
    pub(crate) parameters: Option<ResponseParameters>,
}

impl Error {
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    #[must_use]
    pub fn error_code(&self) -> Option<i32> {
        self.error_code
    }

    #[must_use]
    pub fn parameters(&self) -> Option<&ResponseParameters> {
        self.parameters.as_ref()
    }
}
