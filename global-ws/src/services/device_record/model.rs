use std::time::SystemTime;

use db::schema::device_records;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::{WebSocketError, WebSocketErrorTemplate};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = device_records)]
pub struct DeviceRecord {
    pub id: i64,
    pub device_id: i64,
    pub data: f64,
    pub created_at: SystemTime,
}

impl DeviceRecord {
    // CRUD
    pub fn create_with_custom_time(
        device_record: NewDeviceRecord,
        time: SystemTime,
    ) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let device_record = DeviceRecord {
            id: snowflake::generate(),
            device_id: device_record.device_id,
            data: device_record.data,
            created_at: time,
        };

        let device_record = diesel::insert_into(device_records::table)
            .values(device_record)
            .get_result(connection)?;

        Ok(device_record)
    }

    pub fn find_latest_by_device_id(device_id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let device_record = device_records::table
            .filter(device_records::device_id.eq(device_id))
            .order(device_records::created_at.desc())
            .first(connection)?;

        Ok(device_record)
    }

    pub fn count_by_device_id(device_id: i64) -> Result<i64, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = device_records::table
            .filter(device_records::device_id.eq(device_id))
            .count()
            .get_result(connection)?;

        Ok(greenhouses)
    }

    // Default implementations
    pub fn check_data_size(data: &f64) -> Result<(), WebSocketError> {
        match data {
            size if size < &-100.0 => Err(
                WebSocketErrorTemplate::DeviceRecordDataTooSmall(None).into()
            ),
            size if size > &100.0 => Err(
                WebSocketErrorTemplate::DeviceRecordDataTooBig(None).into()
            ),
            _ => Ok(())
        }
    }
}

pub struct NewDeviceRecord {
    pub device_id: i64,
    pub data: f64,
}
