use std::time::SystemTime;

use db::schema::device_records;
use diesel::{Insertable, Queryable, RunQueryDsl};
use diesel::dsl::avg;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

        let device_records = device_records::table
            .filter(device_records::device_id.eq(device_id))
            .count()
            .get_result(connection)?;

        Ok(device_records)
    }

    pub fn get_average_between_timestamp_by_device_id(
        device_id: i64,
        range: (SystemTime, SystemTime),
    ) -> Result<Option<f64>, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let data = device_records::table
            .select(avg(device_records::data))
            .filter(device_records::device_id.eq(device_id))
            .filter(device_records::created_at.between(range.0, range.1))
            .get_result(connection)?;

        Ok(data)
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceRecordsAverage {
    pub(crate) data: Option<f64>,
    pub(crate) range: (u64, u64),
}

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum DeviceRecordsTimestampRange {
    Today = 0,
    Week = 1,
    Month = 2,
    LastMonth = 3,
    MonthBeforeLast = 4,
    LastThreeMoths = 5,
}

impl Default for DeviceRecordsTimestampRange {
    fn default() -> Self {
        DeviceRecordsTimestampRange::Today
    }
}
