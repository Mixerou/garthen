use std::time::SystemTime;

use db::schema::device_records;
use diesel::{Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::error::WorkerError;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = device_records)]
pub struct DeviceRecord {
    pub id: i64,
    pub device_id: i64,
    pub data: f64,
    pub created_at: SystemTime,
}

impl DeviceRecord {
    pub fn create(device_record: NewDeviceRecord) -> Result<Self, WorkerError> {
        let connection = &mut db::get_connection()?;

        let device_record = DeviceRecord {
            id: snowflake::generate(),
            device_id: device_record.device_id,
            data: device_record.data,
            created_at: SystemTime::now(),
        };

        let device_record = diesel::insert_into(device_records::table)
            .values(device_record)
            .get_result(connection)?;

        Ok(device_record)
    }
}

pub struct NewDeviceRecord {
    pub device_id: i64,
    pub data: f64,
}
