use std::time::SystemTime;

use db::schema::greenhouses;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::WebSocketError;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = greenhouses)]
pub struct Greenhouse {
    pub id: i64,
    pub name: String,
    pub token: String,
    pub owner_id: i64,
    pub created_at: SystemTime,
}

impl Greenhouse {
    pub fn create(greenhouse: NewGreenhouse) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = Greenhouse {
            id: snowflake::generate(),
            name: greenhouse.name,
            token: greenhouse.token,
            owner_id: greenhouse.owner_id,
            created_at: SystemTime::now(),
        };

        let session = diesel::insert_into(greenhouses::table)
            .values(greenhouse)
            .get_result(connection)?;

        Ok(session)
    }

    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::id.eq(id))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_by_token(token: String) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::token.eq(token))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_all_by_owner_id(owner_id: i64) -> Result<Vec<Self>, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .filter(greenhouses::owner_id.eq(owner_id))
            .load(connection)?;

        Ok(greenhouses)
    }

    pub fn count_by_owner_id(owner_id: i64) -> Result<i64, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .filter(greenhouses::owner_id.eq(owner_id))
            .count()
            .get_result(connection)?;

        Ok(greenhouses)
    }
}

pub struct NewGreenhouse {
    pub name: String,
    pub token: String,
    pub owner_id: i64,
}
