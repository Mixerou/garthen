use std::time::SystemTime;

use db::schema::greenhouses;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
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
    pub maximum_average_humidity: Option<f64>,
    pub minimum_average_temperature: Option<f64>,
}

impl Greenhouse {
    pub fn find(id: i64) -> Result<Self, WorkerError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::id.eq(id))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_all() -> Result<Vec<Self>, WorkerError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .load(connection)?;

        Ok(greenhouses)
    }
}
