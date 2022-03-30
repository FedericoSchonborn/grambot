use hyper::Method;
use serde::Serialize;

use crate::traits::Request;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct LogOut;

impl Request for LogOut {
    const NAME: &'static str = "logOut";
    const METHOD: Method = Method::POST;
    type Output = ();
}
