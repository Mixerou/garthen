use std::i64;
use std::time::SystemTime;

use db::schema::users;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::WebSocketError;

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

pub struct UserMe {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub username: String,
    pub created_at: SystemTime,
}

impl From<User> for UserMe {
    fn from(user: User) -> Self {
        UserMe {
            id: user.id,
            email: user.email,
            password_hash: user.password_hash,
            username: user.username,
            created_at: user.created_at,
        }
    }
}