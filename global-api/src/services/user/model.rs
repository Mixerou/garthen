use std::i64;
use std::time::SystemTime;

use db::schema::users;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiErrorTemplate};

#[derive(Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub created_at: SystemTime,
}

impl User {
    // CRUD
    pub fn create(user: NewUser) -> Result<Self, ApiError> {
        let connection = &mut db::get_connection()?;

        let user = User {
            id: snowflake::generate(),
            email: user.email,
            password_hash: user.password_hash,
            username: user.username,
            created_at: SystemTime::now(),
        };

        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(connection)?;

        Ok(user)
    }

    pub fn find_by_email(email: String) -> Result<Self, ApiError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::email.eq(email))
            .first(connection)?;

        Ok(user)
    }

    pub fn find_by_username(username: String) -> Result<Self, ApiError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::username.eq(username))
            .first(connection)?;

        Ok(user)
    }

    // Default implementations
    pub fn check_email_length(email: &str) -> Result<(), ApiError> {
        let email_length = email.chars().count();

        match email_length {
            length if length > 512 => Err(ApiErrorTemplate::EmailTooLong(None).into()),
            _ => Ok(())
        }
    }

    pub fn check_password_length(password: &str) -> Result<(), ApiError> {
        let password_length = password.chars().count();

        match password_length {
            length if length < 8 => Err(ApiErrorTemplate::PasswordTooShort(None).into()),
            length if length > 128 => Err(ApiErrorTemplate::PasswordTooLong(None).into()),
            _ => Ok(())
        }
    }

    pub fn check_username_length(username: &str) -> Result<(), ApiError> {
        let username_length = username.chars().count();

        match username_length {
            length if length < 3 => Err(ApiErrorTemplate::UsernameTooShort(None).into()),
            length if length > 32 => Err(ApiErrorTemplate::UsernameTooLong(None).into()),
            _ => Ok(())
        }
    }
}

#[derive(Deserialize, Serialize, Queryable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: String,
    pub username: String,
}
