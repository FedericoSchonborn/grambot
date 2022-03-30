use hyper::Method;
use serde::{de::DeserializeOwned, Serialize};

pub trait Request: Serialize {
    const NAME: &'static str;
    const METHOD: Method;
    type Output: DeserializeOwned;
}
