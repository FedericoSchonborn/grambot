#![warn(clippy::pedantic, clippy::cargo)]
// Remove these when documentation is done.
#![allow(clippy::missing_errors_doc)]
// Do NOT remove these.
#![allow(
    clippy::module_name_repetitions,
    clippy::multiple_crate_versions,
    clippy::wildcard_imports
)]

pub mod bot;
mod error;
pub mod methods;
pub mod types;

#[doc(inline)]
pub use bot::Bot;
pub use error::Error;
