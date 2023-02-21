use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::WebSocketError;
pub(crate) use crate::messages::data::*;

mod data;

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

#[derive(Debug, Default, Deserialize, Serialize, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct WebSocketMessage {
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(skip)]
    pub connection_id: i64,
    #[serde(rename = "o")]
    pub opcode: Opcode,
    #[serde(rename = "e", skip_serializing_if = "Option::is_none")]
    pub event: Option<DispatchEvent>,
    #[serde(rename = "m", skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
    #[serde(rename = "r", skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    #[serde(
    default = "WebSocketMessageData::default",
    rename = "d",
    skip_serializing_if = "WebSocketMessageData::is_none"
    )]
    pub data: WebSocketMessageData,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum DispatchEvent {
    UserUpdate { id: i64 },
    UserMeUpdate { id: i64 },
    GreenhouseUpdate { id: i64 },
    GreenhouseCreate {
        #[serde(skip)]
        id: Option<i64>,
        owner_id: i64
    },
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
