use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::types::{
    Animation, Audio, AutoDeleteTimerChanged, Chat, Dice, InlineKeyboardMarkup, MessageEntity,
    PhotoSize, User,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::Message")]
pub struct Message {
    pub id: i32,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: DateTime<Utc>,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<DateTime<Utc>>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<DateTime<Utc>>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub kind: MessageKind,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageKind {
    Text {
        text: String,
        entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    Animation {
        animation: Animation,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    Audio {
        audio: Audio,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    Photo {
        photo: Vec<PhotoSize>,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    Dice {
        dice: Dice,
        reply_markup: Option<InlineKeyboardMarkup>,
    },
    NewChatMembers(Vec<User>),
    LeftChatMember(User),
    NewChatTitle(String),
    NewChatPhoto(Vec<PhotoSize>),
    DeleteChatPhoto,
    GroupChatCreated,
    SupergroupChatCreated,
    ChannelChatCreated,
    AutoDeleteTimerChanged(AutoDeleteTimerChanged),
    PinnedMessage(Box<Message>),
    // TODO
}

mod raw {
    use crate::types::Animation;

    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Deserialize)]
    pub struct Message {
        pub message_id: i32,
        pub from: Option<User>,
        pub sender_chat: Option<Chat>,
        #[serde(with = "chrono::serde::ts_seconds")]
        pub date: DateTime<Utc>,
        pub chat: Chat,
        pub forward_from: Option<User>,
        pub forward_from_chat: Option<Chat>,
        pub forward_from_message_id: Option<i32>,
        pub forward_signature: Option<String>,
        pub forward_sender_name: Option<String>,
        #[serde(default, with = "chrono::serde::ts_seconds_option")]
        pub forward_date: Option<DateTime<Utc>>,
        pub is_automatic_forward: Option<bool>,
        pub reply_to_message: Option<Box<Message>>,
        pub via_bot: Option<User>,
        #[serde(default, with = "chrono::serde::ts_seconds_option")]
        pub edit_date: Option<DateTime<Utc>>,
        pub has_protected_content: Option<bool>,
        pub media_group_id: Option<String>,
        pub author_signature: Option<String>,
        pub text: Option<String>,
        pub entities: Option<Vec<MessageEntity>>,
        // TODO: Media messages.
        pub animation: Option<Animation>,
        pub audio: Option<Audio>,
        pub photo: Option<Vec<PhotoSize>>,
        // TODO: Media messages.
        pub caption: Option<String>,
        pub caption_entities: Option<Vec<MessageEntity>>,
        pub dice: Option<Dice>,
        pub new_chat_members: Option<Vec<User>>,
        pub left_chat_member: Option<User>,
        pub new_chat_title: Option<String>,
        pub new_chat_photo: Option<Vec<PhotoSize>>,
        pub delete_chat_photo: Option<bool>,
        pub group_chat_created: Option<bool>,
        pub supergroup_chat_created: Option<bool>,
        pub channel_chat_created: Option<bool>,
        #[serde(rename = "message_auto_deleted_timer_changed")]
        pub auto_deleted_timer_changed: Option<AutoDeleteTimerChanged>,
        // TODO: Service messages.
        pub migrate_to_chat_id: Option<i64>,
        pub pinned_message: Option<Box<Message>>,
        // TODO: Media messages.
        pub reply_markup: Option<InlineKeyboardMarkup>,
    }

    impl From<Box<Message>> for Box<super::Message> {
        fn from(boxed_raw: Box<Message>) -> Self {
            Box::new(super::Message::from(*boxed_raw))
        }
    }

    impl From<Message> for super::Message {
        fn from(raw: Message) -> Self {
            Self {
                id: raw.message_id,
                from: raw.from,
                sender_chat: raw.sender_chat,
                date: raw.date,
                chat: raw.chat,
                forward_from: raw.forward_from,
                forward_from_chat: raw.forward_from_chat,
                forward_from_message_id: raw.forward_from_message_id,
                forward_signature: raw.forward_signature,
                forward_sender_name: raw.forward_sender_name,
                forward_date: raw.forward_date,
                is_automatic_forward: raw.is_automatic_forward,
                reply_to_message: raw.reply_to_message.map(Into::into),
                via_bot: raw.via_bot,
                edit_date: raw.edit_date,
                has_protected_content: raw.has_protected_content,
                media_group_id: raw.media_group_id,
                author_signature: raw.author_signature,
                kind: {
                    #[allow(clippy::enum_glob_use)]
                    use MessageKind::*;

                    if let Some(text) = raw.text {
                        Text {
                            text,
                            entities: raw.entities,
                            reply_markup: raw.reply_markup,
                        }
                    } else if let Some(animation) = raw.animation {
                        Animation {
                            animation,
                            reply_markup: raw.reply_markup,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(audio) = raw.audio {
                        Audio {
                            audio,
                            reply_markup: raw.reply_markup,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(photo) = raw.photo {
                        Photo {
                            photo,
                            reply_markup: raw.reply_markup,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(dice) = raw.dice {
                        Dice {
                            dice,
                            reply_markup: raw.reply_markup,
                        }
                    } else if let Some(new_chat_members) = raw.new_chat_members {
                        NewChatMembers(new_chat_members)
                    } else if let Some(left_chat_member) = raw.left_chat_member {
                        LeftChatMember(left_chat_member)
                    } else if let Some(new_chat_title) = raw.new_chat_title {
                        NewChatTitle(new_chat_title)
                    } else if let Some(new_chat_photo) = raw.new_chat_photo {
                        NewChatPhoto(new_chat_photo)
                    } else if raw.delete_chat_photo.is_some() {
                        DeleteChatPhoto
                    } else if raw.group_chat_created.is_some() {
                        GroupChatCreated
                    } else if raw.supergroup_chat_created.is_some() {
                        SupergroupChatCreated
                    } else if raw.channel_chat_created.is_some() {
                        ChannelChatCreated
                    } else if let Some(auto_delete_timer_changed) = raw.auto_deleted_timer_changed {
                        AutoDeleteTimerChanged(auto_delete_timer_changed)
                    } else if let Some(pinned_message) = raw.pinned_message {
                        PinnedMessage(pinned_message.into())
                    } else {
                        todo!()
                    }
                },
            }
        }
    }
}
