use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

use crate::{bot::DEFAULT_SERVER, Bot};

#[derive(Debug, Clone, Default)]
pub struct Builder {
    client: Option<Client<HttpsConnector<HttpConnector>>>,
    host: Option<String>,
}

impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn client(mut self, value: Client<HttpsConnector<HttpConnector>>) -> Self {
        self.client = Some(value);
        self
    }

    #[must_use]
    pub fn host<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.host = Some(value.into());
        self
    }

    #[must_use]
    pub fn build<T>(self, token: T) -> Bot
    where
        T: Into<String>,
    {
        Bot {
            client: self
                .client
                .unwrap_or_else(|| Client::builder().build(HttpsConnector::new())),
            host: self.host.unwrap_or_else(|| String::from(DEFAULT_SERVER)),
            token: token.into(),
        }
    }
}
