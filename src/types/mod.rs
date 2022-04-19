//! Available types.

use serde::Deserialize;

pub extern crate time;

mod allowed_update;
mod auto_delete_timer_changed;
mod chat;
mod chat_action;
mod chat_id;
mod dice;
pub mod errors;
mod force_reply;
mod inline_keyboard_button;
mod inline_keyboard_markup;
mod keyboard_button;
mod message;
mod message_entity;
mod message_id;
mod parse_mode;
mod reply_keyboard_markup;
mod reply_keyboard_remove;
mod reply_markup;
#[path = "true.rs"]
mod true_;
mod update;
mod user;

pub use allowed_update::*;
pub use auto_delete_timer_changed::*;
pub use chat::*;
pub use chat_action::*;
pub use chat_id::*;
pub use dice::*;
pub use force_reply::*;
pub use inline_keyboard_button::*;
pub use inline_keyboard_markup::*;
pub use keyboard_button::*;
pub use message::*;
pub use message_entity::*;
pub use message_id::*;
pub use parse_mode::*;
pub use reply_keyboard_markup::*;
pub use reply_keyboard_remove::*;
pub use reply_markup::*;
pub use true_::*;
pub use update::*;
pub use user::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct ResponseParameters {
    pub migrate_to_chat_id: i64,
    pub retry_after: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct PhotoSize {
    #[serde(rename = "file_id")]
    pub id: String,
    #[serde(rename = "file_unique_id")]
    pub unique_id: String,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "file_size")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Animation {
    #[serde(rename = "file_id")]
    pub id: String,
    #[serde(rename = "file_unique_id")]
    pub unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    #[serde(rename = "file_name")]
    pub name: Option<String>,
    pub mime_type: Option<String>,
    #[serde(rename = "file_size")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Audio {
    #[serde(rename = "file_id")]
    pub id: String,
    #[serde(rename = "file_unique_id")]
    pub unique_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "file_name")]
    pub name: Option<String>,
    pub mime_type: Option<String>,
    #[serde(rename = "file_size")]
    pub size: Option<i32>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct Document {
    #[serde(rename = "file_id")]
    pub id: String,
    #[serde(rename = "file_unique_id")]
    pub unique_id: String,
    pub thumb: Option<PhotoSize>,
    #[serde(rename = "file_name")]
    pub name: Option<String>,
    pub mime_type: Option<String>,
    #[serde(rename = "file_size")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Deserialize)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub horizontal_accuracy: f64,
    pub live_period: Option<i32>,
    pub heading: Option<i16>,
    pub proximity_alert_radius: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
    pub foursquare_type: Option<String>,
    pub google_place_id: Option<String>,
    pub google_place_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct InlineQuery {
    pub id: String,
    pub from: User,
    pub query: String,
    pub offset: String,
    // TODO: Extend ChatKind?
    pub chat_type: Option<String>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChosenInlineResult {
    #[serde(rename = "result_id")]
    pub id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<i64>,
    pub query: String,
}
