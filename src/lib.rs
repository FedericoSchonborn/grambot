#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

pub mod bot;
mod error;
pub mod methods;
pub mod types;

#[doc(inline)]
pub use bot::Bot;
pub use error::Error;
