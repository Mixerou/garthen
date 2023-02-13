use std::fmt;
use std::io::Error;
use std::string::ToString;

use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode as HttpStatusCode;
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use serde::Deserialize;
use serde_json::json;

const UNKNOWN_JSON_ERROR_CODE: u32 = 0;

#[derive(Debug)]
pub enum ApiErrorKind {
    StdError(Error),
    DieselError(DieselError),
    R2d2Error(R2d2Error),
    Other(Option<String>),
}

impl Default for ApiErrorKind {
    fn default() -> Self { ApiErrorKind::Other(None) }
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    #[serde(skip)]
    pub http_code: u16,
    #[serde(rename(deserialize = "code"))]
    pub json_code: u32,
    pub message: String,
    #[serde(skip)]
    pub kind: ApiErrorKind,
}

impl ApiError {
    pub fn new(http_code: u16, json_code: Option<u32>, message: String, error: Option<ApiErrorKind>) -> ApiError {
        ApiError {
            http_code,
            json_code: json_code.unwrap_or(UNKNOWN_JSON_ERROR_CODE),
            message,
            kind: error.unwrap_or(ApiErrorKind::Other(None)),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<Error> for ApiError {
    fn from(error: Error) -> ApiError {
        ApiError::new(
            500,
            None,
            format!("std error: {error}"),
            Some(ApiErrorKind::StdError(error)),
        )
    }
}

impl From<R2d2Error> for ApiError {
    fn from(error: R2d2Error) -> ApiError {
        ApiError::new(
            500,
            None,
            format!("r2d2 error: {error}"),
            Some(ApiErrorKind::R2d2Error(error)),
        )
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        match error {
            DieselError::NotFound => {
                ApiErrorTemplate::NotFound(Some(ApiErrorKind::DieselError(error))).into()
            },
            error => {
                ApiError::new(
                    500,
                    None,
                    format!("Diesel error: {error}"),
                    Some(ApiErrorKind::DieselError(error)),
                )
            },
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match HttpStatusCode::from_u16(self.http_code) {
            Ok(status_code) => status_code,
            Err(_) => HttpStatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.to_owned(),
            false => {
                error!("{}", self.message);
                "Internal server error".to_string()
            },
        };

        HttpResponse::build(status_code).json(json!({
            "code": self.json_code,
            "message": message
        }))
    }
}

macro_rules! api_error_template {
    ( $( ($http_code:expr, $json_code:expr, $name:ident, $message:expr); )+ ) => {
        pub enum ApiErrorTemplate {
        $( $name(Option<ApiErrorKind>), )+
        }

        impl From<ApiErrorTemplate> for ApiError {
            fn from(template: ApiErrorTemplate) -> ApiError {
                match template {
                $(
                    ApiErrorTemplate::$name(error) => {
                        ApiError::new($http_code, $json_code, $message.to_string(), error)
                    },
                )+
                }
            }
        }
    }
}

api_error_template! {
    (400, None, NotFound, "Not found");
}
