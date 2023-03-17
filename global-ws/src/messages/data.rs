use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use serde_variant::to_variant_name;

use crate::services::device::{Device, DeviceKind, DeviceStatus};
use crate::services::device_record::{DeviceRecord, DeviceRecordsAverage, DeviceRecordsTimestampRange};
use crate::services::greenhouse::Greenhouse;
use crate::services::user::{UserMe, UserPublic, UserTheme};

// Tag `a` from the word `action`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "a")]
pub enum WebSocketMessageData {
    // Requests (Opcode: Request)
    RequestPatchUser {
        email: String,
        username: String,
        locale: String,
        theme: UserTheme,
        new_password: Option<String>,
        current_password: Option<String>,
    },
    RequestPostGreenhouse {
        name: String,
        token: String,
    },
    RequestPatchGreenhouse {
        id: i64,
        name: String,
        token: String,
        maximum_average_humidity: Option<f64>,
        minimum_average_temperature: Option<f64>,
    },
    RequestDeleteGreenhouse {
        id: i64,
        current_password: String,
    },
    RequestPatchDevice {
        id: i64,
        greenhouse_id: i64,
        name: Option<String>,
        maximum_data_value: Option<f64>,
    },
    RequestPatchDevicesResetNames { greenhouse_id: i64 },
    RequestPatchDeviceState {
        id: i64,
        greenhouse_id: i64,
        state: u8,
    },
    RequestPostDeviceCustomData {
        id: i64,
        greenhouse_id: i64,
        data: f64,
        time: u64,
    },
    RequestPostDeviceRequestData {
        id: Option<i64>,
        greenhouse_id: i64,
    },
    RequestPostDeviceDisable {
        id: i64,
        greenhouse_id: i64,
    },
    RequestPostDeviceEnable {
        id: i64,
        greenhouse_id: i64,
    },

    // Requests (Opcode: Authorize)
    Authorize { token: String },

    // Requests (Opcode: Subscribe)
    SubscribeToUserUpdate { id: i64 },
    SubscribeToUserMeUpdates {},
    SubscribeToGreenhouseUpdate { id: i64 },
    SubscribeToDeviceUpdate {
        id: i64,
        greenhouse_id: i64,
    },
    SubscribeToDevicesUpdate { greenhouse_id: i64 },
    SubscribeToDeviceRecordsUpdate {
        device_id: i64,
        greenhouse_id: i64,
    },
    SubscribeToDeviceRecordsAverageUpdate {
        device_id: i64,
        greenhouse_id: i64,
        range: DeviceRecordsTimestampRange,
    },

    // Dispatches
    DispatchUserUpdate {
        id: i64,
        username: String,
        created_at: u64,
        greenhouses: i64,
    },
    DispatchUserMeUpdate {
        id: i64,
        email: String,
        username: String,
        created_at: u64,
        locale: String,
        theme: UserTheme,
        greenhouses: i64,
    },
    DispatchGreenhouseMineUpdate {
        id: i64,
        name: String,
        token: String,
        owner_id: i64,
        created_at: u64,
        maximum_average_humidity: Option<f64>,
        minimum_average_temperature: Option<f64>,
    },
    DispatchGreenhouseMineDelete { id: i64 },
    DispatchDeviceUpdate {
        id: i64,
        external_id: Option<i16>,
        name: Option<String>,
        status: DeviceStatus,
        kind: DeviceKind,
        greenhouse_id: i64,
        created_at: u64,
        maximum_data_value: Option<f64>,
        latest_data: Option<f64>,
    },
    DispatchDeviceRecordsUpdate {
        device_id: i64,
        quantity: i64,
    },
    DispatchDeviceRecordsAverageUpdate {
        device_id: i64,
        range: DeviceRecordsTimestampRange,
        records: Vec<DeviceRecordsAverage>,
    },

    // Other
    Response {
        code: u32,
        message: String,
    },
    #[default]
    None,
}

impl WebSocketMessageData {
    pub fn is_none(&self) -> bool {
        matches!(self, WebSocketMessageData::None)
    }
}

impl From<UserPublic> for WebSocketMessageData {
    fn from(user: UserPublic) -> Self {
        WebSocketMessageData::DispatchUserUpdate {
            id: user.id,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            greenhouses: user.greenhouses,
        }
    }
}

impl From<UserMe> for WebSocketMessageData {
    fn from(user: UserMe) -> Self {
        WebSocketMessageData::DispatchUserMeUpdate {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            locale: to_variant_name(&user.locale).unwrap().to_string(),
            theme: user.theme,
            greenhouses: user.greenhouses,
        }
    }
}

impl From<Greenhouse> for WebSocketMessageData {
    fn from(greenhouse: Greenhouse) -> Self {
        WebSocketMessageData::DispatchGreenhouseMineUpdate {
            id: greenhouse.id,
            name: greenhouse.name,
            token: greenhouse.token,
            owner_id: greenhouse.owner_id,
            created_at: greenhouse.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            maximum_average_humidity: greenhouse.maximum_average_humidity,
            minimum_average_temperature: greenhouse.minimum_average_temperature,
        }
    }
}

impl From<Device> for WebSocketMessageData {
    fn from(device: Device) -> Self {
        let latest_data = match DeviceRecord::find_latest_by_device_id(device.id) {
            Ok(record) => Some(record.data),
            Err(_) => None,
        };

        WebSocketMessageData::DispatchDeviceUpdate {
            id: device.id,
            external_id: device.external_id,
            name: device.name,
            status: device.status,
            kind: device.kind,
            greenhouse_id: device.greenhouse_id,
            created_at: device.created_at.duration_since(UNIX_EPOCH).unwrap().as_secs(),
            maximum_data_value: device.maximum_data_value,
            latest_data,
        }
    }
}
