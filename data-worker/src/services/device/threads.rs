use std::str::from_utf8;
use std::thread;
use std::thread::JoinHandle;

use futures::executor::block_on;
use futures::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions};
use lapin::types::FieldTable;
use serde::Deserialize;
use tokio::runtime::Runtime;

use crate::{amqp_client, garthen};
use crate::amqp_client::{AmqpPayload, AmqpPublisherMessage};
use crate::error::WorkerError;
use crate::services::device::{Device, DeviceKind};
use crate::services::device_record::{DeviceRecord, NewDeviceRecord};
use crate::services::greenhouse::Greenhouse;

#[derive(Debug, Deserialize)]
struct ExternalApiResponse {
    code: u16,
}

pub fn start_change_controller_state_consumer() -> JoinHandle<Result<(), WorkerError>> {
    let consumer_name = "controller-state-changer";

    info!("Starting AMQP {consumer_name} consumer thread");

    let runtime = Runtime::new().unwrap();
    let channel = block_on(async move {
        amqp_client::get_connection().create_channel()
            .await.expect("Failed to create AMQP channel in {consumer_name} consumer")
    });

    thread::spawn(move || -> Result<(), WorkerError> {
        runtime.block_on(async move {
            let mut consumer = channel.basic_consume(
                "change-controller-state",
                format!("data-worker-{consumer_name}").as_str(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            ).await.unwrap_or_else(|_| panic!("Failed to create AMQP {consumer_name} consumer"));

            while let Some(delivery) = consumer.next().await {
                let delivery = delivery.unwrap();

                delivery.ack(BasicAckOptions::default())
                    .await.unwrap_or_else(|_| panic!("Failed to ACK in {consumer_name} consumer"));

                if let Ok(AmqpPayload::ChangeControllerState {
                              device_id,
                              state,
                          }) = serde_json::from_str::<AmqpPayload>(
                    from_utf8(delivery.data.as_slice()).unwrap_or("")
                ) {
                    let device = match Device::find(device_id) {
                        Ok(device) => device,
                        Err(_) => continue,
                    };
                    let greenhouse = match Greenhouse::find(device.greenhouse_id) {
                        Ok(greenhouse) => greenhouse,
                        Err(_) => continue,
                    };
                    let client = reqwest::Client::new();

                    let response: ExternalApiResponse = match device.kind {
                        DeviceKind::HumidificationController => {
                            client.patch(format!(
                                "{}/total_hum?state={}",
                                garthen::get_external_devices_api_url(),
                                state,
                            ))
                                .header("x-auth-token", greenhouse.token)
                                .send().await.unwrap().json().await.unwrap()
                        },
                        DeviceKind::IrrigationControllers => {
                            client.patch(format!(
                                "{}/watering?id={}&state={}",
                                garthen::get_external_devices_api_url(),
                                device.external_id.unwrap_or(1),
                                state,
                            ))
                                .header("x-auth-token", greenhouse.token)
                                .send().await.unwrap().json().await.unwrap()
                        },
                        DeviceKind::WindowsController => {
                            client.patch(format!(
                                "{}/fork_drive?state={}",
                                garthen::get_external_devices_api_url(),
                                state,
                            ))
                                .header("x-auth-token", greenhouse.token)
                                .send().await.unwrap().json().await.unwrap()
                        },
                        _ => continue,
                    };

                    if response.code != 200 { continue; }

                    if DeviceRecord::create(NewDeviceRecord {
                        device_id,
                        data: state as f64,
                    }).is_ok() {
                        amqp_client::publish(AmqpPublisherMessage {
                            exchange: Some("device"),
                            routing_key: Some("device.controller.state.changed"),
                            payload: AmqpPayload::DispatchDevice { id: device_id },
                        }).await;
                    }
                }
            }
        });

        Ok(())
    })
}
