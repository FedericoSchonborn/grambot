use serde::Deserialize;

use crate::types::{ResponseError, ResponseParameters};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::Response<T>")]
pub enum Response<T> {
    Ok(T),
    Err(ResponseError),
}

mod raw {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize)]
    pub struct Response<T> {
        ok: bool,
        description: Option<String>,
        result: Option<T>,
        error_code: Option<i32>,
        parameters: Option<ResponseParameters>,
    }

    impl<T> From<Response<T>> for super::Response<T> {
        fn from(raw: Response<T>) -> Self {
            if raw.ok {
                Self::Ok(raw.result.expect("missing result field in `Ok` response"))
            } else {
                Self::Err(ResponseError {
                    description: raw
                        .description
                        .expect("missing description field in `Err` response"),
                    error_code: raw.error_code,
                    parameters: raw.parameters,
                })
            }
        }
    }
}
