//! Bot API client.

use hyper::{body, client::HttpConnector, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    builders::{ForwardMessageBuilder, GetUpdatesBuilder, SendDiceBuilder, SendMessageBuilder},
    error::Error,
    methods::{GetChat, SendChatAction},
    types::{BotUser, Chat, ChatAction, ChatId, Response},
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
        let content = serde_json::to_vec(&params)?;
        let body = Body::from(content);
        let request = Request::builder()
            .method(method)
            .uri(format!(
                "{server}/bot{token}/{endpoint}",
                server = self.server,
                token = self.token,
            ))
            .header("Content-Type", "application/json")
            .body(body)?;

        let body = self.client.request(request).await?.into_body();
        let bytes = body::to_bytes(body).await?;
        let response: Response<T> = serde_json::from_slice(&bytes)?;
        match response {
            Response::Ok(result) => Ok(result),
            Response::Err(err) => Err(err),
        }
    }

    #[must_use]
    pub fn get_updates(&self) -> GetUpdatesBuilder<'_> {
        GetUpdatesBuilder::new(self)
    }

    pub async fn get_me(&self) -> Result<BotUser, Error> {
        self.request(Method::GET, "getMe", ()).await
    }

    pub async fn log_out(&self) -> Result<(), Error> {
        self.request::<_, bool>(Method::POST, "logOut", ())
            .await
            .map(|_| ())
    }

    pub async fn close(&self) -> Result<(), Error> {
        self.request::<_, bool>(Method::POST, "close", ())
            .await
            .map(|_| ())
    }

    #[must_use]
    pub fn new_message<C, T>(&self, chat_id: C, text: T) -> SendMessageBuilder<'_>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessageBuilder::new(self, chat_id, text)
    }

    #[must_use]
    pub fn forward_message<C, F>(
        &self,
        chat_id: C,
        from_chat_id: F,
        message_id: i64,
    ) -> ForwardMessageBuilder<'_>
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        ForwardMessageBuilder::new(self, chat_id, from_chat_id, message_id)
    }

    #[must_use]
    pub fn new_dice<C>(&self, chat_id: C) -> SendDiceBuilder<'_>
    where
        C: Into<ChatId>,
    {
        SendDiceBuilder::new(self, chat_id)
    }

    pub async fn send_chat_action<C>(&self, chat_id: C, action: ChatAction) -> Result<(), Error>
    where
        C: Into<ChatId>,
    {
        self.request::<_, bool>(
            Method::POST,
            "sendChatAction",
            SendChatAction::new(chat_id, action),
        )
        .await
        .map(|_| ())
    }

    pub async fn get_chat<C>(&self, chat_id: C) -> Result<Chat, Error>
    where
        C: Into<ChatId>,
    {
        self.request(Method::GET, "getChat", GetChat::new(chat_id))
            .await
    }
}
