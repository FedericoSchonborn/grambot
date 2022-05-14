use hyper::Method;
use serde::Serialize;

use crate::{
    methods::Request,
    types::{AllowedUpdate, Update},
};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<AllowedUpdate>>,
}

impl GetUpdates {
    #[must_use]
    pub fn new() -> Self {
        Self {
            offset: None,
            limit: None,
            timeout: None,
            allowed_updates: None,
        }
    }
}

impl Request for GetUpdates {
    const NAME: &'static str = "getUpdates";
    const METHOD: Method = Method::GET;
    type Response = Vec<Update>;
}
