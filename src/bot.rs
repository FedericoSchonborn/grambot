use hyper::{body, client::HttpConnector, http::response::Parts, Body, Client, Method, Request};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    builders::SendMessageBuilder,
    error::Error,
    methods::{ChatId, SendMessage},
    types::{Message, ResponseParameters, User},
};

#[derive(Debug)]
pub struct Bot {
    client: Client<HttpsConnector<HttpConnector>>,
    token: String,
}

impl Bot {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            client: Client::builder().build(HttpsConnector::new()),
            token: token.into(),
        }
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
        #[derive(Deserialize)]
        struct Response<T> {
            pub ok: bool,
            pub description: Option<String>,
            pub result: Option<T>,
            pub error_code: Option<i32>,
            pub parameters: Option<ResponseParameters>,
        }

        let query = serde_urlencoded::to_string(&params)?;
        let request = Request::builder()
            .method(method)
            .uri(format!(
                "https://api.telegram.org/bot{token}/{endpoint}?{query}",
                token = self.token,
            ))
            .body(Body::empty())?;

        let (Parts { status, .. }, body) = self.client.request(request).await?.into_parts();
        if !status.is_success() {
            return Err(Error::StatusCode(status));
        }

        let body = body::to_bytes(body).await?;
        let response: Response<T> = serde_json::from_slice(&body)?;
        if response.ok {
            Ok(response.result.ok_or(Error::MissingResult)?)
        } else {
            Err(Error::Api {
                description: response.description.ok_or(Error::MissingDescription)?,
                error_code: response.error_code,
                parameters: response.parameters,
            })
        }
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

    pub fn new_message<C, T>(&self, chat_id: C, text: T) -> SendMessageBuilder<'_>
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        SendMessageBuilder::new(self, chat_id, text)
    }
}
