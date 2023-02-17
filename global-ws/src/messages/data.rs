use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};

use crate::services::user::{User, UserMe};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WebSocketMessageData {
    // Requests (Opcode: Authorize)
    Authorize {
        token: String,
    },

    // Requests (Opcode: Subscribe)
    SubscribeToUserUpdates {
        id: Option<i64>,
        is_me: Option<bool>,
    },
    SubscribeToUserMeUpdates {},

    // Dispatches
    DispatchUser {
        id: i64,
        username: String,
        created_at: u64,
    },
    DispatchUserMe {
        id: i64,
        email: String,
        username: String,
        created_at: u64,
    },

    // Other
    Response {
        code: u32,
        message: String,
    },
    None,
}

impl WebSocketMessageData {
    pub fn is_none(&self) -> bool {
        matches!(self, WebSocketMessageData::None)
    }
}

impl Default for WebSocketMessageData {
    fn default() -> Self {
        WebSocketMessageData::None
    }
}

impl From<User> for WebSocketMessageData {
    fn from(user: User) -> Self {
        WebSocketMessageData::DispatchUser {
            id: user.id,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl From<UserMe> for WebSocketMessageData {
    fn from(user: UserMe) -> Self {
        WebSocketMessageData::DispatchUserMe {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}
