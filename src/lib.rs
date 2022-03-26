#![warn(clippy::pedantic, clippy::cargo)]

pub mod bot;
mod error;
pub mod methods;
pub mod shared;
pub mod types;

#[doc(inline)]
pub use bot::Bot;
pub use error::Error;
