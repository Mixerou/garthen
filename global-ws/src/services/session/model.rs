use std::time::SystemTime;

use db::schema::sessions;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::WebSocketError;

#[derive(Clone, Deserialize, Insertable, Serialize, Queryable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: Option<i64>,
    pub created_at: SystemTime,
}

impl Session {
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let session = sessions::table
            .filter(sessions::id.eq(id))
            .first(connection)?;

        Ok(session)
    }

    pub fn find_by_token(token: String) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let session = sessions::table
            .filter(sessions::token.eq(token))
            .first(connection)?;

        Ok(session)
    }
}
