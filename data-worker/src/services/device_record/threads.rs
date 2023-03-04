use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::error::WorkerError;
use crate::garthen;
use crate::services::device_record::{DeviceRecord, NewDeviceRecord};
use crate::services::device::{Device, DeviceKind};
use crate::services::greenhouse::Greenhouse;

#[derive(Debug, Deserialize)]
struct TemperatureAndHumidityData {
    temperature: f64,
    humidity: f64,
}

#[derive(Debug, Deserialize)]
struct SoilMoistureData {
    humidity: f64,
}

pub fn start_data_requesting_with_interval() -> JoinHandle<Result<(), WorkerError>> {
    thread::spawn(|| -> Result<(), WorkerError> {
        info!("Starting data requesting thread");

        loop {
            thread::sleep(Duration::from_secs(60));

            let greenhouses = Greenhouse::find_all()?;

            for greenhouse in greenhouses {
                let devices = Device::find_all_by_greenhouse_id(greenhouse.id)?;
                let mut threads = vec![];

                for device in devices.clone() {
                    let devices = devices.clone();
                    let token = greenhouse.token.to_owned();

                    threads.push(thread::spawn(move || {
                        let client = reqwest::Client::new();
                        let runtime = Runtime::new().unwrap();

                        runtime.block_on(async move {
                            match device.kind {
                                // DeviceKind::HumiditySensor gets data from the same path
                                DeviceKind::TemperatureSensor => {
                                    let data: TemperatureAndHumidityData = client
                                        .get(format!(
                                            "{}/temp_hum/{}",
                                            garthen::get_external_devices_api_url(),
                                            device.external_id.unwrap(),
                                        ))
                                        .header("x-auth-token", token)
                                        .send().await.unwrap().json().await.unwrap();

                                    DeviceRecord::create(NewDeviceRecord {
                                        device_id: device.id,
                                        data: data.temperature,
                                    }).unwrap();

                                    if let Some(device) = devices.iter().find(|&found|
                                        found.kind == DeviceKind::HumiditySensor
                                            && found.external_id == device.external_id
                                    ) {
                                        DeviceRecord::create(NewDeviceRecord {
                                            device_id: device.id,
                                            data: data.humidity,
                                        }).unwrap();
                                    }
                                },
                                DeviceKind::SoilMoistureSensor => {
                                    let data: SoilMoistureData = client
                                        .get(format!(
                                            "{}/hum/{}",
                                            garthen::get_external_devices_api_url(),
                                            device.external_id.unwrap(),
                                        ))
                                        .header("x-auth-token", token)
                                        .send().await.unwrap().json().await.unwrap();

                                    DeviceRecord::create(NewDeviceRecord {
                                        device_id: device.id,
                                        data: data.humidity,
                                    }).unwrap();
                                },
                                _ => {},
                            };
                        })
                    }));
                }

                for thread in threads {
                    thread.join().unwrap();
                }
            }
        }
    })
}
