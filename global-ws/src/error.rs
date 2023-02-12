use std::fmt;

use actix::MailboxError as ActixMailboxError;
use serde::Deserialize;
use serde_eetf::Error as SerdeEetfError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum WebSocketErrorField {
    ActixMailboxError(ActixMailboxError),
    SerdeEetfError(SerdeEetfError),
    SerdeJsonError(SerdeJsonError),
    Other(Option<String>),
}

impl Default for WebSocketErrorField {
    fn default() -> Self { WebSocketErrorField::Other(None) }
}

#[derive(Debug, Deserialize)]
pub struct WebSocketError {
    pub status_code: u16,
    pub message: String,
    #[serde(skip)]
    pub error: WebSocketErrorField,
}

impl WebSocketError {
    pub fn new(status_code: u16, message: String, error: Option<WebSocketErrorField>) -> WebSocketError {
        WebSocketError {
            status_code,
            message,
            error: error.unwrap_or(WebSocketErrorField::Other(None)),
        }
    }

    pub fn get_safe_message(&self) -> String {
        match self.status_code < 500 {
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
            format!("Actix mailbox error: {error}"),
            Some(WebSocketErrorField::ActixMailboxError(error)),
        )
    }
}

impl From<SerdeEetfError> for WebSocketError {
    fn from(error: SerdeEetfError) -> Self {
        WebSocketError::new(
            500,
            format!("Serde EETF error: {error}"),
            Some(WebSocketErrorField::SerdeEetfError(error)),
        )
    }
}

impl From<SerdeJsonError> for WebSocketError {
    fn from(error: SerdeJsonError) -> Self {
        WebSocketError::new(
            500,
            format!("Serde JSON error: {error}"),
            Some(WebSocketErrorField::SerdeJsonError(error)),
        )
    }
}

impl fmt::Display for WebSocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}
