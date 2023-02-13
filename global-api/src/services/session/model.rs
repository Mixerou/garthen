use std::time::SystemTime;

use db::schema::sessions;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

#[derive(Clone, Deserialize, Insertable, Serialize, Queryable)]
#[diesel(table_name = sessions)]
pub struct Session {
    pub id: i64,
    pub token: String,
    pub user_id: Option<i64>,
    pub created_at: SystemTime,
}

impl Session {
    pub fn create() -> Result<Self, ApiError> {
        let connection = &mut db::get_connection()?;

        let session = Session {
            id: snowflake::generate(),
            token: format!("{}{}", nanoid!(45), snowflake::generate()),
            user_id: None,
            created_at: SystemTime::now(),
        };

        let session = diesel::insert_into(sessions::table)
            .values(session)
            .get_result(connection)?;

        Ok(session)
    }

    pub fn find_by_token(token: String) -> Result<Self, ApiError> {
        let connection = &mut db::get_connection()?;

        let session = sessions::table
            .filter(sessions::token.eq(token))
            .first(connection)?;

        Ok(session)
    }
}
