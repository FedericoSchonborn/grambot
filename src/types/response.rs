use serde::Deserialize;

use crate::{types::ResponseParameters, Error};

#[derive(Debug, Deserialize)]
#[serde(from = "raw::Response<T>")]
pub enum Response<T> {
    Ok(T),
    Err(Error),
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
                Self::Err(Error::Response {
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
