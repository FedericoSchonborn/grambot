use hyper::{body, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    error::Error,
    methods::{
        builders::{GetUpdatesBuilder, SendMessageBuilder},
        types::ChatId,
        GetChat, GetUpdates, SendMessage,
    },
    types::{Chat, Message, Response, Update, User},
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

    async fn request<P, T>(&self, method: Method, endpoint: &str, params: P) -> Result<T, Error>
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

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_updates(&self, params: GetUpdates) -> Result<Vec<Update>, Error> {
        self.request(Method::GET, "getUpdates", params).await
    }

    #[must_use]
    pub fn new_get_updates(&self) -> GetUpdatesBuilder<'_> {
        GetUpdatesBuilder::new(self)
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_me(&self) -> Result<User, Error> {
        self.request(Method::GET, "getMe", ()).await
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn log_out(&self) -> Result<(), Error> {
        self.request::<_, bool>(Method::POST, "logOut", ())
            .await
            .map(|_| ())
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn close(&self) -> Result<(), Error> {
        self.request::<_, bool>(Method::POST, "close", ())
            .await
            .map(|_| ())
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send_message(&self, params: SendMessage) -> Result<Message, Error> {
        self.request(Method::POST, "sendMessage", params).await
    }

    #[must_use]
    pub fn new_message<C, T>(&self, chat_id: C, text: T) -> SendMessageBuilder<'_>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessageBuilder::new(self, chat_id, text)
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn get_chat<C>(&self, chat_id: C) -> Result<Chat, Error>
    where
        C: Into<ChatId>,
    {
        self.request(Method::GET, "getChat", GetChat::new(chat_id))
            .await
    }
}
