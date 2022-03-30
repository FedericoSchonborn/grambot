//! Bot API client.

use hyper::{body, client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use crate::{error::Error, traits::Request, types::ResponseParameters};

mod builder;
pub use builder::*;

pub(crate) const DEFAULT_SERVER: &str = "https://api.telegram.org";

#[derive(Debug)]
pub struct Bot {
    pub(crate) client: Client<HttpsConnector<HttpConnector>>,
    pub(crate) server: String,
    pub(crate) token: String,
}

impl Bot {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: Client::builder().build(HttpsConnector::new()),
            server: String::from(DEFAULT_SERVER),
            token: token.into(),
        }
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub async fn send<R>(&self, request: R) -> Result<R::Output, Error>
    where
        R: Request,
    {
        #[derive(Deserialize)]
        pub struct Response<T> {
            ok: bool,
            description: Option<String>,
            result: Option<T>,
            error_code: Option<i32>,
            parameters: Option<ResponseParameters>,
        }

        let content = serde_json::to_vec(&request)?;
        let body = Body::from(content);
        let request = hyper::Request::builder()
            .method(R::METHOD)
            .uri(format!(
                "{server}/bot{token}/{endpoint}",
                server = self.server,
                token = self.token,
                endpoint = R::NAME,
            ))
            .header("Content-Type", "application/json")
            .body(body)?;

        let body = self.client.request(request).await?.into_body();
        let bytes = body::to_bytes(body).await?;

        let response: Response<R::Output> = serde_json::from_slice(&bytes)?;
        if response.ok {
            Ok(response
                .result
                .expect("result field is missing in ok response"))
        } else {
            Err(Error::Response {
                description: response
                    .description
                    .expect("description field is missing in error response"),
                error_code: response.error_code,
                parameters: response.parameters,
            })
        }
    }
}
