use thiserror::Error;

use crate::types::ResponseParameters;

/// Compound error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{description}")]
    Response {
        description: String,
        error_code: Option<i32>,
        parameters: Option<ResponseParameters>,
    },
}
