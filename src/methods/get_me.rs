use hyper::Method;
use serde::Serialize;

use crate::{methods::Request, types::User};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct GetMe;

impl Request for GetMe {
    const NAME: &'static str = "getMe";
    const METHOD: Method = Method::GET;
    type Response = User;
}
