use std::i64;
use std::time::SystemTime;

use db::schema::{greenhouses, users};
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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
            greenhouses,
        })
    }
}
