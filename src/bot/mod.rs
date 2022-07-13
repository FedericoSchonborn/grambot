//! Bot API client.

use hyper::{body, client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use crate::{
    consts::DEFAULT_HOST,
    error::Error,
    methods::{
        builders::{ForwardMessageBuilder, GetUpdatesBuilder, SendDiceBuilder, SendMessageBuilder},
        GetUpdates, Request,
    },
    stream::UpdateStream,
    types::{ResponseParameters, Target},
};

mod builder;
pub use builder::*;

#[derive(Debug)]
pub struct Bot {
    pub(self) client: Client<HttpsConnector<HttpConnector>>,
    pub(self) host: String,
    pub(self) token: String,
}

impl Bot {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: Client::builder().build(HttpsConnector::new()),
            host: String::from(DEFAULT_HOST),
            token: token.into(),
        }
    }

    pub fn builder<T>(token: T) -> Builder
    where
        T: Into<String>,
    {
        Builder::new(token)
    }

    pub async fn send<R>(&self, request: R) -> Result<R::Output, Error>
    where
        R: Request,
    {
        #[derive(Deserialize)]
        struct Response<T> {
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
                "{host}/bot{token}/{endpoint}",
                host = self.host,
                token = self.token,
                endpoint = R::ENDPOINT,
            ))
            .header(
                "User-Agent",
                concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),),
            )
            .header("Content-Type", "application/json")
            .body(body)?;

        let body = self.client.request(request).await?.into_body();
        let bytes = body::to_bytes(body).await?;

        let response: Response<R::Output> = serde_json::from_slice(&bytes)?;
        if response.ok {
            Ok(response
                .result
                .expect("successful response is missing result"))
        } else {
            Err(Error::Response {
                description: response
                    .description
                    .expect("error response is missing description"),
                error_code: response.error_code,
                parameters: response.parameters,
            })
        }
    }

    #[must_use]
    pub fn stream_updates(&self) -> UpdateStream<'_> {
        UpdateStream::new(self, GetUpdates::default())
    }

    #[must_use]
    pub fn get_updates(&self) -> GetUpdatesBuilder<'_> {
        GetUpdatesBuilder::new(self)
    }

    pub fn message<T, S>(&self, target: T, text: S) -> SendMessageBuilder<'_>
    where
        T: Into<Target>,
        S: Into<String>,
    {
        SendMessageBuilder::new(self, target, text)
    }

    pub fn forward_message<T, F>(
        &self,
        target: T,
        from_target: F,
        message_id: i64,
    ) -> ForwardMessageBuilder<'_>
    where
        T: Into<Target>,
        F: Into<Target>,
    {
        ForwardMessageBuilder::new(self, target, from_target, message_id)
    }

    pub fn dice<T>(&self, target: T) -> SendDiceBuilder<'_>
    where
        T: Into<Target>,
    {
        SendDiceBuilder::new(self, target)
    }
}
