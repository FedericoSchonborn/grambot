use hyper::Method;
use serde::Serialize;

use crate::{methods::Request, types::True};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct LogOut;

impl Request for LogOut {
    const ENDPOINT: &'static str = "logOut";
    const METHOD: Method = Method::POST;
    type Output = True;
}
