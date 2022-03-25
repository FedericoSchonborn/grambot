use serde::{de::Error as DeError, Deserialize, Deserializer};
use thiserror::Error;

use crate::types::ResponseParameters;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Response<T> {
    Ok(T),
    Err(Error),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Error)]
#[error("{description}")]
pub struct Error {
    description: String,
    error_code: Option<i32>,
    parameters: Option<ResponseParameters>,
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
    pub fn parameters(&self) -> Option<ResponseParameters> {
        self.parameters
    }
}

impl<'de, T> Deserialize<'de> for Response<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Inner<T> {
            ok: bool,
            description: Option<String>,
            result: Option<T>,
            error_code: Option<i32>,
            parameters: Option<ResponseParameters>,
        }

        let inner = Inner::deserialize(deserializer)?;
        if inner.ok {
            Ok(Response::Ok(
                inner
                    .result
                    .ok_or_else(|| DeError::missing_field("result"))?,
            ))
        } else {
            Ok(Response::Err(Error {
                description: inner
                    .description
                    .ok_or_else(|| DeError::missing_field("description"))?,
                error_code: inner.error_code,
                parameters: inner.parameters,
            }))
        }
    }
}
