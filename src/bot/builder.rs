use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

use crate::{consts::DEFAULT_HOST, Bot};

#[derive(Debug, Clone)]
pub struct Builder {
    token: String,
    client: Option<Client<HttpsConnector<HttpConnector>>>,
    host: Option<String>,
}

impl Builder {
    pub fn new<T>(token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            token: token.into(),
            client: None,
            host: None,
        }
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
    pub fn build(self) -> Bot {
        Bot {
            token: self.token,
            client: self
                .client
                .unwrap_or_else(|| Client::builder().build(HttpsConnector::new())),
            host: self.host.unwrap_or_else(|| String::from(DEFAULT_HOST)),
        }
    }
}
