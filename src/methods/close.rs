use hyper::Method;
use serde::Serialize;

use crate::{methods::Request, types::True};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct Close;

impl Request for Close {
    const NAME: &'static str = "close";
    const METHOD: Method = Method::POST;
    type Response = True;
}
