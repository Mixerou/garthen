use amqp::{BasicProperties, Channel, ExchangeKind, Queue};
use amqp::options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions};
use amqp::types::FieldTable;
use futures::executor::block_on;
use lapin::Connection;
use lapin::options::BasicPublishOptions;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AmqpPayload {
    DispatchData {
        device_id: i64,
    },
    RequestData {
        device_id: Option<i64>,
        greenhouse_id: Option<i64>,
    },
    ChangeControllerState {
        device_id: i64,
        state: u8,
    },
    DispatchDevice {
        id: i64,
    },
    Ping,
}

pub struct AmqpPublisherMessage<'a> {
    pub exchange: Option<&'a str>,
    pub routing_key: Option<&'a str>,
    pub payload: AmqpPayload,
}

lazy_static! {
    static ref CONNECTION: &'static Connection = {
        amqp::get_connection()
    };

    static ref CHANNEL: Channel = {
        block_on(async move {
            get_connection().create_channel().await.expect("Failed to create AMQP channel")
        })
    };
}

pub fn get_connection<'a>() -> &'a Connection
{
    <&Connection>::clone(&CONNECTION)
}

pub fn get_channel() -> Channel
{
    CHANNEL.clone()
}

pub fn init() {
    info!("Initialize AMQP Client");

    lazy_static::initialize(&CONNECTION);
    lazy_static::initialize(&CHANNEL);

    let runtime = Runtime::new().unwrap();
    let channel = get_channel();

    runtime.block_on(async move {
        // Exchanges
        declare_exchange(&channel, "data", ExchangeKind::Topic).await;
        declare_exchange(&channel, "device", ExchangeKind::Topic).await;

        // Queues
        declare_queue(&channel, "request-data").await;
        declare_queue(&channel, "dispatch-data").await;
        declare_queue(&channel, "change-controller-state").await;
        declare_queue(&channel, "dispatch-device").await;

        // Queue bindings
        bind_queue(
            &channel,
            "request-data",
            "data",
            "data.request",
        ).await;
        bind_queue(
            &channel,
            "dispatch-data",
            "data",
            "data.created",
        ).await;
        bind_queue(
            &channel,
            "change-controller-state",
            "device",
            "device.controller.state.change",
        ).await;
        bind_queue(
            &channel,
            "dispatch-device",
            "device",
            "device.controller.state.changed",
        ).await;
    })
}

async fn declare_exchange(channel: &Channel, name: &str, kind: ExchangeKind) {
    channel.exchange_declare(
        name,
        kind,
        ExchangeDeclareOptions {
            durable: true,
            ..Default::default()
        },
        FieldTable::default(),
    ).await.unwrap_or_else(|_| panic!("Failed to declare {name} exchange"))
}

async fn declare_queue(channel: &Channel, name: &str) -> Queue {
    channel.queue_declare(
        name,
        QueueDeclareOptions {
            durable: true,
            ..Default::default()
        },
        FieldTable::default(),
    ).await.unwrap_or_else(|_| panic!("Failed to declare {name} queue"))
}

async fn bind_queue(channel: &Channel, name: &str, exchange: &str, routing_key: &str) {
    channel.queue_bind(
        name,
        exchange,
        routing_key,
        QueueBindOptions::default(),
        FieldTable::default(),
    ).await.unwrap_or_else(|_| panic!(
        "Failed to bind {name} queue to {exchange} exchange with {routing_key} routing key",
    ))
}

pub async fn publish(message: AmqpPublisherMessage<'_>) {
    let channel = get_channel();
    let exchange = message.exchange.unwrap_or("").to_string();
    let routing_key = message.routing_key.unwrap_or("").to_string();

    if let Ok(payload) = serde_json::to_string(&message.payload) {
        channel.basic_publish(
            exchange.as_str(),
            routing_key.as_str(),
            BasicPublishOptions::default(),
            payload.as_bytes(),
            BasicProperties::default(),
        ).await.unwrap().await.unwrap();
    }
}
