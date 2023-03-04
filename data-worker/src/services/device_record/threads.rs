use std::str::from_utf8;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use futures::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::amqp_client::{AmqpClient, AmqpPayload};
use crate::error::WorkerError;
use crate::garthen;
use crate::services::device::{Device, DeviceKind, DeviceStatus};
use crate::services::device_record::{DeviceRecord, NewDeviceRecord};
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

fn request(device: Device, token: String, devices: Option<Vec<Device>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let client = reqwest::Client::new();
        let runtime = Runtime::new().unwrap();

        if device.status == DeviceStatus::Disabled { return; }

        runtime.block_on(async move {
            match device.kind {
                // DeviceKind::HumiditySensor gets data from the same path
                DeviceKind::TemperatureSensor => {
                    let data: TemperatureAndHumidityData = client
                        .get(format!(
                            "{}/temp_hum/{}",
                            garthen::get_external_devices_api_url(),
                            device.external_id.unwrap_or(1),
                        ))
                        .header("x-auth-token", token)
                        .send().await.unwrap().json().await.unwrap();

                    DeviceRecord::create(NewDeviceRecord {
                        device_id: device.id,
                        data: data.temperature,
                    }).ok();

                    match devices {
                        Some(devices) => {
                            if let Some(device) = devices.iter().find(|&found|
                                found.kind == DeviceKind::HumiditySensor
                                    && found.external_id == device.external_id
                            ) {
                                DeviceRecord::create(NewDeviceRecord {
                                    device_id: device.id,
                                    data: data.humidity,
                                }).ok();
                            }
                        },
                        None => {
                            let device
                                = Device::find_humidity_sensor_by_external_id_and_greenhouse_id(
                                device.external_id,
                                device.greenhouse_id,
                            );

                            if let Ok(device) = device {
                                DeviceRecord::create(NewDeviceRecord {
                                    device_id: device.id,
                                    data: data.humidity,
                                }).ok();
                            }
                        },
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
    })
}

pub fn start_data_requesting_with_interval() -> JoinHandle<Result<(), WorkerError>> {
    info!("Starting data requesting thread");

    thread::spawn(|| -> Result<(), WorkerError> {
        loop {
            thread::sleep(Duration::from_secs(60));

            let greenhouses = Greenhouse::find_all()?;

            for greenhouse in greenhouses {
                let devices = Device::find_all_by_greenhouse_id(greenhouse.id)?;
                let mut threads = vec![];

                for device in devices.clone() {
                    let devices = devices.clone();
                    let token = greenhouse.token.to_owned();

                    threads.push(request(device, token, Some(devices)));
                }

                for thread in threads {
                    thread.join().unwrap();
                }
            }
        }
    })
}

pub fn start_data_request_consumer(amqp_client: &AmqpClient) -> JoinHandle<Result<(), WorkerError>> {
    let consumer_name = "data-requester";

    info!("Starting AMQP {consumer_name} consumer thread");

    let runtime = Runtime::new().unwrap();
    let channel = runtime.block_on(async move {
        amqp_client.connection.create_channel().await
            .unwrap_or_else(|_| panic!("Failed to create AMQP channel in {consumer_name} consumer"))
    });

    thread::spawn(move || -> Result<(), WorkerError> {
        runtime.block_on(async move {
            let mut consumer = channel.basic_consume(
                "request-data",
                format!("data-worker-{consumer_name}").as_str(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await.unwrap_or_else(|_| panic!("Failed to create AMQP {consumer_name} consumer"));

            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.unwrap();

                delivery.ack(BasicAckOptions::default())
                    .await.unwrap_or_else(|_| panic!("Failed to ACK in {consumer_name} consumer"));

                if let Ok(AmqpPayload::RequestData {
                              device_id,
                              greenhouse_id,
                          }) = serde_json::from_str::<AmqpPayload>(
                    from_utf8(delivery.data.as_slice()).unwrap_or("")
                ) {
                    if let Some(device_id) = device_id {
                        if let Ok(device) = Device::find(device_id) {
                            let device = match device.kind {
                                DeviceKind::HumiditySensor => {
                                    Device::find_temperature_sensor_by_external_id_and_greenhouse_id(
                                        device.external_id,
                                        device.greenhouse_id,
                                    ).unwrap_or(device)
                                },
                                _ => device,
                            };
                            let greenhouse
                                = Greenhouse::find(device.greenhouse_id);

                            if let Ok(greenhouse) = greenhouse {
                                request(device, greenhouse.token, None).join().ok();
                            }
                        }
                    } else if let Some(greenhouse_id) = greenhouse_id {
                        let greenhouse
                            = Greenhouse::find(greenhouse_id);
                        let devices
                            = Device::find_all_by_greenhouse_id(greenhouse_id);
                        let mut threads = vec![];

                        if let (Ok(greenhouse), Ok(devices))
                            = (greenhouse, devices) {
                            for device in devices.clone() {
                                let devices = devices.clone();
                                let token = greenhouse.token.to_owned();

                                threads.push(request(device, token, Some(devices)));
                            }

                            for thread in threads {
                                thread.join().ok();
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    })
}
