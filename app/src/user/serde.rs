use super::field::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SerializableUser {
    pub version: u8,
    pub show_player_id: bool,
    pub self_intro: String,
    pub avatar: String,
    pub background: String,
}

impl From<User> for SerializableUser {
    fn from(user: User) -> Self {
        SerializableUser {
            version: user.version,
            show_player_id: user.show_player_id,
            self_intro: user.self_intro.into(),
            avatar: user.avatar.into(),
            background: user.background.into(),
        }
    }
}

impl From<SerializableUser> for User {
    fn from(su: SerializableUser) -> Self {
        User {
            version: su.version,
            show_player_id: su.show_player_id,
            self_intro: su.self_intro.into(),
            avatar: su.avatar.into(),
            background: su.background.into(),
        }
    }
}
