use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

use crate::{
    bot::{Bot, DEFAULT_SERVER},
    error::Error,
    methods::{AllowedUpdate, ChatId, GetUpdates, ParseMode, SendMessage},
    types::{Message, MessageEntity},
    update::Update,
};

#[derive(Debug, Clone, Default)]
pub struct Builder {
    client: Option<Client<HttpsConnector<HttpConnector>>>,
    server: Option<String>,
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
    pub fn server<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.server = Some(value.into());
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
            server: self.server.unwrap_or_else(|| String::from(DEFAULT_SERVER)),
            token: token.into(),
        }
    }
}

pub struct GetUpdatesBuilder<'bot> {
    bot: &'bot Bot,
    inner: GetUpdates,
}

impl<'bot> GetUpdatesBuilder<'bot> {
    #[must_use]
    pub fn new(bot: &'bot Bot) -> Self {
        Self {
            bot,
            inner: GetUpdates::new(),
        }
    }

    #[must_use]
    pub fn offset(mut self, value: i32) -> Self {
        self.inner.offset = Some(value);
        self
    }

    #[must_use]
    pub fn limit(mut self, value: i8) -> Self {
        self.inner.limit = Some(value);
        self
    }

    #[must_use]
    pub fn timeout(mut self, value: i32) -> Self {
        self.inner.timeout = Some(value);
        self
    }

    #[must_use]
    pub fn allowed_updates(mut self, value: Vec<AllowedUpdate>) -> Self {
        self.inner.allowed_updates = Some(value);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Vec<Update>, Error> {
        self.bot.get_updates(self.inner).await
    }
}

pub struct SendMessageBuilder<'bot> {
    bot: &'bot Bot,
    inner: SendMessage,
}

impl<'bot> SendMessageBuilder<'bot> {
    pub(crate) fn new<C, T>(bot: &'bot Bot, chat_id: C, text: T) -> Self
    where
        C: Into<ChatId>,
        T: Into<String>,
    {
        Self {
            bot,
            inner: SendMessage::new(chat_id, text),
        }
    }

    #[must_use]
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.inner.parse_mode = Some(value);
        self
    }

    #[must_use]
    pub fn entities(mut self, value: Vec<MessageEntity>) -> Self {
        self.inner.entities = Some(value);
        self
    }

    #[must_use]
    pub fn disable_web_page_preview(mut self, value: bool) -> Self {
        self.inner.disable_web_page_preview = Some(value);
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.inner.disable_notification = Some(value);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, value: bool) -> Self {
        self.inner.protect_content = Some(value);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.inner.reply_to_message_id = Some(value);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, value: bool) -> Self {
        self.inner.allow_sending_without_reply = Some(value);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    pub async fn send(self) -> Result<Message, Error> {
        self.bot.send_message(self.inner).await
    }
}
