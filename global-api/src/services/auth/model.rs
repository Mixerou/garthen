use std::io::ErrorKind;

use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiErrorKind, ApiErrorTemplate};
use crate::services::session::Session;
use crate::services::user::{NewUser, User};
use crate::utils::dns;

pub struct Auth;

impl Auth {
    pub fn register(credentials: RegistrationRequest, session_id: i64) -> Result<(), ApiError> {
        User::check_email_length(&credentials.email)?;
        User::check_password_length(&credentials.password)?;
        User::check_username_length(&credentials.username)?;

        let email_domain: Vec<&str> = credentials.email.split('@').collect();

        if email_domain.len() != 2 {
            return Err(ApiErrorTemplate::EmailInvalid(None).into());
        }

        let domain_mx_records = match dns::get_mx_records(email_domain[1]) {
            Ok(records) => records,
            Err(error) => {
                if let ApiErrorKind::StdError(std_error) = &error.kind {
                    if std_error.kind() == ErrorKind::InvalidData {
                        return Err(ApiErrorTemplate::EmailInvalid(None).into());
                    }
                }

                return Err(error);
            }
        };

        if domain_mx_records.is_empty() {
            return Err(ApiErrorTemplate::EmailInvalid(None).into());
        }

        if User::find_by_username(credentials.username.to_owned()).is_ok() {
            return Err(ApiErrorTemplate::UsernameInvalidOrTaken(None).into());
        }

        let user = User::find_by_email(credentials.email.to_owned());

        let user = match user {
            Ok(_) => return Err(ApiErrorTemplate::EmailInvalid(None).into()),
            Err(error) => {
                match error.http_code {
                    404 => {
                        let password_hash = passwd::hash(credentials.password)?;

                        User::create(NewUser {
                            email: credentials.email,
                            password_hash,
                            username: credentials.username,
                        })?
                    }
                    _ => return Err(error),
                }
            }
        };

        Session::update_user_id(session_id, Some(user.id))?;

        Ok(())
    }

    pub fn login(credentials: LoginRequest, session_id: i64) -> Result<(), ApiError> {
        User::check_email_length(&credentials.email)?;
        User::check_password_length(&credentials.password)?;

        let user = User::find_by_email(credentials.email)?;

        if passwd::verify(credentials.password, user.password_hash).is_err() {
            return Err(ApiErrorTemplate::NotFound(None).into());
        }

        Session::update_user_id(session_id, Some(user.id))?;

        Ok(())
    }

    pub fn logout(session_id: i64) -> Result<(), ApiError> {
        Session::update_user_id(session_id, None)?;

        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
pub struct RegistrationRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
