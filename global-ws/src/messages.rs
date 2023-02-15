use std::time::UNIX_EPOCH;

use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::WebSocketError;
use crate::services::user::User;

#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Dispatch = 0,
    HeartBeat = 1,
    Request = 2,
    Response = 3,
    Error = 4,
    Authorize = 5,
    Subscribe = 6,
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode::Response
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct WebSocketMessageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_me: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct WebSocketMessage {
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(skip)]
    pub connection_id: i64,
    #[serde(rename = "o")]
    pub opcode: Opcode,
    #[serde(rename = "e")]
    pub event: Option<DispatchEvent>,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub data: Option<WebSocketMessageData>,
}

impl From<User> for WebSocketMessageData {
    fn from(user: User) -> Self {
        WebSocketMessageData {
            id: Some(user.id),
            email: Some(user.email),
            username: Some(user.username),
            created_at: Some(user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs()),
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum DispatchEvent {
    UserUpdate { id: i64 },
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct DispatchMessage {
    pub event: DispatchEvent,
    pub new_subscribers: Option<Vec<i64>>,
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct AuthorizationMessage {
    pub id: i64,
    pub connection_id: i64,
    pub token: String,
    pub address: Recipient<WebSocketMessage>,
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct DisconnectionMessage {
    pub connection_id: i64,
}
