use thiserror::Error;

use crate::types::ResponseError;

/// Compound error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error("status code: {0}")]
    StatusCode(hyper::http::StatusCode),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Response(#[from] ResponseError),
}
