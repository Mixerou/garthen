use std::time::SystemTime;

use db::schema::device_records;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::WebSocketError;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = device_records)]
pub struct DeviceRecord {
    pub id: i64,
    pub device_id: i64,
    pub data: f64,
    pub created_at: SystemTime,
}

impl DeviceRecord {
    pub fn find_latest_by_device_id(device_id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let device_record = device_records::table
            .filter(device_records::device_id.eq(device_id))
            .order(device_records::created_at.desc())
            .first(connection)?;

        Ok(device_record)
    }
}
