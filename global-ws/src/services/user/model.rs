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

use crate::error::WebSocketError;
use crate::services::greenhouse::Greenhouse;

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
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let user = users::table
            .filter(users::id.eq(id))
            .first(connection)?;

        Ok(user)
    }
}

#[derive(Clone, Deserialize, Serialize, Queryable)]
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

pub struct UserMe {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub created_at: SystemTime,
    pub locale: UserLocale,
    pub theme: UserTheme,
    pub greenhouses: i64,
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
