use std::i64;
use std::mem::transmute;
use std::time::SystemTime;

use db::schema::users;
use diesel::{deserialize, ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::deserialize::FromStaticSqlRow;
use diesel::expression::AsExpression;
use diesel::helper_types::AsExprOf;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::row::Row;
use diesel::sql_types::{SmallInt, VarChar};
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_variant::to_variant_name;

use crate::error::{ApiError, ApiErrorTemplate};

#[derive(Clone, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub created_at: SystemTime,
    pub locale: UserLocale,
    pub theme: UserTheme,
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
            locale: user.locale,
            theme: user.theme,
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
    pub locale: UserLocale,
    pub theme: UserTheme,
}

#[derive(Copy, Clone, Deserialize_enum_str, Serialize_enum_str, Eq, PartialEq)]
pub enum UserLocale {
    #[serde(alias = "en", rename = "en-GB")]
    EnGb,
    #[serde(alias = "ru", rename="ru-RU")]
    Ru,
}

impl FromStaticSqlRow<VarChar, Pg> for UserLocale {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> deserialize::Result<Self> {
        Ok(String::build_from_row(row)?.parse::<UserLocale>()?)
    }
}

impl AsExpression<VarChar> for UserLocale {
    type Expression = AsExprOf<String, VarChar>;

    fn as_expression(self) -> Self::Expression {
        // TODO: Delete `unwrap`
        <String as AsExpression<VarChar>>::as_expression(to_variant_name(&self).unwrap().to_string())
    }
}

impl<'a> AsExpression<VarChar> for &'a UserLocale {
    type Expression = AsExprOf<String, VarChar>;

    fn as_expression(self) -> Self::Expression {
        // TODO: Delete `unwrap`
        <String as AsExpression<VarChar>>::as_expression(to_variant_name(&self).unwrap().to_string())
    }
}

impl Queryable<VarChar, Pg> for UserLocale {
    type Row = String;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(row.parse::<UserLocale>()?)
    }
}

#[derive(Copy, Clone, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[repr(i16)]
pub enum UserTheme {
    Auto = 0,
    Light = 1,
    Dark = 2,
}

impl FromStaticSqlRow<SmallInt, Pg> for UserTheme {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(i16::build_from_row(row)?) })
    }
}

impl AsExpression<SmallInt> for UserTheme {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        // TODO: Delete `unwrap`
        <i16 as AsExpression<SmallInt>>::as_expression(self as i16)
    }
}

impl<'a> AsExpression<SmallInt> for &'a UserTheme {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        // TODO: Delete `unwrap`
        <i16 as AsExpression<SmallInt>>::as_expression(*self as i16)
    }
}

impl Queryable<SmallInt, Pg> for UserTheme {
    type Row = i16;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(row) })
    }
}
