//! Available methods.

use hyper::Method;
use serde::{de::DeserializeOwned, Serialize};

pub mod builders;
mod close;
mod forward_message;
mod get_chat;
mod get_me;
mod get_updates;
mod log_out;
mod send_chat_action;
mod send_dice;
mod send_message;

pub use close::*;
pub use forward_message::*;
pub use get_chat::*;
pub use get_me::*;
pub use get_updates::*;
pub use log_out::*;
pub use send_chat_action::*;
pub use send_dice::*;
pub use send_message::*;

pub trait Request: Serialize {
    const NAME: &'static str;
    const METHOD: Method;
    type Output: DeserializeOwned;
}
