use thiserror::Error;

use crate::output::ResponseParameters;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    UrlEncoded(#[from] serde_urlencoded::ser::Error),
    #[error(transparent)]
    Http(#[from] hyper::http::Error),
    #[error(transparent)]
    Hyper(#[from] hyper::Error),
    #[error("status code: {0}")]
    StatusCode(hyper::http::StatusCode),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("{description}")]
    Api {
        description: String,
        error_code: Option<i32>,
        parameters: Option<ResponseParameters>,
    },
    #[error("missing result")]
    MissingResult,
    #[error("missing error description")]
    MissingDescription,
}
