use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use serde::Serialize;

use crate::types::errors::TryFromChatActionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordVoice,
    UploadVoice,
    UploadDocument,
    ChooseSticker,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

impl Display for ChatAction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Self::Typing => "typing",
            Self::UploadPhoto => "upload_photo",
            Self::RecordVideo => "record_video",
            Self::UploadVideo => "upload_video",
            Self::RecordVoice => "record_voice",
            Self::UploadVoice => "upload_voice",
            Self::UploadDocument => "upload_document",
            Self::ChooseSticker => "choose_sticker",
            Self::FindLocation => "find_location",
            Self::RecordVideoNote => "record_video_note",
            Self::UploadVideoNote => "upload_video_note",
        })
    }
}

impl FromStr for ChatAction {
    type Err = TryFromChatActionError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "typing" => Ok(Self::Typing),
            "upload_photo" => Ok(Self::UploadPhoto),
            "record_video" => Ok(Self::RecordVideo),
            "upload_video" => Ok(Self::UploadVideo),
            "record_voice" => Ok(Self::RecordVoice),
            "upload_voice" => Ok(Self::UploadVoice),
            "upload_document" => Ok(Self::UploadDocument),
            "choose_sticker" => Ok(Self::ChooseSticker),
            "find_location" => Ok(Self::FindLocation),
            "record_video_note" => Ok(Self::RecordVideoNote),
            "upload_video_note" => Ok(Self::UploadVideoNote),
            _ => Err(TryFromChatActionError),
        }
    }
}
