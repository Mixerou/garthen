use actix::{Message, Recipient, WeakAddr};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::WebSocketError;
pub(crate) use crate::messages::data::*;
use crate::server::Socket;

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

// Tag `n` from the word `name`
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "snake_case", tag = "n")]
pub enum DispatchEvent {
    UserUpdate {
        #[serde(skip)]
        id: i64,
    },
    UserMeUpdate {
        #[serde(skip)]
        id: i64,
    },
    GreenhouseUpdate {
        #[serde(skip)]
        id: i64,
    },
    GreenhouseCreate {
        #[serde(skip)]
        id: Option<i64>,
        #[serde(skip)]
        owner_id: i64,
    },
    DeviceUpdate {
        #[serde(skip)]
        id: i64,
    },
    DeviceRecordsUpdate {
        #[serde(skip)]
        device_id: i64,
    },
}

#[derive(Debug, Message)]
#[rtype(result = "Result<(), WebSocketError>")]
pub struct DispatchMessage {
    pub event: DispatchEvent,
    pub new_subscribers: Option<Vec<i64>>,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct DispatchAmqpMessage {
    pub payload: AmqpPayload,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmqpPayload {
    DispatchData {
        device_id: i64,
    },
    RequestData {
        device_id: Option<i64>,
        greenhouse_id: Option<i64>,
    },
    ChangeControllerState {
        device_id: i64,
        state: u8,
    },
    DispatchDevice {
        id: i64,
    },
    Ping,
}

#[derive(Clone, Debug, Message)]
#[rtype(result = "()")]
pub struct InitAmqpConsumersMessage(pub WeakAddr<Socket>);

impl Default for AmqpPayload {
    fn default() -> Self {
        AmqpPayload::Ping
    }
}

#[derive(Clone, Debug, Default, Message)]
#[rtype(result = "()")]
pub struct AmqpPublisherMessage<'a> {
    pub exchange: Option<&'a str>,
    pub routing_key: Option<&'a str>,
    pub payload: AmqpPayload,
}
