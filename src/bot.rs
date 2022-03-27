use hyper::{body, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::Error,
    methods::{types::ChatId, Close, GetChat, GetMe, GetUpdates, LogOut, SendMessage},
    types::Response,
};

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

    pub(crate) async fn request<P, T>(
        &self,
        method: Method,
        endpoint: &str,
        params: P,
    ) -> Result<T, Error>
    where
        P: Serialize,
        T: DeserializeOwned,
    {
        let content = serde_json::to_string(&params)?;
        let request = Request::builder()
            .method(method)
            .uri(format!(
                "{server}/bot{token}/{endpoint}",
                server = self.server,
                token = self.token,
            ))
            .header("Content-Type", "application/json")
            .body(Body::from(content))?;

        let body = self.client.request(request).await?.into_body();
        let bytes = body::to_bytes(body).await?;
        let response: Response<T> = serde_json::from_slice(&bytes)?;
        match response {
            Response::Ok(result) => Ok(result),
            Response::Err(err) => Err(Error::Response(err)),
        }
    }

    #[must_use]
    pub fn get_updates(&self) -> GetUpdates<'_> {
        GetUpdates::new(self)
    }

    #[must_use]
    pub fn get_me(&self) -> GetMe<'_> {
        GetMe::new(self)
    }

    #[must_use]
    pub fn log_out(&self) -> LogOut<'_> {
        LogOut::new(self)
    }

    #[must_use]
    pub fn close(&self) -> Close<'_> {
        Close::new(self)
    }

    #[must_use]
    pub fn new_message<C, T>(&self, chat_id: C, text: T) -> SendMessage<'_>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessage::new(self, chat_id, text)
    }

    #[must_use]
    pub fn get_chat<C>(&self, chat_id: C) -> GetChat<'_>
    where
        C: Into<ChatId>,
    {
        GetChat::new(self, chat_id)
    }
}
