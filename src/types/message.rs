use serde::Deserialize;

use crate::types::{
    time::OffsetDateTime, Animation, Audio, AutoDeleteTimerChanged, Chat, Dice, Document,
    InlineKeyboardMarkup, MessageEntity, PhotoSize, User,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(from = "raw::Message")]
pub struct Message {
    pub id: i64,
    pub from: Option<User>,
    pub sender_chat: Option<Chat>,
    pub date: OffsetDateTime,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<i64>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<OffsetDateTime>,
    pub is_automatic_forward: Option<bool>,
    pub reply_to_message: Option<Box<Message>>,
    pub via_bot: Option<User>,
    pub edit_date: Option<OffsetDateTime>,
    pub has_protected_content: Option<bool>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub kind: MessageKind,
}

impl Message {
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.kind.text()
    }

    #[must_use]
    pub fn animation(&self) -> Option<&Animation> {
        self.kind.animation()
    }

    #[must_use]
    pub fn audio(&self) -> Option<&Audio> {
        self.kind.audio()
    }

    #[must_use]
    pub fn photo(&self) -> Option<&[PhotoSize]> {
        self.kind.photo()
    }

    #[must_use]
    pub fn dice(&self) -> Option<&Dice> {
        self.kind.dice()
    }

    #[must_use]
    pub fn entities(&self) -> Option<&[MessageEntity]> {
        self.kind.entities()
    }

    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        self.kind.caption()
    }

    #[must_use]
    pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
        self.kind.caption_entities()
    }
}

// Reference: https://github.com/tdlib/telegram-bot-api/blob/c57b04c4c8c4e8d8bb6fdd0bd3bfb5b93b9d8f05/telegram-bot-api/Client.cpp#L1606
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MessageKind {
    Text {
        text: String,
        entities: Option<Vec<MessageEntity>>,
    },
    Animation {
        animation: Animation,
        document: Document,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Audio {
        audio: Audio,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Document {
        document: Document,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Photo {
        photo: Vec<PhotoSize>,
        caption: Option<String>,
        caption_entities: Option<Vec<MessageEntity>>,
    },
    Dice {
        dice: Dice,
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

impl MessageKind {
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text { text, .. } => Some(text),
            _ => None,
        }
    }

    #[must_use]
    pub fn animation(&self) -> Option<&Animation> {
        match self {
            Self::Animation { animation, .. } => Some(animation),
            _ => None,
        }
    }

    #[must_use]
    pub fn audio(&self) -> Option<&Audio> {
        match self {
            Self::Audio { audio, .. } => Some(audio),
            _ => None,
        }
    }

    #[must_use]
    pub fn document(&self) -> Option<&Document> {
        match self {
            Self::Animation { document, .. } | Self::Document { document, .. } => Some(document),
            _ => None,
        }
    }

    #[must_use]
    pub fn photo(&self) -> Option<&[PhotoSize]> {
        match self {
            Self::Photo { photo, .. } => Some(photo),
            _ => None,
        }
    }

    #[must_use]
    pub fn dice(&self) -> Option<&Dice> {
        match self {
            Self::Dice { dice, .. } => Some(dice),
            _ => None,
        }
    }

    #[must_use]
    pub fn entities(&self) -> Option<&[MessageEntity]> {
        match self {
            Self::Text { entities, .. } => entities.as_deref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Animation { caption, .. }
            | Self::Audio { caption, .. }
            | Self::Photo { caption, .. }
            | Self::Document { caption, .. } => caption.as_deref(),
            _ => None,
        }
    }

    #[must_use]
    pub fn caption_entities(&self) -> Option<&[MessageEntity]> {
        match self {
            Self::Animation {
                caption_entities, ..
            }
            | Self::Audio {
                caption_entities, ..
            }
            | Self::Photo {
                caption_entities, ..
            }
            | Self::Document {
                caption_entities, ..
            } => caption_entities.as_deref(),
            _ => None,
        }
    }
}

mod raw {
    use super::*;

    #[derive(Deserialize)]
    pub struct Message {
        pub message_id: i64,
        pub from: Option<User>,
        pub sender_chat: Option<Chat>,
        #[serde(with = "time::serde::timestamp")]
        pub date: OffsetDateTime,
        pub chat: Chat,
        pub forward_from: Option<User>,
        pub forward_from_chat: Option<Chat>,
        pub forward_from_message_id: Option<i64>,
        pub forward_signature: Option<String>,
        pub forward_sender_name: Option<String>,
        #[serde(default, with = "time::serde::timestamp::option")]
        pub forward_date: Option<OffsetDateTime>,
        pub is_automatic_forward: Option<bool>,
        pub reply_to_message: Option<Box<Message>>,
        pub via_bot: Option<User>,
        #[serde(default, with = "time::serde::timestamp::option")]
        pub edit_date: Option<OffsetDateTime>,
        pub has_protected_content: Option<bool>,
        pub media_group_id: Option<String>,
        pub author_signature: Option<String>,
        pub text: Option<String>,
        pub entities: Option<Vec<MessageEntity>>,
        // TODO: Media messages.
        pub animation: Option<Animation>,
        pub audio: Option<Audio>,
        pub document: Option<Document>,
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
                reply_markup: raw.reply_markup,
                kind: {
                    #[allow(clippy::enum_glob_use)]
                    use MessageKind::*;

                    if let Some(text) = raw.text {
                        Text {
                            text,
                            entities: raw.entities,
                        }
                    } else if let Some(animation) = raw.animation {
                        Animation {
                            animation,
                            // See: https://github.com/tdlib/telegram-bot-api/blob/c57b04c4c8c4e8d8bb6fdd0bd3bfb5b93b9d8f05/telegram-bot-api/Client.cpp#L1672
                            document: raw.document.unwrap(),
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(audio) = raw.audio {
                        Audio {
                            audio,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(document) = raw.document {
                        Document {
                            document,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(photo) = raw.photo {
                        Photo {
                            photo,
                            caption: raw.caption,
                            caption_entities: raw.caption_entities,
                        }
                    } else if let Some(dice) = raw.dice {
                        Dice { dice }
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
