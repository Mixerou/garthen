use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::WebSocketError;

#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum Opcode {
    Dispatch = 0,
    HeartBeat = 1,
    Request = 2,
    Response = 3,
    Error = 4,
    Authorize = 5,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WebSocketMessageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct WebSocketMessage {
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(skip)]
    pub connection_id: i64,
    #[serde(rename = "o")]
    pub opcode: Opcode,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(rename = "d", skip_serializing_if = "Option::is_none")]
    pub data: Option<WebSocketMessageData>,
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
