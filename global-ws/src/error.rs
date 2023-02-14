use std::fmt;
use std::io::Error;

use actix::{ActorContext, MailboxError as ActixMailboxError};
use actix_web_actors::ws::{CloseCode, CloseReason, WebsocketContext};
use serde::Deserialize;
use serde_eetf::Error as SerdeEetfError;
use serde_json::Error as SerdeJsonError;

use crate::server::{Socket, WebSocketConnection};

#[derive(Debug)]
pub enum WebSocketErrorKind {
    StdError(Error),
    ActixMailboxError(ActixMailboxError),
    SerdeEetfError(SerdeEetfError),
    SerdeJsonError(SerdeJsonError),
    Other(Option<String>),
}

impl Default for WebSocketErrorKind {
    fn default() -> Self { WebSocketErrorKind::Other(None) }
}

#[derive(Debug, Deserialize)]
pub struct WebSocketError {
    #[serde(skip)]
    pub http_code: u16,
    #[serde(rename(deserialize = "code"))]
    pub json_code: u32,
    pub message: String,
    #[serde(skip)]
    pub kind: WebSocketErrorKind,
}

impl WebSocketError {
    pub fn new(http_code: u16, json_code: Option<u32>, message: String, error: Option<WebSocketErrorKind>) -> WebSocketError {
        WebSocketError {
            http_code,
            json_code: json_code.unwrap_or(http_code as u32),
            message,
            kind: error.unwrap_or(WebSocketErrorKind::Other(None)),
        }
    }

    pub fn get_safe_message(&self) -> String {
        match self.http_code < 500 {
            true => self.message.to_owned(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            }
        }
    }
}

impl From<ActixMailboxError> for WebSocketError {
    fn from(error: ActixMailboxError) -> Self {
        WebSocketError::new(
            500,
            None,
            format!("Actix mailbox error: {error}"),
            Some(WebSocketErrorKind::ActixMailboxError(error)),
        )
    }
}

impl From<Error> for WebSocketError {
    fn from(error: Error) -> WebSocketError {
        WebSocketError::new(
            500,
            None,
            format!("std error: {error}"),
            Some(WebSocketErrorKind::StdError(error)),
        )
    }
}

impl From<SerdeEetfError> for WebSocketError {
    fn from(error: SerdeEetfError) -> Self {
        WebSocketError::new(
            500,
            None,
            format!("Serde EETF error: {error}"),
            Some(WebSocketErrorKind::SerdeEetfError(error)),
        )
    }
}

impl From<SerdeJsonError> for WebSocketError {
    fn from(error: SerdeJsonError) -> Self {
        WebSocketError::new(
            500,
            None,
            format!("Serde JSON error: {error}"),
            Some(WebSocketErrorKind::SerdeJsonError(error)),
        )
    }
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

macro_rules! websocket_error_template {
    ( $( ($http_code:expr, $json_code:expr, $name:ident, $message:expr); )+ ) => {
        pub enum WebSocketErrorTemplate {
        $( $name(Option<WebSocketErrorKind>), )+
        }

        impl From<WebSocketErrorTemplate> for WebSocketError {
            fn from(template: WebSocketErrorTemplate) -> WebSocketError {
                match template {
                $(
                    WebSocketErrorTemplate::$name(error) => {
                        WebSocketError::new($http_code, $json_code, $message.to_string(), error)
                    },
                )+
                }
            }
        }
    }
}

websocket_error_template! {
    (400, None, BadRequest, "Bad request");
}

macro_rules! close_error {
    ( $( ($code:expr, $name:ident, $description:expr); )+ ) => {
        #[allow(dead_code)]
        #[repr(u16)]
        pub enum WebSocketCloseError {
        $( $name = $code, )+
        }

        impl Socket {
            pub fn close_connection(
                error: WebSocketCloseError,
                context: &mut WebsocketContext<WebSocketConnection>,
            ) {
                match error {
                    $(
                        WebSocketCloseError::$name => {
                            let close_reason = CloseReason {
                                code: CloseCode::Other($code),
                                description: Some($description.to_string()),
                            };

                            context.close(Some(close_reason));
                            context.stop();
                        },
                    )+
                }
            }
        }
    }
}

close_error! {
    (4000, Unknown, "Unknown error");
    (4001, Opcode, "Opcode not allowed");
    (4002, InvalidPayload, "Invalid payload");
    (4003, NotAuthenticated, "Not authenticated");
    (4004, AlreadyAuthenticated, "Already authenticated");
}
