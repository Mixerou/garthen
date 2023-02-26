use std::i64;
use std::mem::transmute;
use std::time::SystemTime;

use db::schema::{greenhouses, users};
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

use crate::error::{WebSocketError, WebSocketErrorTemplate};
use crate::services::greenhouse::Greenhouse;

#[derive(Clone, Deserialize, Serialize, Insertable, Queryable, PartialEq)]
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
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(connection)?;

        Ok(user)
    }

    pub fn find_by_email(email: String) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::email.eq(email))
            .first(connection)?;

        Ok(user)
    }

    pub fn find_by_username(username: String) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::username.eq(username))
            .first(connection)?;

        Ok(user)
    }

    pub fn hard_update(
        id: i64,
        new_email: String,
        new_password_hash: String,
        new_username: String,
        new_locale: UserLocale,
        new_theme: UserTheme,
    ) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((
                users::email.eq(new_email),
                users::password_hash.eq(new_password_hash),
                users::username.eq(new_username),
                users::locale.eq(new_locale),
                users::theme.eq(new_theme),
            ))
            .get_result(connection)?;

        Ok(user)
    }

    pub fn soft_update(
        id: i64,
        new_locale: UserLocale,
        new_theme: UserTheme,
    ) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = diesel::update(users::table)
            .filter(users::id.eq(id))
            .set((
                users::locale.eq(new_locale),
                users::theme.eq(new_theme),
            ))
            .get_result(connection)?;

        Ok(user)
    }

    // Default implementations
    pub fn check_email_length(email: &str) -> Result<(), WebSocketError> {
        let email_length = email.chars().count();

        match email_length {
            length if length > 512 => Err(WebSocketErrorTemplate::EmailTooLong(None).into()),
            _ => Ok(())
        }
    }

    pub fn check_password_length(password: &str) -> Result<(), WebSocketError> {
        let password_length = password.chars().count();

        match password_length {
            length if length < 8 => Err(WebSocketErrorTemplate::PasswordTooShort(None).into()),
            length if length > 128 => Err(WebSocketErrorTemplate::PasswordTooLong(None).into()),
            _ => Ok(())
        }
    }

    pub fn check_username_length(username: &str) -> Result<(), WebSocketError> {
        let username_length = username.chars().count();

        match username_length {
            length if length < 3 => Err(WebSocketErrorTemplate::UsernameTooShort(None).into()),
            length if length > 32 => Err(WebSocketErrorTemplate::UsernameTooLong(None).into()),
            _ => Ok(())
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Queryable, PartialEq)]
pub struct UserPublic {
    pub id: i64,
    pub username: String,
    pub created_at: SystemTime,
    pub greenhouses: i64,
}

impl UserPublic {
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let user = User::find(id)?;
        let greenhouses = Greenhouse::count_by_owner_id(id)?;

        Ok(UserPublic {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            greenhouses,
        })
    }
}

impl From<UserMe> for UserPublic {
    fn from(user: UserMe) -> Self {
        UserPublic {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            greenhouses: user.greenhouses,
        }
    }
}

#[derive(PartialEq)]
pub struct UserMe {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub created_at: SystemTime,
    pub locale: UserLocale,
    pub theme: UserTheme,
    pub greenhouses: i64,
}

impl TryFrom<User> for UserMe {
    type Error = WebSocketError;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        Ok(UserMe {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            locale: user.locale,
            theme: user.theme,
            greenhouses: Greenhouse::count_by_owner_id(user.id)?,
        })
    }
}

impl UserMe {
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = User::find(id)?;
        let greenhouses = greenhouses::table
            .filter(greenhouses::owner_id.eq(id))
            .count()
            .get_result(connection)?;

        Ok(UserMe {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at,
            locale: user.locale,
            theme: user.theme,
            greenhouses,
        })
    }
}

#[derive(Copy, Clone, Debug, Deserialize_enum_str, Serialize_enum_str, Eq, PartialEq)]
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

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
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
