use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::services::greenhouse::Greenhouse;
use crate::services::user::{UserMe, UserPublic, UserTheme};

// Tag `a` from the word `action`
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "a")]
pub enum WebSocketMessageData {
    // Requests (Opcode: Request)
    RequestPatchUser {
        email: String,
        username: String,
        locale: String,
        theme: UserTheme,
        new_password: Option<String>,
        current_password: Option<String>,
    },
    RequestPostGreenhouse {
        name: String,
        token: String,
    },

    // Requests (Opcode: Authorize)
    Authorize { token: String },

    // Requests (Opcode: Subscribe)
    SubscribeToUserUpdate { id: i64 },
    SubscribeToUserMeUpdates {},
    SubscribeToGreenhouseUpdate { id: i64 },

    // Dispatches
    DispatchUserUpdate {
        id: i64,
        username: String,
        created_at: u64,
        greenhouses: i64,
    },
    DispatchUserMeUpdate {
        id: i64,
        email: String,
        username: String,
        created_at: u64,
        locale: String,
        theme: UserTheme,
        greenhouses: i64,
    },
    DispatchGreenhouseMineUpdate {
        id: i64,
        name: String,
        token: String,
        owner_id: i64,
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

impl From<UserPublic> for WebSocketMessageData {
    fn from(user: UserPublic) -> Self {
        WebSocketMessageData::DispatchUserUpdate {
            id: user.id,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            greenhouses: user.greenhouses,
        }
    }
}

impl From<UserMe> for WebSocketMessageData {
    fn from(user: UserMe) -> Self {
        WebSocketMessageData::DispatchUserMeUpdate {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            locale: to_variant_name(&user.locale).unwrap().to_string(),
            theme: user.theme,
            greenhouses: user.greenhouses,
        }
    }
}

impl From<Greenhouse> for WebSocketMessageData {
    fn from(greenhouse: Greenhouse) -> Self {
        WebSocketMessageData::DispatchGreenhouseMineUpdate {
            id: greenhouse.id,
            name: greenhouse.name,
            token: greenhouse.token,
            owner_id: greenhouse.owner_id,
            created_at: greenhouse.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}
