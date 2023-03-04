use std::time::SystemTime;

use db::schema::greenhouses;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::WorkerError;

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
    pub fn find_all() -> Result<Vec<Self>, WorkerError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .load(connection)?;

        Ok(greenhouses)
    }
}
