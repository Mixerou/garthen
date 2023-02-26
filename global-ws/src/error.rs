use std::fmt;
use std::io::Error;

use actix::{ActorContext, MailboxError as ActixMailboxError};
use actix_web_actors::ws::{CloseCode, CloseReason, WebsocketContext};
use argon2::Error as Argon2Error;
use argon2::password_hash::Error as Argon2PasswordHashError;
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use serde::Deserialize;
use serde_eetf::Error as SerdeEetfError;
use serde_json::Error as SerdeJsonError;

use crate::server::{Socket, WebSocketConnection};

#[derive(Debug)]
pub enum WebSocketErrorKind {
    StdError(Error),
    ActixMailboxError(ActixMailboxError),
    Argon2Error(Argon2Error),
    Argon2PasswordHashError(Argon2PasswordHashError),
    DieselError(DieselError),
    R2d2Error(R2d2Error),
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

impl From<Argon2Error> for WebSocketError {
    fn from(error: Argon2Error) -> WebSocketError {
        WebSocketError::new(
            500,
            None,
            format!("Argon2 error: {error}"),
            Some(WebSocketErrorKind::Argon2Error(error)),
        )
    }
}

impl From<Argon2PasswordHashError> for WebSocketError {
    fn from(error: Argon2PasswordHashError) -> WebSocketError {
        WebSocketError::new(
            500,
            None,
            format!("Argon2 password hash error: {error}"),
            Some(WebSocketErrorKind::Argon2PasswordHashError(error)),
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

impl From<DieselError> for WebSocketError {
    fn from(error: DieselError) -> WebSocketError {
        match error {
            DieselError::NotFound => {
                WebSocketErrorTemplate::NotFound(Some(WebSocketErrorKind::DieselError(error))).into()
            },
            error => {
                WebSocketError::new(
                    500,
                    None,
                    format!("Diesel error: {error}"),
                    Some(WebSocketErrorKind::DieselError(error)),
                )
            },
        }
    }
}

impl From<R2d2Error> for WebSocketError {
    fn from(error: R2d2Error) -> WebSocketError {
        WebSocketError::new(
            500,
            None,
            format!("r2d2 error: {error}"),
            Some(WebSocketErrorKind::R2d2Error(error)),
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
    // Default HTTP errors
    (400, None, BadRequest, "Bad request");
    (401, None, Unauthorized, "Unauthorized");
    (403, None, Forbidden, "Forbidden");
    (404, None, NotFound, "Not found");
    (405, None, MethodNotAllowed, "Method not allowed");

    // Minimum / Maximum number of ... reached
    (400, Some(30001), GreenhousesTooMany, "There are too many greenhouses");
    (400, Some(30002), GreenhouseNameTooShort, "Greenhouse name is too short");
    (400, Some(30003), GreenhouseNameTooLong, "Greenhouse name is too long");
    (400, Some(30004), GreenhouseTokenTooShort, "Greenhouse token is too short");
    (400, Some(30005), GreenhouseTokenTooLong, "Greenhouse token is too long");
    (400, Some(30006), EmailTooLong, "The email address is too long");
    (400, Some(30007), PasswordTooShort, "The password is too short");
    (400, Some(30008), PasswordTooLong, "The password is too long");
    (400, Some(30009), UsernameTooShort, "The username is too short");
    (400, Some(30010), UsernameTooLong, "The username is too long");

    // Invalid body or something else
    (400, Some(40001), InvalidRequestField, "Invalid request");
    (400, Some(40002), GreenhouseTokenTaken, "Greenhouse token taken");
    (400, Some(40003), EmailInvalid, "Invalid email");
    (400, Some(40004), IncorrectPassword, "Incorrect password");
    (400, Some(40005), UsernameInvalidOrTaken, "The username is either invalid or taken");
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
    (4004, AuthenticationFailed, "Authentication failed");
    (4005, AlreadyAuthenticated, "Already authenticated");
}
