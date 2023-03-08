use std::fmt;
use std::io::Error;

use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use reqwest::Error as ReqwestError;
use serde::Deserialize;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub enum WorkerErrorKind {
    StdError(Error),
    DieselError(DieselError),
    R2d2Error(R2d2Error),
    ReqwestError(ReqwestError),
    SerdeJsonError(SerdeJsonError),
    Other(Option<String>),
}

impl Default for WorkerErrorKind {
    fn default() -> Self { WorkerErrorKind::Other(None) }
}

#[derive(Debug, Deserialize)]
pub struct WorkerError {
    #[serde(rename(deserialize = "code"))]
    pub http_code: u16,
    pub message: String,
    #[serde(skip)]
    pub kind: WorkerErrorKind,
}

impl WorkerError {
    pub fn new(http_code: u16, message: String, error: Option<WorkerErrorKind>) -> WorkerError {
        WorkerError {
            http_code,
            message,
            kind: error.unwrap_or(WorkerErrorKind::Other(None)),
        }
    }
}

impl From<Error> for WorkerError {
    fn from(error: Error) -> WorkerError {
        WorkerError::new(
            500,
            format!("std error: {error}"),
            Some(WorkerErrorKind::StdError(error)),
        )
    }
}

impl From<DieselError> for WorkerError {
    fn from(error: DieselError) -> WorkerError {
        match error {
            DieselError::NotFound => {
                WorkerErrorTemplate::NotFound(Some(WorkerErrorKind::DieselError(error))).into()
            },
            error => {
                WorkerError::new(
                    500,
                    format!("Diesel error: {error}"),
                    Some(WorkerErrorKind::DieselError(error)),
                )
            },
        }
    }
}

impl From<R2d2Error> for WorkerError {
    fn from(error: R2d2Error) -> WorkerError {
        WorkerError::new(
            500,
            format!("r2d2 error: {error}"),
            Some(WorkerErrorKind::R2d2Error(error)),
        )
    }
}

impl From<ReqwestError> for WorkerError {
    fn from(error: ReqwestError) -> Self {
        WorkerError::new(
            500,
            format!("Reqwest error: {error}"),
            Some(WorkerErrorKind::ReqwestError(error)),
        )
    }
}

impl From<SerdeJsonError> for WorkerError {
    fn from(error: SerdeJsonError) -> Self {
        WorkerError::new(
            500,
            format!("Serde JSON error: {error}"),
            Some(WorkerErrorKind::SerdeJsonError(error)),
        )
    }
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

macro_rules! worker_error_template {
    ( $( ($http_code:expr, $name:ident, $message:expr); )+ ) => {
        pub enum WorkerErrorTemplate {
        $( $name(Option<WorkerErrorKind>), )+
        }

        impl From<WorkerErrorTemplate> for WorkerError {
            fn from(template: WorkerErrorTemplate) -> WorkerError {
                match template {
                $(
                    WorkerErrorTemplate::$name(error) => {
                        WorkerError::new($http_code, $message.to_string(), error)
                    },
                )+
                }
            }
        }
    }
}

worker_error_template! {
    (404, NotFound, "Not found");
}
